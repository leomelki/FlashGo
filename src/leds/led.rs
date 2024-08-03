use crate::leds::color::Color;

#[derive(Clone, Copy)]
pub struct Led<'a> {
    color: &'a Color,
}

impl<'a> Led<'a> {
    pub(crate) fn new() -> Self {
        Led { color: &Color::RED }
    }

    pub fn set_color(&mut self, color: &'a Color) {
        self.color = color;
    }

    pub fn get_color(&self) -> &Color {
        self.color
    }
}
