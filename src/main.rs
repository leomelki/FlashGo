#![allow(clippy::needless_lifetimes)]
mod consts;
mod drivers;
mod leds;
mod mic;
mod server;

use drivers::{driver::DriverError, leds::Color};
use leds::leds_controller::LedsController;

fn main() -> Result<(), DriverError> {
    let driver = Box::leak(crate::drivers::driver::create_driver()?);

    let mut led_controller = LedsController::new(driver.take_leds())?;

    led_controller.set_color(3, 1, Color::red());
    led_controller.set_color(3, 2, Color::red());
    led_controller.set_color(3, 3, Color::red());
    led_controller.set_color(3, 4, Color::red());
    led_controller.set_color(3, 5, Color::red());
    led_controller.set_color(2, 5, Color::red());
    led_controller.set_color(1, 5, Color::red());
    led_controller.set_color(0, 0, Color::red());
    led_controller.update()?;

    let mic = driver.take_mic();
    let mut mic_reader = mic::micreader::MicReader::new(mic);
    loop {
        mic_reader.read_buffer_process()?;
    }
    Ok(())
}
