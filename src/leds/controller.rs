use anyhow::Result;

use crate::drivers::leds::{Color, Leds, LED_COUNT};

pub struct LedsController {
    colors: [Color; LED_COUNT],
    changed: bool,
    pub width: usize,
    pub height: usize,
}

impl LedsController {
    pub fn new() -> Result<Self> {
        Ok(LedsController {
            colors: [Color::black(); LED_COUNT],
            changed: false,
            width: 8,
            height: 8,
        })
    }

    pub fn update<L: Leds>(&mut self, leds: &mut L) -> Result<()> {
        if self.changed {
            leds.update(self.colors)
        } else {
            Ok(())
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.colors[y << 3 | x >> 3]
    }
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let final_y = 7 - y;
        let final_x = 7 - if y % 2 == 0 { x } else { 7 - x };
        if self.colors[final_x + final_y * 8] != color {
            self.colors[final_x + final_y * 8] = color;
            self.changed = true;
        }
    }
    pub fn set_color_by_index(&mut self, index: usize, color: Color) {
        if self.colors[index] != color {
            self.colors[index].set(&color);
            self.changed = true;
        }
    }

    pub fn set_all_colors(&mut self, color: Color) {
        let mut changed = false;
        for i in 0..LED_COUNT {
            if self.colors[i] != color {
                changed = true;
                self.colors[i] = color;
            }
        }
        self.changed = changed;
    }

    pub fn get_colors(&self) -> &[Color; LED_COUNT] {
        &self.colors
    }
}
