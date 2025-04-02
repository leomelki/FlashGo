use super::driver::DriverError;

pub struct Config {
    pub sample_rate: usize,
    pub buffer_size: usize,
}

pub const MIC_ANALYSIS_CONFIG: Config = Config {
    sample_rate: 44100 / 4,
    buffer_size: 1024 / 4,
};

pub trait Mic {
    fn read_buffer(&mut self) -> Result<[f32; MIC_ANALYSIS_CONFIG.buffer_size], DriverError>;
}
