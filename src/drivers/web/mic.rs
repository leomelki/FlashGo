use crate::drivers::mic::Mic;

pub struct MicSimImpl {}

#[wasm_bindgen]
extern "C" {
    fn read_buffer(size: usize) -> [f32; size];
}

impl MicSimImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Mic for MicSimImpl {
    fn read_buffer(&mut self) -> Result<[f32; MIC_ANALYSIS_CONFIG.buffer_size], DriverError> {
        Ok(unsafe { read_buffer(MIC_ANALYSIS_CONFIG.buffer_size) })
    }
}
