use crate::leds::color::Color;
use crate::leds::led::Led;

const LED_COUNT: usize = 8 * 8;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::sys::EspError;

use super::driver::Ws2812Esp32RmtDriver;

pub struct LedsController<'a> {
    encoder_driver: Ws2812Esp32RmtDriver<'a>,
    leds: [Led<'a>; LED_COUNT],
}

impl<'a> LedsController<'a> {
    pub(crate) fn new() -> Result<Self, EspError> {
        let peripherals = Peripherals::take()?;
        let led = peripherals.pins.gpio23;
        let channel = peripherals.rmt.channel0;

        Ok(LedsController {
            leds: [Led::new(); LED_COUNT],
            encoder_driver: Ws2812Esp32RmtDriver::new(channel, led)?,
        })
    }

    pub fn update(&mut self) -> Result<(), EspError> {
        self.encoder_driver
            .write_blocking(self.leds.iter().flat_map(|led: &Led| {
                let color = led.get_color();
                [color.green, color.red, color.blue]
            }))
    }

    pub fn set_color(&mut self, index: usize, color: &'a Color) {
        self.leds[index].set_color(color);
    }

    pub fn get_led(&self, index: usize) -> Led {
        self.leds[index]
    }

    pub fn get_leds(&self) -> &[Led; LED_COUNT] {
        &self.leds
    }
}
