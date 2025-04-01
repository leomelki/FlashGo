use super::driver::DriverError;

pub const LED_COUNT: usize = 8 * 8;
pub trait Leds {
    fn update(&mut self, colors: [Color; LED_COUNT]) -> Result<(), DriverError>;
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
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
