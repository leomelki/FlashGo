#![allow(clippy::needless_lifetimes)]
mod consts;
mod core;
mod leds;
mod mic;
mod server;

use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::thread::Builder;

use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::gpio::Gpio33;
use esp_idf_svc::hal::peripheral::Peripheral;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::sys::EspError;
use leds::leds_controller::LedsController;

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take()?;
    let mut led_controller =
        LedsController::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?;

    led_controller.set_color(3, 1, leds::color::Color::RED);
    led_controller.set_color(3, 2, leds::color::Color::RED);
    led_controller.set_color(3, 3, leds::color::Color::RED);
    led_controller.set_color(3, 4, leds::color::Color::RED);
    led_controller.set_color(3, 5, leds::color::Color::RED);
    led_controller.set_color(2, 5, leds::color::Color::RED);
    led_controller.set_color(1, 5, leds::color::Color::RED);
    led_controller.set_color(0, 0, leds::color::Color::RED);
    led_controller.update()?;

    log::info!("1!");

    let mut mic_reader = mic::micreader::MicReader::new(peripherals.pins.gpio33, peripherals.adc1)?;
    log::info!("2");
    let mut mic = mic::mic::Mic::new()?;
    log::info!("3");
    mic.start_task::<Gpio33>(&mut mic_reader)?;
    log::info!("done");
    loop {
        Delay::new(1).delay_ms(500);
    }
}
