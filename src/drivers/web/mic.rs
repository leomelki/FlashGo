use std::time::Instant;

use anyhow::Result;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::drivers::mic::{Mic, MIC_ANALYSIS_CONFIG};

pub struct MicSimImpl {}

#[wasm_bindgen]
extern "C" {
    fn read_buffer_js(size: usize) -> Vec<f32>;
}

impl MicSimImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Mic for MicSimImpl {
    fn read_buffer(&mut self) -> Result<[f32; MIC_ANALYSIS_CONFIG.buffer_size]> {
        // same code as in MicESPImpl to wait
        let sample_period = 1_000_000_000 / MIC_ANALYSIS_CONFIG.sample_rate;
        for _ in 0..MIC_ANALYSIS_CONFIG.buffer_size {
            let start_read = Instant::now();
            while start_read.elapsed().as_nanos() < sample_period as u128 {
                // wait
            }
        }

        let buffer = read_buffer_js(MIC_ANALYSIS_CONFIG.buffer_size);
        let mut result = [0.0; MIC_ANALYSIS_CONFIG.buffer_size];

        for (i, val) in buffer
            .iter()
            .enumerate()
            .take(MIC_ANALYSIS_CONFIG.buffer_size)
        {
            result[i] = *val;
        }

        Ok(result)
    }
}
