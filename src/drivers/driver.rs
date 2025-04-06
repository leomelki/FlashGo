use super::leds::Leds;
use super::mic::Mic;
use anyhow::Result;

pub fn delay_ms(ms: u32) {
    #[cfg(feature = "esp")]
    {
        esp_idf_svc::hal::delay::Delay::new(100).delay_ms(ms);
    }
}

pub fn create_drivers() -> Result<(impl Leds, impl Mic)> {
    #[cfg(feature = "esp")]
    let drivers = super::esp::driver::new()?;
    #[cfg(feature = "wasm")]
    let drivers = super::web::driver::new()?;

    Ok(drivers)
}
