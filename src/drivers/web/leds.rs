use wasm_bindgen::prelude::wasm_bindgen;

use crate::drivers::leds::{Color, Leds, LED_COUNT};
use anyhow::Result;

#[wasm_bindgen]
extern "C" {
    fn update_leds_js(r: &[u8], g: &[u8], b: &[u8]);
}

pub struct LedsSimImpl {}

impl LedsSimImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Leds for LedsSimImpl {
    fn update(&mut self, leds: [Color; LED_COUNT]) -> Result<()> {
        let mut r = [0u8; LED_COUNT];
        let mut g = [0u8; LED_COUNT];
        let mut b = [0u8; LED_COUNT];

        for i in 0..LED_COUNT {
            r[i] = leds[i].red;
            g[i] = leds[i].green;
            b[i] = leds[i].blue;
        }

        update_leds_js(&r, &g, &b);
        Ok(())
    }
}
