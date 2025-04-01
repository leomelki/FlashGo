use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::rmt::RmtChannel;
use esp_idf_svc::sys::EspError;

use crate::drivers::driver::DriverError;

use super::super::leds::Leds as LedsTrait;
use super::super::leds::{Color, LED_COUNT};
use super::leds_driver::Ws2812Esp32RmtDriver;
pub struct Leds {
    encoder_driver: Ws2812Esp32RmtDriver<'static>,
}

impl Leds {
    pub(crate) fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'static,
        pin: impl Peripheral<P = impl OutputPin> + 'static,
    ) -> Result<Self, EspError> {
        Ok(Leds {
            encoder_driver: Ws2812Esp32RmtDriver::new(channel, pin)?,
        })
    }
}

impl LedsTrait for Leds {
    fn update(&mut self, colors: [Color; LED_COUNT]) -> Result<(), DriverError> {
        self.encoder_driver.write_blocking(
            colors
                .iter()
                .flat_map(|color: &Color| [color.green, color.red, color.blue]),
        )
    }
}
