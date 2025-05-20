use crate::drivers::driver;
use crate::drivers::leds::Color as LedsColor;
use crate::protos::utils_::{Color, ColorType, Color_, StaticColor};
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};

impl Color {
    pub fn to_color(&self) -> LedsColor {
        self.to_color_with_time(0, 1000)
    }

    pub fn to_color_with_time(&self, time_ms: u64, random_change_delay_ms: u64) -> LedsColor {
        match self.r#type {
            ColorType::Static => {
                if let Some(static_color) = self.color() {
                    let rgb = static_color.rgb;
                    let red = ((rgb >> 16) & 0xFF) as u8;
                    let green = ((rgb >> 8) & 0xFF) as u8;
                    let blue = (rgb & 0xFF) as u8;
                    LedsColor::new(red, green, blue)
                } else {
                    LedsColor::black()
                }
            }
            ColorType::Rainbow => {
                // For Rainbow, calculate a color based on the current time
                // Cycle through the hue spectrum (0-360 degrees)
                let hue = (time_ms % 3600) as f32 / 10.0; // Complete cycle every 3.6 seconds
                LedsColor::from_hsv(hue, 1.0, 1.0)
            }
            ColorType::Random => {
                // For Random, generate pseudo-random color based on time
                // Change color every random_change_delay_ms
                let seed = time_ms / random_change_delay_ms;

                let mut rng = StdRng::seed_from_u64(seed);
                let r = rng.random_range(0..=255) as u8;
                let g = rng.random_range(0..=255) as u8;
                let b = rng.random_range(0..=255) as u8;
                LedsColor::new(r, g, b)
            }
            ColorType::SyncRandom => {
                // SyncRandom is similar to Random but uses a global time source
                // This ensures all devices using this color change at the same time
                let seed = time_ms / random_change_delay_ms;
                let nbr = driver::random_u32_seeded(seed);
                let r = ((nbr >> 16) & 0xFF) as u8;
                let g = ((nbr >> 8) & 0xFF) as u8;
                let b = (nbr & 0xFF) as u8;
                LedsColor::new(r, g, b)
            }
            _ => LedsColor::black(), // Default case for unknown color types
        }
    }
}

impl LedsColor {
    pub fn to_proto(&self) -> Color {
        Color {
            r#type: ColorType::Static,
            color: StaticColor {
                rgb: (self.red as u32) << 16 | (self.green as u32) << 8 | self.blue as u32,
            },
            _has: Color_::_Hazzer::default().init_color(),
        }
    }
}
