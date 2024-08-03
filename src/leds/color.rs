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
}
