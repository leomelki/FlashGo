use std::time::Instant;

use super::super::mic::Mic;
use super::super::mic::MIC_ANALYSIS_CONFIG;

use anyhow::Result;
use esp_idf_svc::hal::{
    adc::{
        attenuation::DB_11,
        oneshot::{config::AdcChannelConfig, *},
    },
    gpio::ADCPin,
    peripheral::Peripheral,
};
pub struct MicESPImpl<Pin>
where
    Pin: ADCPin,
{
    channel: AdcChannelDriver<'static, Pin, AdcDriver<'static, Pin::Adc>>,
    buffer: [f32; MIC_ANALYSIS_CONFIG.buffer_size],
}

impl<Pin> MicESPImpl<Pin>
where
    Pin: ADCPin,
{
    pub fn new(pin: Pin, adc: impl Peripheral<P = Pin::Adc> + 'static) -> Result<Self> {
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

impl<Pin> Mic for MicESPImpl<Pin>
where
    Pin: ADCPin,
{
    async fn read_buffer(&mut self) -> Result<[f32; MIC_ANALYSIS_CONFIG.buffer_size]> {
        //respect the sample rate
        let sample_period = 1_000_000_000 / MIC_ANALYSIS_CONFIG.sample_rate;
        for x in self.buffer.iter_mut().take(MIC_ANALYSIS_CONFIG.buffer_size) {
            let start_read = Instant::now();
            *x = self.channel.read()? as f32;
            while start_read.elapsed().as_nanos() < sample_period as u128 {
                // wait
            }
        }

        Ok(self.buffer)
    }
}
