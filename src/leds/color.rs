#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    pub const RED: Color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    pub fn set_red(&mut self, red: u8) {
        self.red = red;
    }
    pub fn set_green(&mut self, green: u8) {
        self.green = green;
    }
    pub fn set_blue(&mut self, blue: u8) {
        self.blue = blue;
    }

    pub fn set(&mut self, color: &Color) {
        self.red = color.red;
        self.green = color.green;
        self.blue = color.blue;
    }
}
