use std::future::Future;
use std::thread::Builder;
use std::time::Duration;

use super::leds::Leds;
use super::mic::Mic;
use super::sync::Sync;
use anyhow::Result;

#[cfg(feature = "wasm")]
pub type Instant = web_time::Instant;

#[cfg(not(feature = "wasm"))]
pub type Instant = std::time::Instant;

pub fn create_drivers() -> Result<(impl Leds, impl Mic, impl Sync)> {
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

pub fn run_async(task: impl Future<Output = Result<()>> + Send + 'static, thread_name: &str) {
    #[cfg(feature = "esp")]
    {
        Builder::new()
            .name(thread_name.into())
            // .stack_size(1000)
            .spawn(move || {
                esp_idf_svc::hal::task::block_on(task).unwrap();
            })
            .expect("failed to spawn thread");
    }

    #[cfg(not(feature = "esp"))]
    {
        wasm_bindgen_futures::spawn_local(async move {
            task.await;
        });
    }
}
