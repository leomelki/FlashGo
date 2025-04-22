use anyhow::Result;

use crate::drivers::{
    driver::{delay_ms, log_data, Instant},
    mic::{Mic, MIC_ANALYSIS_CONFIG},
};

pub struct MicReader<M> {
    mic: M,
    buffer: Box<[f32; MIC_ANALYSIS_CONFIG.buffer_size]>,
}

impl<M: Mic> MicReader<M> {
    pub fn new(mic: M) -> Self {
        MicReader {
            mic,
            buffer: Box::new([0.0; MIC_ANALYSIS_CONFIG.buffer_size]),
        }
    }

    fn analyze_fft(&mut self) {
        let mean = self.buffer.iter().sum::<f32>() / self.buffer.len() as f32;

        // subtract the mean
        for x in self.buffer.iter_mut() {
            *x -= mean;
        }

        let spectrum = microfft::real::rfft_1024(&mut self.buffer);

        spectrum[0].im = 0.0;

        // the spectrum has a spike at index `signal_freq`
        let amplitudes: Vec<_> = spectrum.iter().map(|c| c.norm_sqr() as u32).collect();

        let index_80hz = 80 * MIC_ANALYSIS_CONFIG.buffer_size / MIC_ANALYSIS_CONFIG.sample_rate;

        let index_40hz = 30 * MIC_ANALYSIS_CONFIG.buffer_size / MIC_ANALYSIS_CONFIG.sample_rate;

        // between 40 and 80 hz
        let bass_volume = amplitudes
            .iter()
            .skip(index_40hz)
            .take(index_80hz - index_40hz)
            .sum::<u32>() as f32
            / (index_80hz - index_40hz) as f32;

        log_data("bass_volume", bass_volume as f32);
    }

    pub async fn read_buffer_process(&mut self) -> Result<()> {
        let start = Instant::now();

        self.mic.read_buffer(&mut self.buffer).await?;

        delay_ms(1).await;

        self.analyze_fft();

        let elapsed = start.elapsed();
        log_data("polling_frequency_hz", 1000.0 / elapsed.as_millis() as f32);

        Ok(())
    }
}
