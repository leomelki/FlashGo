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
use drivers::{
    ble::{Server, Service},
    driver,
};
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
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    init().await.unwrap();
}

async fn init() -> Result<()> {
    let mut ble_server = driver::create_ble_server();
    let (leds, mic, sync) = crate::drivers::driver::create_drivers().await.unwrap();
    log::info!("Starting ESP32");

    let animation_thread = AnimationThread::init(leds).await;

    let animation_orchestrator = Box::leak(Box::new(AnimationsOrchestrator::new(
        ble_server.register_service("animation")?,
        animation_thread,
        DevicesSyncer::new(sync),
    )?));

    ble_server
        .register_service("identity-flashgo-v1")?
        .register_characteristic("version", true, false)?;

    let mut mic_reader = mic::mic_reader::MicReader::new(mic);

    animation_orchestrator.init().await?;

    ble_server.start_advertisement();
    println!("Advertising started");

    loop {
        // mic_reader.read_buffer_process().await?;
        driver::delay_ms(100).await;
    }
}
