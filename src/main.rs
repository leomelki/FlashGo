#![allow(clippy::needless_lifetimes)]
mod consts;
mod drivers;
mod leds;
mod macros;
mod mic;

use anyhow::Result;
use drivers::{ble::Server, driver};
use leds::animations::{
    thread::{messages::Message, AnimationThread},
    AnimationType,
};

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
    let (leds, mic) = crate::drivers::driver::create_drivers()?;
    log::info!("Starting ESP32");

    let mut ble_server = driver::create_ble_server();

    let mut animation_thread = AnimationThread::init(leds);
    animation_thread.send(Message::Init(1));
    animation_thread.send(Message::SetAnimation(AnimationType::Rainbow));
    let mut mic_reader = mic::mic_reader::MicReader::new(mic);

    ble_server.start_advertisement();

    loop {
        mic_reader.read_buffer_process().await?;
        // driver::delay_ms(100).await;
    }
}
