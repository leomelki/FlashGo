use esp_idf_svc::hal::gpio::ADCPin;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::timer::EspTimerService;
use std::time::Duration;

use super::micreader::MicReader;

pub struct Mic {}

impl Mic {
    pub fn new() -> Result<Self, EspError> {
        Ok(Mic {})
    }

    pub fn start_task<Pin>(
        &mut self,
        pin: Pin,
        adc: impl Peripheral<P = Pin::Adc> + 'static,
    ) -> Result<(), EspError>
    where
        Pin: ADCPin,
    {
        log::info!("a");

        let mut reader = MicReader::new(pin, adc)?;
        log::info!("b");

        const SAMPLE_PERIOD: u32 = 1_000_000_000 / MIC_ANALYSIS_CONFIG.sample_rate;
        let wait_duration: Duration = Duration::from_nanos(SAMPLE_PERIOD as u64);

        let service = EspTimerService::new().unwrap();
        log::info!("start mic");

        let timer = service
            .timer(move || {
                // reader.update().unwrap();
            })
            .unwrap();

        timer.every(wait_duration).unwrap();
        std::thread::sleep(Duration::from_millis(10));
        log::info!("started mic");
        Ok(())
    }
}
