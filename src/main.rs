#![allow(clippy::needless_lifetimes)]
mod consts;
mod drivers;
mod leds;
mod mic;
mod server;

use anyhow::Result;
use drivers::leds::Color;
use leds::leds_controller::LedsController;

#[cfg(feature = "esp")]
fn main() -> Result<()> {
    embassy_futures::block_on(init())
}

async fn init() -> Result<()> {
    let (leds, mic) = crate::drivers::driver::create_drivers()?;
    log::info!("Starting ESP32");

    let mut led_controller = LedsController::new(leds)?;

    led_controller.set_color(3, 1, Color::red());
    led_controller.set_color(3, 2, Color::red());
    led_controller.set_color(3, 3, Color::red());
    led_controller.set_color(3, 4, Color::red());
    led_controller.set_color(3, 5, Color::red());
    led_controller.set_color(2, 5, Color::red());
    led_controller.set_color(1, 5, Color::red());
    led_controller.set_color(0, 0, Color::red());
    led_controller.update()?;

    let mut mic_reader = mic::mic_reader::MicReader::new(mic);
    loop {
        mic_reader.read_buffer_process().await?;
    }
    Ok(())
}
