use crate::drivers::esp::driver::Driver as EspDriver;
use crate::drivers::leds::Leds;
use crate::drivers::mic::Mic;
use esp_idf_svc::sys::EspError;

pub type DriverError = EspError;

pub trait Driver {
    fn get_leds(&mut self) -> &mut dyn Leds;

    fn get_mic(&mut self) -> &mut dyn Mic;
}

pub fn create_driver() -> Result<Box<dyn Driver>, DriverError> {
    let driver = EspDriver::new()?;
    Ok(Box::new(driver))
}
