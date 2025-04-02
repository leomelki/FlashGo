use super::leds::Leds;
use super::mic::Mic;

#[cfg(not(target_arch = "wasm32"))]
pub type DriverError = esp_idf_svc::sys::EspError;

#[cfg(target_arch = "wasm32")]
pub type DriverError = ();

pub trait Driver {
    fn take_leds(&mut self) -> Box<dyn Leds>;

    fn take_mic(&mut self) -> Box<dyn Mic>;
}

pub fn delay_ms(ms: u32) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        esp_idf_svc::hal::delay::Delay::new(100).delay_ms(ms);
    }
}
pub fn log(message: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        log::info!("{}", message);
    }
}

pub fn create_driver() -> Result<Box<dyn Driver>, DriverError> {
    #[cfg(not(target_arch = "wasm32"))]
    let driver = super::esp::driver::DriverESPImpl::new()?;
    #[cfg(target_arch = "wasm32")]
    let driver = super::web::driver::DriverWebImpl::new()?;

    Ok(Box::new(driver))
}
