use crate::leds::color::Color;
use crate::leds::led::Led;

use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::rmt::RmtChannel;
use esp_idf_svc::sys::EspError;

use super::driver::Ws2812Esp32RmtDriver;

const LED_COUNT: usize = 8 * 8;

pub struct LedsController<'a> {
    encoder_driver: Ws2812Esp32RmtDriver<'a>,
    leds: [Led<'a>; LED_COUNT],
}

impl<'a> LedsController<'a> {
    pub(crate) fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'a,
        pin: impl Peripheral<P = impl OutputPin> + 'a,
    ) -> Result<Self, EspError> {
        Ok(LedsController {
            leds: [Led::new(); LED_COUNT],
            encoder_driver: Ws2812Esp32RmtDriver::new(channel, pin)?,
        })
    }

    pub fn update(&mut self) -> Result<(), EspError> {
        self.encoder_driver
            .write_blocking(self.leds.iter().flat_map(|led: &Led| {
                let color = led.get_color();
                [color.green, color.red, color.blue]
            }))
    }

    pub fn get_led(&self, x: usize, y: usize) -> Led {
        self.leds[y << 3 | x >> 3]
    }

    pub fn get_leds(&self) -> &[Led; LED_COUNT] {
        &self.leds
    }
}
