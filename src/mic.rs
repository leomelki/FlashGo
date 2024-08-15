use core::{ops::Add, time::Duration};

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

const MIC_ANALYSIS_CONFIG: Config = Config {
    sample_rate: 40000,
    buffer_size: 1024,
};

pub struct Mic<'a, Pin>
where
    Pin: ADCPin,
{
    channel: AdcChannelDriver<'a, Pin, AdcDriver<'a, Pin::Adc>>,
}

impl<'a: 'static, Pin> Mic<'a, Pin>
where
    Pin: ADCPin,
{
    pub fn new(pin: Pin, adc: impl Peripheral<P = Pin::Adc> + 'a) -> Result<Self, EspError> {
        log::info!("Created mic");
        let adc_driver = AdcDriver::new(adc)?;
        let adc_config = AdcChannelConfig {
            attenuation: DB_11,
            calibration: true,
            ..Default::default()
        };
        let adc_channel = AdcChannelDriver::new(adc_driver, pin, &adc_config)?;
        Ok(Mic {
            channel: adc_channel,
        })
    }

    fn start(&'a mut self) -> Result<(), EspError> {
        log::info!("started");
        let timer_service = EspTimerService::new()?;

        let mut buffer = [0; MIC_ANALYSIS_CONFIG.buffer_size];
        const SAMPLE_PERIOD: u32 = 1_000_000_000 / MIC_ANALYSIS_CONFIG.sample_rate;
        let wait_duration: Duration = Duration::from_nanos(SAMPLE_PERIOD as u64);
        log::info!("started 2");

        let mut i = 0;
        let timer = timer_service.timer(move || match i.cmp(&MIC_ANALYSIS_CONFIG.buffer_size) {
            core::cmp::Ordering::Less => {
                log::info!("read");
                let read = self.channel.read();
                if read.is_err() {
                    log::info!("Error reading");
                } else {
                    log::info!("success");
                    buffer[i] = read.unwrap();
                    log::info!("defined");
                }
                log::info!("red");
                i += 1;
            }
            core::cmp::Ordering::Equal => {
                i += 1;
                log::info!("Process and reset");
                i = 0;
            }
            core::cmp::Ordering::Greater => {
                log::info!("Buffer overflow");
            }
        })?;
        log::info!("started 3");

        timer.every(wait_duration)?;
        log::info!("started done");
        Ok(())
    }

    #[inline(always)]
    fn read_data(&mut self) -> Result<u16, EspError> {
        self.channel.read()
    }
}

pub extern "C" fn start_task<Pin>(mic_ptr: *mut core::ffi::c_void)
where
    Pin: ADCPin,
{
    log::info!("Microphone task started");
    let mic: &mut Mic<Pin> = unsafe { &mut *(mic_ptr as *mut Mic<Pin>) };
    log::info!("Mic ptr: {:?}", mic_ptr);
    mic.start();
    log::info!("Mic started");
}
