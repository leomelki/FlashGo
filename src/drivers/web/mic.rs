use crate::drivers::mic::{Mic, MIC_ANALYSIS_CONFIG};
use anyhow::Result;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::js_sys::Float32Array;

pub struct MicSimImpl {}

#[wasm_bindgen]
extern "C" {
    async fn read_buffer_js(size: usize, rate: usize) -> JsValue;
}

impl MicSimImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Mic for MicSimImpl {
    async fn read_buffer(
        &mut self,
        buffer: &mut [f32; MIC_ANALYSIS_CONFIG.buffer_size],
    ) -> Result<()> {
        let buffer_js = read_buffer_js(
            MIC_ANALYSIS_CONFIG.buffer_size,
            MIC_ANALYSIS_CONFIG.sample_rate,
        )
        .await;
        let js_array = Float32Array::from(buffer_js);

        js_array.copy_to(buffer);
        Ok(())
    }
}
