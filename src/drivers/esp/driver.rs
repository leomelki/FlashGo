use crate::drivers::driver::DriverError;

use super::super::driver::Driver as DriverTrait;
use super::super::leds::Leds as LedsTrait;
use super::super::mic::Mic as MicTrait;
use super::leds::Leds;
use super::mic::Mic;
use esp_idf_svc::hal::{gpio::Gpio35, prelude::Peripherals};

pub struct Driver {
    leds: Leds,
    mic: Mic<Gpio35>,
}

impl Driver {
    pub fn new() -> Result<Self, DriverError> {
        // It is necessary to call this function once. Otherwise some patches to the runtime
        // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
        esp_idf_svc::sys::link_patches();

        // Bind the log crate to the ESP Logging facilities
        esp_idf_svc::log::EspLogger::initialize_default();

        let peripherals = Peripherals::take()?;

        let driver = Driver {
            leds: Leds::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?,
            mic: Mic::new(peripherals.pins.gpio35, peripherals.adc1)?,
        };
        Ok(driver)
    }
}

impl DriverTrait for Driver {
    fn get_leds(&mut self) -> Box<&mut dyn LedsTrait> {
        Box::new(&mut self.leds)
    }

    fn get_mic(&mut self) -> Box<&mut dyn MicTrait> {
        Box::new(&mut self.mic)
    }
}
