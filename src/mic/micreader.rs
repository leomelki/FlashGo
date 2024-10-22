use core::time::Duration;

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
    timer::EspTimerService,
};

pub struct Config {
    pub sample_rate: u32,
    pub buffer_size: usize,
}

pub const MIC_ANALYSIS_CONFIG: Config = Config {
    sample_rate: 3000, //40000 / 2,
    buffer_size: 1024,
};

pub struct MicReader<'a, Pin>
where
    Pin: ADCPin,
{
    channel: AdcChannelDriver<'a, Pin, AdcDriver<'a, Pin::Adc>>,
    buffer: [f32; MIC_ANALYSIS_CONFIG.buffer_size],
    index: usize,
}

impl<'a: 'static, Pin> MicReader<'a, Pin>
where
    Pin: ADCPin,
{
    pub fn new(pin: Pin, adc: impl Peripheral<P = Pin::Adc> + 'a) -> Result<Self, EspError> {
        log::info!("Created mic reader");
        let adc_driver = AdcDriver::new(adc)?;
        let adc_config = AdcChannelConfig {
            attenuation: DB_11,
            calibration: true,
            ..Default::default()
        };
        let adc_channel = AdcChannelDriver::new(adc_driver, pin, &adc_config)?;
        Ok(MicReader {
            channel: adc_channel,
            buffer: [0f32; MIC_ANALYSIS_CONFIG.buffer_size],
            index: 0,
        })
    }

    fn analyze_fft(&mut self) {
        let spectrum = microfft::real::rfft_1024(&mut self.buffer);
        const LEN_AMPLITUDES: usize = MIC_ANALYSIS_CONFIG.buffer_size / 2;
        let mut amplitudes = [0f32; LEN_AMPLITUDES];
        let mut max_ampl = 0f32;
        let mut max_freq = 0usize;

        for i in 0..LEN_AMPLITUDES {
            amplitudes[i] = spectrum[i].norm_sqr();
            if amplitudes[i] > max_ampl {
                max_ampl = amplitudes[i];
                max_freq = i;
            }
        }

        log::info!("Max fft: {} at index {}", max_ampl, max_freq);
    }

    pub fn update(&mut self) -> Result<(), EspError> {
        match self.index.cmp(&MIC_ANALYSIS_CONFIG.buffer_size) {
            core::cmp::Ordering::Less => {
                let read = self.channel.read()?;
                self.buffer[self.index] = read as f32;
                self.index += 1;
            }
            core::cmp::Ordering::Equal => {
                self.index += 1;
                log::info!("Analyze");
                // self.analyze_fft();
                self.index = 0;
            }
            core::cmp::Ordering::Greater => {
                log::info!("Buffer overflow");
            }
        }
        Ok(())
    }
}
