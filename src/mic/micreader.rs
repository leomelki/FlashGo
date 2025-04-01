use esp_idf_svc::{
    hal::{
        adc::{
            attenuation::DB_11,
            oneshot::{config::AdcChannelConfig, *},
        },
        delay::Delay,
        gpio::ADCPin,
        peripheral::Peripheral,
    },
    sys::EspError,
};

pub struct Config {
    pub sample_rate: usize,
    pub buffer_size: usize,
}

const MIC_ANALYSIS_CONFIG: Config = Config {
    sample_rate: 44100 / 4,
    buffer_size: 1024,
};

pub struct MicReader<Pin>
where
    Pin: ADCPin,
{
    channel: AdcChannelDriver<'static, Pin, AdcDriver<'static, Pin::Adc>>,
}

impl<Pin> MicReader<Pin>
where
    Pin: ADCPin,
{
    pub fn new(pin: Pin, adc: impl Peripheral<P = Pin::Adc> + 'static) -> Result<Self, EspError> {
        let adc_driver = AdcDriver::new(adc)?;
        let adc_config: AdcChannelConfig = AdcChannelConfig {
            calibration: true,
            attenuation: DB_11,
            ..Default::default()
        };
        let adc_channel = AdcChannelDriver::new(adc_driver, pin, &adc_config)?;

        Ok(MicReader {
            channel: adc_channel,
        })
    }

    fn analyze_fft(&mut self, buffer: &mut [f32; MIC_ANALYSIS_CONFIG.buffer_size]) {
        let mean = buffer.iter().sum::<f32>() / buffer.len() as f32;

        // subtract the mean
        for x in buffer.iter_mut() {
            *x -= mean;
        }

        let spectrum = microfft::real::rfft_1024(buffer);

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

    pub fn read_buffer_process(&mut self) -> Result<(), EspError> {
        let buffer = &mut [0f32; MIC_ANALYSIS_CONFIG.buffer_size];
        //respect the sample rate
        let sample_period = 1_000_000_000 / MIC_ANALYSIS_CONFIG.sample_rate;

        let start = std::time::Instant::now();
        for x in buffer.iter_mut().take(MIC_ANALYSIS_CONFIG.buffer_size) {
            let start_read = std::time::Instant::now();
            *x = self.channel.read()? as f32;
            while start_read.elapsed().as_nanos() < sample_period as u128 {
                // wait
            }
        }

        let elapsed = start.elapsed();

        log::info!(
            "Elapsed: {:?}ns and should be {:?}. It is {:?}%",
            elapsed.as_nanos(),
            sample_period * MIC_ANALYSIS_CONFIG.buffer_size,
            (elapsed.as_nanos() as f32 / (sample_period * MIC_ANALYSIS_CONFIG.buffer_size) as f32)
                * 100.0
        );

        Delay::new(100).delay_ms(1);
        // log::info!("Elapsed after wait: {:?}", elapsed.as_millis());

        self.analyze_fft(buffer);

        let elapsed = start.elapsed();
        log::info!("Freq: {:?} Hz", 1000 / elapsed.as_millis());

        Ok(())
    }
}
