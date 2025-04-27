use anyhow::Result;
use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::rmt::RmtChannel;

use super::super::leds::Leds;
use super::super::leds::{Color, LED_COUNT};
use super::leds_driver::Ws2812Esp32RmtDriver;

pub struct LedsESPImpl {
    encoder_driver: Ws2812Esp32RmtDriver<'static>,
}

impl LedsESPImpl {
    pub(crate) fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'static,
        pin: impl Peripheral<P = impl OutputPin> + 'static,
    ) -> Result<Self> {
        Ok(LedsESPImpl {
            encoder_driver: Ws2812Esp32RmtDriver::new(channel, pin)?,
        })
    }
}

impl Leds for LedsESPImpl {
    fn update(&mut self, colors: [Color; LED_COUNT]) -> Result<()> {
        self.encoder_driver.write_blocking(
            colors
                .iter()
                .flat_map(|color: &Color| [color.green, color.red, color.blue]),
        )
    }
}
