use wasm_bindgen::prelude::*;

use crate::drivers::leds::{Color, Leds, LED_COUNT};

#[wasm_bindgen]
extern "C" {
    fn update_leds(leds: &[Color]);
}

pub struct LedsSimImpl {}

impl LedsSimImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Leds for LedsSimImpl {
    fn update(&mut self, leds: &[Color; LED_COUNT]) {
        update_leds(leds);
    }
}
