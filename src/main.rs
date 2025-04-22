#![allow(clippy::needless_lifetimes)]
mod consts;
mod drivers;
mod leds;
mod mic;
mod server;

use anyhow::Result;
use leds::thread::{messages::Message, thread::AnimationThread};

#[cfg(feature = "esp")]
fn main() -> Result<()> {
    embassy_futures::block_on(init())
}

async fn init() -> Result<()> {
    let (leds, mic) = crate::drivers::driver::create_drivers()?;
    log::info!("Starting ESP32");

    let mut animation_thread = AnimationThread::init(leds);
    animation_thread.send(Message::Init(1));

    let mut mic_reader = mic::mic_reader::MicReader::new(mic);
    loop {
        mic_reader.read_buffer_process().await?;
    }
    Ok(())
}
