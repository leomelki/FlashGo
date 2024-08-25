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

    let mut peripherals = Peripherals::take()?;
    let ledPin = unsafe { peripherals.pins.gpio23.clone_unchecked() };
    let ledChannel = unsafe { peripherals.rmt.channel0.clone_unchecked() };
    let mut ledController = LedsController::new(ledChannel, ledPin)?;
    //    unsafe {
    //      MIC.replace(mic::Mic::new(peripherals.pins.gpio33, peripherals.adc1)?);
    // }
    log::info!("1!");

    // let micPin = peripherals.pins.gpio36;
    // let i2s = peripherals.i2s0;
    // let micChannel = peripherals.adc1;
    let mut mic_reader = mic::micreader::MicReader::new(peripherals.pins.gpio33, peripherals.adc1)?;
    log::info!("2");
    let mut mic = mic::mic::Mic::new()?;
    log::info!("3");
    mic.start_task::<Gpio33>(&mut mic_reader)?;
    log::info!("done");
    loop {
        Delay::new(1).delay_ms(500);
    }

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
