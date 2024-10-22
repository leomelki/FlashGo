use core::ffi::{c_void, CStr, FromBytesWithNulError};
use esp_idf_svc::hal::cpu;
use esp_idf_svc::hal::gpio::ADCPin;
use esp_idf_svc::hal::task;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::timer::EspTimerService;
use std::time::Duration;

use crate::consts;
use crate::mic;

use super::micreader::MicReader;

pub struct Mic {}

impl Mic {
    pub fn new() -> Result<Self, EspError> {
        Ok(Mic {})
    }

    pub fn start_task<Pin>(&mut self, reader: &mut MicReader<Pin>) -> Result<(), EspError>
    where
        Pin: ADCPin,
    {
        /*const TASK_NAME: Result<&'static CStr, FromBytesWithNulError> =
            CStr::from_bytes_with_nul(b"MIC_ANALYSIS_THREAD\0");
        let mic_ptr: *mut c_void = reader as *mut _ as *mut c_void;
        log::info!("Creating task!!!");
        let core = Option::Some(cpu::Core::from(consts::OTHER_THREAD_ID));
        unsafe {
            task::create(
                mic::micreader::start_mic_reader_task::<Pin>,
                TASK_NAME.unwrap(),
                10000,
                mic_ptr,
                5,
                core,
            )?;
            log::info!("Createdd");
        }
        log::info!("Created");*/
        log::info!("a");
        let static_reader = unsafe {
            core::mem::transmute::<&mut MicReader<Pin>, &'static mut MicReader<Pin>>(reader)
        };
        log::info!("b");

        const SAMPLE_PERIOD: u32 = 1_000_000_000 / mic::micreader::MIC_ANALYSIS_CONFIG.sample_rate;
        let wait_duration: Duration = Duration::from_nanos(SAMPLE_PERIOD as u64);

        let service = EspTimerService::new()?;
        log::info!("start mic");

        let timer = service.timer(move || {
            static_reader.update().unwrap();
        })?;
        log::info!("created");
        timer.every(wait_duration)?;
        log::info!("started");
        std::thread::sleep(Duration::from_millis(10));
        log::info!("started mic");
        Ok(())
    }
}
