use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use super::leds::Leds;
use super::mic::Mic;
use super::sync::SyncTrait;
use anyhow::Result;
use rand::RngCore;
use rand::SeedableRng;

#[cfg(feature = "wasm")]
pub type Instant = web_time::Instant;

#[cfg(not(feature = "wasm"))]
pub type Instant = std::time::Instant;

pub async fn create_drivers() -> Result<(impl Leds, impl Mic, impl SyncTrait)> {
    #[cfg(feature = "esp")]
    let drivers = super::esp::driver::new()?;
    #[cfg(feature = "wasm")]
    let drivers = super::web::driver::new()?;

    Ok(drivers)
}

pub async fn delay_ms(ms: u32) {
    #[cfg(feature = "wasm")]
    {
        gloo_timers::future::sleep(Duration::from_millis(ms as u64)).await;
    }
    #[cfg(feature = "esp")]
    {
        embassy_time::Timer::after_millis(ms as u64).await;
    }
}

pub fn log_data(key: &str, value: f32) {
    #[cfg(feature = "esp")]
    {
        log::info!("{}: {}", key, value);
    }
    #[cfg(feature = "wasm")]
    {
        super::web::driver::log_data_js(key, value);
    }
}

#[cfg(feature = "wasm")]
type BleServer = super::web::driver::BLEServerSimImpl;

#[cfg(not(feature = "wasm"))]
type BleServer = super::esp::driver::EspServer;

// Thread-safe singleton for BLE server
// static BLE_SERVER: OnceCell<Mutex<BleServer>> = OnceCell::new();

// pub fn get_ble_server() -> &'static Mutex<BleServer> {
//     BLE_SERVER.get_or_init(|| Mutex::new(create_ble_server()))
// }

pub fn create_ble_server() -> BleServer {
    #[cfg(feature = "esp")]
    let ble_server = super::esp::driver::create_ble_server();
    #[cfg(feature = "wasm")]
    let ble_server = super::web::driver::create_ble_server();

    ble_server
}

pub fn is_master() -> bool {
    #[cfg(feature = "esp")]
    return super::esp::driver::is_master();
    #[cfg(feature = "wasm")]
    return false;
}

pub fn random_u32() -> u32 {
    rand::random::<u32>()
}

pub fn random_u32_seeded(seed: u64) -> u32 {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    rng.next_u32()
}

pub async fn run_async(task: impl Future<Output = Result<()>> + 'static) {
    #[cfg(feature = "esp")]
    {
        #[embassy_executor::task(pool_size = 4)]
        async fn run_task(task: Pin<Box<dyn Future<Output = Result<()>>>>) {
            task.await.unwrap();
        }
        let spawner = embassy_executor::Spawner::for_current_executor().await;
        spawner.spawn(run_task(Box::pin(task))).unwrap();
    }

    #[cfg(not(feature = "esp"))]
    {
        wasm_bindgen_futures::spawn_local(async move {
            task.await;
        });
    }
}
