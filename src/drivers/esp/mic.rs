use crate::drivers::driver::DriverError;

use super::super::mic::Mic as MicTrait;
use super::super::mic::MIC_ANALYSIS_CONFIG;
use esp_idf_svc::hal::{
    adc::{
        attenuation::DB_11,
        oneshot::{config::AdcChannelConfig, *},
    },
    delay::Delay,
    gpio::ADCPin,
    peripheral::Peripheral,
};
pub struct Mic<Pin>
where
    Pin: ADCPin,
{
    channel: AdcChannelDriver<'static, Pin, AdcDriver<'static, Pin::Adc>>,
    buffer: [f32; MIC_ANALYSIS_CONFIG.buffer_size],
}

impl<Pin> Mic<Pin>
where
    Pin: ADCPin,
{
    pub fn new(
        pin: Pin,
        adc: impl Peripheral<P = Pin::Adc> + 'static,
    ) -> Result<Self, DriverError> {
        let adc_driver = AdcDriver::new(adc)?;
        let adc_config: AdcChannelConfig = AdcChannelConfig {
            calibration: true,
            attenuation: DB_11,
            ..Default::default()
        };
        let channel = AdcChannelDriver::new(adc_driver, pin, &adc_config)?;

        Ok(Self {
            channel,
            buffer: [0f32; MIC_ANALYSIS_CONFIG.buffer_size],
        })
    }
}

impl<Pin> MicTrait for Mic<Pin>
where
    Pin: ADCPin,
{
    fn read_buffer(&mut self) -> Result<&mut [f32; MIC_ANALYSIS_CONFIG.buffer_size], DriverError> {
        //respect the sample rate
        let sample_period = 1_000_000_000 / MIC_ANALYSIS_CONFIG.sample_rate;

        let start = std::time::Instant::now();
        for x in self.buffer.iter_mut().take(MIC_ANALYSIS_CONFIG.buffer_size) {
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

        Ok(&mut self.buffer)
    }
}
