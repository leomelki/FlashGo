use crate::drivers::driver::DriverError;

use super::super::driver::Driver;
use super::super::leds::Leds;
use super::super::mic::Mic;
use super::leds::LedsESPImpl;
use super::mic::MicESPImpl;
use esp_idf_svc::hal::{gpio::Gpio35, prelude::Peripherals};
use std::cell::RefCell;

pub struct DriverESPImpl {
    leds: RefCell<Option<LedsESPImpl>>,
    mic: RefCell<Option<MicESPImpl<Gpio35>>>,
}

impl DriverESPImpl {
    pub fn new() -> Result<Self, DriverError> {
        // It is necessary to call this function once. Otherwise some patches to the runtime
        // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
        esp_idf_svc::sys::link_patches();

        // Bind the log crate to the ESP Logging facilities
        esp_idf_svc::log::EspLogger::initialize_default();

        let peripherals = Peripherals::take()?;

        let leds = LedsESPImpl::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?;
        let mic = MicESPImpl::new(peripherals.pins.gpio35, peripherals.adc1)?;

        Ok(DriverESPImpl {
            leds: RefCell::new(Some(leds)),
            mic: RefCell::new(Some(mic)),
        })
    }
}

impl Driver for DriverESPImpl {
    fn take_leds(&mut self) -> Box<dyn Leds> {
        let leds = self.leds.borrow_mut().take().expect("LEDs already taken");
        Box::new(leds)
    }

    fn take_mic(&mut self) -> Box<dyn Mic> {
        let mic = self
            .mic
            .borrow_mut()
            .take()
            .expect("Microphone already taken");
        Box::new(mic)
    }
}
