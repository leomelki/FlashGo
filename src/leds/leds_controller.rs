use crate::leds::color::Color;

use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::rmt::RmtChannel;
use esp_idf_svc::sys::EspError;

use super::driver::Ws2812Esp32RmtDriver;

const LED_COUNT: usize = 8 * 8;

pub struct LedsController<'a> {
    encoder_driver: Ws2812Esp32RmtDriver<'a>,
    colors: [Color; LED_COUNT],
}

impl<'a> LedsController<'a> {
    pub(crate) fn new<C: RmtChannel>(
        channel: impl Peripheral<P = C> + 'a,
        pin: impl Peripheral<P = impl OutputPin> + 'a,
    ) -> Result<Self, EspError> {
        Ok(LedsController {
            colors: [Color::BLACK; LED_COUNT],
            encoder_driver: Ws2812Esp32RmtDriver::new(channel, pin)?,
        })
    }

    pub fn update(&mut self) -> Result<(), EspError> {
        self.encoder_driver.write_blocking(
            self.colors
                .iter()
                .flat_map(|color: &Color| [color.green(), color.red(), color.blue()]),
        )
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.colors[y << 3 | x >> 3]
    }
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let final_y = 7 - y;
        let final_x = 7 - if final_y % 2 == 0 { x } else { 7 - x };
        self.colors[final_x + final_y * 8].set(&color)
    }
    pub fn set_color_by_index(&mut self, index: usize, color: Color) {
        self.colors[index].set(&color)
    }

    pub fn get_colors(&self) -> &[Color; LED_COUNT] {
        &self.colors
    }
}
