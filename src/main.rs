#![allow(clippy::needless_lifetimes)]
mod consts;
mod core;
mod drivers;
mod leds;
mod mic;
mod server;

use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::thread::Builder;

use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_svc::hal::adc::oneshot::AdcChannelDriver;
use esp_idf_svc::hal::adc::oneshot::AdcDriver;
use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::gpio::Gpio33;
use esp_idf_svc::hal::peripheral::Peripheral;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::sys::EspError;
use leds::leds_controller::LedsController;

fn main() -> Result<(), EspError> {
    // let mut led_controller =
    //     LedsController::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?;

    // led_controller.set_color(3, 1, leds::color::Color::RED);
    // led_controller.set_color(3, 2, leds::color::Color::RED);
    // led_controller.set_color(3, 3, leds::color::Color::RED);
    // led_controller.set_color(3, 4, leds::color::Color::RED);
    // led_controller.set_color(3, 5, leds::color::Color::RED);
    // led_controller.set_color(2, 5, leds::color::Color::RED);
    // led_controller.set_color(1, 5, leds::color::Color::RED);
    // led_controller.set_color(0, 0, leds::color::Color::RED);
    // led_controller.update()?;

    // log::info!("1!");

    // unsafe {
    //     let remaining_ram = esp_idf_svc::sys::esp_get_free_heap_size();
    //     log::info!("Remaining RAM 1: {}", remaining_ram);
    // }
    // let mut mic_reader = mic::micreader::MicReader::new(peripherals.pins.gpio35, peripherals.adc1)?;

    // unsafe {
    //     let remaining_ram = esp_idf_svc::sys::esp_get_free_heap_size();
    //     log::info!("Remaining RAM 2: {}", remaining_ram);
    // }

    // loop {
    //     mic_reader.read_buffer_process()?;
    // }

    crate::drivers::driver::create_driver()?;

    Ok(())
    // let mut mic = mic::mic::Mic::new()?;
    // mic.start_task(peripherals.pins.gpio33, peripherals.adc1)?;
    // log::info!("done");
}
