#![no_std]
#![no_main]

mod consts;
mod core;
mod leds;
mod mic;
mod server;

use ::core::ffi::{c_void, CStr, FromBytesWithNulError};
use core::Core;
use esp_idf_svc::hal::cpu;
use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::task;

use esp_idf_svc::sys::TaskHandle_t;
use esp_idf_svc::{hal::prelude::Peripherals, sys::EspError};
use leds::leds_controller::LedsController;
use mic::Mic;

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
    let mut ledController = LedsController::new(ledChannel, ledPin)?;

    let micPin = peripherals.pins.gpio36;
    let i2s = peripherals.i2s0;
    let micChannel = peripherals.adc1;

    let mut mic = mic::Mic::new(micChannel, i2s, micPin);
    let mut core = Core::new(&mut ledController, &mic)?;
    // core.start();

    ledController.set_color(3, 1, leds::color::Color::RED);
    ledController.set_color(3, 2, leds::color::Color::RED);
    ledController.set_color(3, 3, leds::color::Color::RED);
    ledController.set_color(3, 4, leds::color::Color::RED);
    ledController.set_color(3, 5, leds::color::Color::RED);
    ledController.set_color(2, 5, leds::color::Color::RED);
    ledController.set_color(1, 5, leds::color::Color::RED);
    ledController.set_color(0, 0, leds::color::Color::RED);
    loop {
        Delay::new(1).delay_ms(500);
    }
    ledController.update()
}
