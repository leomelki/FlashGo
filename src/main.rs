#![allow(clippy::needless_lifetimes)]
mod animations;
mod consts;
mod drivers;
mod leds;
mod macros;
mod mic;
mod protos;
mod sync;
mod time;
mod utils;

use animations::orchestrator::AnimationsOrchestrator;
use anyhow::Result;
use drivers::{ble::Server, driver};
use leds::animations::thread::AnimationThread;
use sync::DevicesSyncer;

#[cfg(feature = "esp")]
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    spawner.spawn(init_esp()).unwrap();
}

#[cfg(feature = "esp")]
#[embassy_executor::task]
async fn init_esp() {
    init().await.unwrap();
}

async fn init() -> Result<()> {
    let (leds, mic, sync) = crate::drivers::driver::create_drivers().await.unwrap();
    log::info!("Starting ESP32");

    let mut ble_server = driver::create_ble_server();

    let animation_thread = AnimationThread::init(leds).await;

    let animation_orchestrator = Box::leak(Box::new(AnimationsOrchestrator::new(
        ble_server.register_service("animation")?,
        animation_thread,
        DevicesSyncer::new(sync),
    )?));

    let mut mic_reader = mic::mic_reader::MicReader::new(mic);

    animation_orchestrator.init().await?;

    ble_server.start_advertisement();

    loop {
        // mic_reader.read_buffer_process().await?;
        driver::delay_ms(100).await;
    }
}
