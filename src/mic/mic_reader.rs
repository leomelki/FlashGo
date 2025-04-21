use anyhow::Result;

use crate::drivers::{
    driver::{delay_ms, log_data, Instant},
    mic::{Mic, MIC_ANALYSIS_CONFIG},
};

pub struct MicReader<M> {
    mic: M,
}

impl<M: Mic> MicReader<M> {
    pub fn new(mic: M) -> Self {
        MicReader { mic }
    }

    fn analyze_fft(&self, buffer: &mut [f32; MIC_ANALYSIS_CONFIG.buffer_size]) {
        let mean = buffer.iter().sum::<f32>() / buffer.len() as f32;

        // subtract the mean
        for x in buffer.iter_mut() {
            *x -= mean;
        }

        let spectrum = microfft::real::rfft_1024(buffer);

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

        let mut buffer = self.mic.read_buffer().await?;

        delay_ms(1).await;

        self.analyze_fft(&mut buffer);

        let elapsed = start.elapsed();
        log_data("polling_frequency_hz", 1000.0 / elapsed.as_millis() as f32);

        Ok(())
    }
}
