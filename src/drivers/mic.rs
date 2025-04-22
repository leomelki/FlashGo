use anyhow::Result;

pub struct Config {
    pub sample_rate: usize,
    pub buffer_size: usize,
}

pub const MIC_ANALYSIS_CONFIG: Config = Config {
    sample_rate: 44100 / 4,
    buffer_size: 1024,
};

pub trait Mic {
    async fn read_buffer(
        &mut self,
        buffer: &mut [f32; MIC_ANALYSIS_CONFIG.buffer_size],
    ) -> Result<()>;
}
