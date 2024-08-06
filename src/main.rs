#![no_std]
#![no_main]

mod consts;
mod core;
mod leds;
mod mic;
mod server;

use esp_idf_svc::hal::adc::Adc;
use esp_idf_svc::{hal::prelude::Peripherals, sys::EspError};

#[no_mangle]
fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!3");

    let peripherals = Peripherals::take()?;
    let ledPin = peripherals.pins.gpio23;
    let ledChannel = peripherals.rmt.channel0;
    let micPin = peripherals.pins.gpio0;
    let micChannel = peripherals.adc2;
    let mut ledController = leds::leds_controller::LedsController::new(ledChannel, ledPin)?;
    let mut mic = mic::Mic::new(micChannel, micPin);

    ledController.set_color(3, 1, leds::color::Color::RED);
    ledController.set_color(3, 2, leds::color::Color::RED);
    ledController.set_color(3, 3, leds::color::Color::RED);
    ledController.set_color(3, 4, leds::color::Color::RED);
    ledController.set_color(3, 5, leds::color::Color::RED);
    ledController.set_color(2, 5, leds::color::Color::RED);
    ledController.set_color(1, 5, leds::color::Color::RED);
    ledController.set_color(0, 0, leds::color::Color::RED);
    ledController.update()
}
