use anyhow::Result;

pub const LED_COUNT: usize = 8 * 8;

pub trait Leds: Send + Sync {
    fn update(&mut self, colors: [Color; LED_COUNT]) -> Result<()>;
}

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        let c = value * saturation;
        let x = c * (1.0 - (hue / 60.0 % 2.0 - 1.0).abs());
        let m = value - c;
        let (r, g, b) = if hue < 60.0 {
            (c, x, 0.0)
        } else if hue < 120.0 {
            (x, c, 0.0)
        } else if hue < 180.0 {
            (0.0, c, x)
        } else if hue < 240.0 {
            (0.0, x, c)
        } else {
            (x, 0.0, c)
        };
        let (r, g, b) = ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0);
        Self::new(r as u8, g as u8, b as u8)
    }

    pub fn set(&mut self, color: &Color) {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
    }

    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
        }
    }

    pub fn red() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub fn green() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub fn blue() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 255,
        }
    }
}
