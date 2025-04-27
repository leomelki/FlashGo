use super::leds::LedsESPImpl;
use super::mic::MicESPImpl;

use anyhow::Result;
use esp_idf_svc::hal::{gpio::Gpio35, prelude::Peripherals};

pub fn new() -> Result<(LedsESPImpl, MicESPImpl<Gpio35>)> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let leds = LedsESPImpl::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?;
    let mic = MicESPImpl::new(peripherals.pins.gpio35, peripherals.adc1)?;
    Ok((leds, mic))
}
