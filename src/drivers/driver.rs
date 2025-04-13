use std::time::Duration;

use super::leds::Leds;
use super::mic::Mic;
use anyhow::Result;

#[cfg(feature = "wasm")]
pub type Instant = web_time::Instant;

#[cfg(not(feature = "wasm"))]
pub type Instant = std::time::Instant;

pub fn create_drivers() -> Result<(impl Leds, impl Mic)> {
    #[cfg(feature = "esp")]
    let drivers = super::esp::driver::new()?;
    #[cfg(feature = "wasm")]
    let drivers = super::web::driver::new()?;

    Ok(drivers)
}

pub async fn delay_ms(ms: u32) {
    #[cfg(feature = "wasm")]
    {
        gloo_timers::future::sleep(Duration::from_millis(ms as u64)).await;
    }
    #[cfg(feature = "esp")]
    {
        embassy_time::Timer::after_millis(ms as u64).await;
    }
}
