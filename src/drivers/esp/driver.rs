use crate::drivers::driver::DriverError;

use super::super::driver::Driver as DriverTrait;
use super::super::leds::Leds as LedsTrait;
use super::super::mic::Mic as MicTrait;
use super::leds::Leds;
use super::mic::Mic;
use esp_idf_svc::hal::{gpio::Gpio35, prelude::Peripherals};
use std::cell::RefCell;

pub struct Driver {
    leds: RefCell<Option<Leds>>,
    mic: RefCell<Option<Mic<Gpio35>>>,
}

impl Driver {
    pub fn new() -> Result<Self, DriverError> {
        // It is necessary to call this function once. Otherwise some patches to the runtime
        // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
        esp_idf_svc::sys::link_patches();

        // Bind the log crate to the ESP Logging facilities
        esp_idf_svc::log::EspLogger::initialize_default();

        let peripherals = Peripherals::take()?;

        let leds = Leds::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?;
        let mic = Mic::new(peripherals.pins.gpio35, peripherals.adc1)?;

        Ok(Driver {
            leds: RefCell::new(Some(leds)),
            mic: RefCell::new(Some(mic)),
        })
    }
}

impl DriverTrait for Driver {
    fn take_leds(&mut self) -> Box<dyn LedsTrait> {
        let leds = self.leds.borrow_mut().take().expect("LEDs already taken");
        Box::new(leds)
    }

    fn take_mic(&mut self) -> Box<dyn MicTrait> {
        let mic = self
            .mic
            .borrow_mut()
            .take()
            .expect("Microphone already taken");
        Box::new(mic)
    }
}
