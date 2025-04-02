use crate::drivers::leds::{Color, Leds, LED_COUNT};

use esp_idf_svc::sys::EspError;

pub struct LedsController {
    leds: &'static mut dyn Leds,
    colors: [Color; LED_COUNT],
}

impl LedsController {
    pub(crate) fn new(leds: &'static mut dyn Leds) -> Result<Self, EspError> {
        Ok(LedsController {
            leds,
            colors: [Color::black(); LED_COUNT],
        })
    }

    pub fn update(&mut self) -> Result<(), EspError> {
        self.leds.update(self.colors)
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.colors[y << 3 | x >> 3]
    }
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let final_y = 7 - y;
        let final_x = 7 - if final_y % 2 == 0 { x } else { 7 - x };
        self.colors[final_x + final_y * 8] = color;
    }
    pub fn set_color_by_index(&mut self, index: usize, color: Color) {
        self.colors[index].set(&color)
    }

    pub fn get_colors(&self) -> &[Color; LED_COUNT] {
        &self.colors
    }
}
