use crate::drivers::esp::driver::Driver as EspDriver;
use crate::drivers::leds::Leds;
use crate::drivers::mic::Mic;
use esp_idf_svc::sys::EspError;

pub type DriverError = EspError;

pub trait Driver {
    fn take_leds(&mut self) -> Box<dyn Leds>;

    fn take_mic(&mut self) -> Box<dyn Mic>;
}

pub fn create_driver() -> Result<Box<dyn Driver>, DriverError> {
    let driver = EspDriver::new()?;
    Ok(Box::new(driver))
}
