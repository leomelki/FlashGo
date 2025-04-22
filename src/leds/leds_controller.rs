use anyhow::Result;

use crate::drivers::leds::{Color, Leds, LED_COUNT};

pub struct LedsController<L> {
    leds: L,
    colors: [Color; LED_COUNT],
}

impl<L: Leds> LedsController<L> {
    pub fn new(leds: L) -> Result<Self> {
        Ok(LedsController {
            leds,
            colors: [Color::black(); LED_COUNT],
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.leds.update(self.colors)
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.colors[y << 3 | x >> 3]
    }
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let final_y = 7 - y;
        let final_x = 7 - if y % 2 == 0 { x } else { 7 - x };
        self.colors[final_x + final_y * 8] = color;
    }
    pub fn set_color_by_index(&mut self, index: usize, color: Color) {
        self.colors[index].set(&color)
    }

    pub fn get_colors(&self) -> &[Color; LED_COUNT] {
        &self.colors
    }
}
