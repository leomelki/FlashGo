use anyhow::Result;

use crate::drivers::{
    driver::{self, delay_ms, Instant},
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

        let spectrum = microfft::real::rfft_256(buffer);

        spectrum[0].im = 0.0;

        // the spectrum has a spike at index `signal_freq`
        let amplitudes: Vec<_> = spectrum.iter().map(|c| c.norm_sqr() as u32).collect();

        // let max_amplitude = amplitudes.iter().max().unwrap();
        // let max_amplitude_index = amplitudes.iter().position(|x| x == max_amplitude).unwrap();

        // // convert to frequency
        // let signal_freq =
        //     max_amplitude_index * MIC_ANALYSIS_CONFIG.sample_rate / MIC_ANALYSIS_CONFIG.buffer_size;

        // log::info!("Signal frequency: {} Hz", signal_freq);

        let index_80hz = 80 * MIC_ANALYSIS_CONFIG.buffer_size / MIC_ANALYSIS_CONFIG.sample_rate;

        let bass_volume = amplitudes.iter().take(index_80hz).sum::<u32>() / index_80hz as u32;

        // log::info!("Bass volume: {}", bass_volume);
        log::info!("{}", bass_volume);

        // let other_volume = amplitudes.iter().skip(index_80hz).sum::<u32>()
        //     / (MIC_ANALYSIS_CONFIG.buffer_size - index_80hz) as u32;

        // // log::info!("Other volume: {}", other_volume);

        // let ratio = bass_volume as f32 / other_volume as f32;

        // print the amplitudes
        // log::info!("{:?}", amplitudes);
    }

    pub async fn read_buffer_process(&mut self) -> Result<()> {
        let start = Instant::now();

        let mut buffer = self.mic.read_buffer().await?;

        let elapsed = start.elapsed();

        let sample_period = 1_000_000_000 / MIC_ANALYSIS_CONFIG.sample_rate;
        log::info!(
            "Elapsed: {:?}ns and should be {:?}. It is {:?}%",
            elapsed.as_nanos(),
            sample_period * MIC_ANALYSIS_CONFIG.buffer_size,
            (elapsed.as_nanos() as f32 / (sample_period * MIC_ANALYSIS_CONFIG.buffer_size) as f32)
                * 100.0
        );

        delay_ms(1).await;
        // log::info!("Elapsed after wait: {:?}", elapsed.as_millis());

        self.analyze_fft(&mut buffer);

        let elapsed = start.elapsed();
        log::info!("Freq: {:?} Hz", 1000 / elapsed.as_millis());

        Ok(())
    }
}
