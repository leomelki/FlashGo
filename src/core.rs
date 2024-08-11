use crate::leds::leds_controller::LedsController;
use crate::mic::Mic;
use crate::{consts, mic};
use core::ffi::{c_void, CStr, FromBytesWithNulError};
use esp_idf_svc::hal::cpu;
use esp_idf_svc::hal::task;
use esp_idf_svc::sys::{EspError, TaskHandle_t};

pub struct Core<'a: 'b, 'b> {
    leds_controller: &'b mut LedsController<'a>,
    mic: &'b Mic<'a>,
    task: TaskHandle_t,
}

impl<'a: 'b, 'b> Core<'a, 'b> {
    pub fn new(
        leds_controller: &'b mut LedsController<'a>,
        mut mic: &'b Mic<'a>,
    ) -> Result<Self, EspError> {
        const TASK_NAME: Result<&'static CStr, FromBytesWithNulError> =
            CStr::from_bytes_with_nul(b"MIC_ANALYSIS_THREAD\0");
        let mic_ptr: *mut c_void = &mut mic as *mut _ as *mut c_void;
        log::info!("Creating task!!!");
        let tk: TaskHandle_t;
        let core = Option::Some(cpu::Core::from(consts::OTHER_THREAD_ID));
        unsafe {
            tk = task::create(mic::start_task, TASK_NAME.unwrap(), 10000, mic_ptr, 5, core)?;
            log::info!("Createdd");
        }
        log::info!("Created");
        Ok(Core {
            leds_controller,
            mic,
            task: tk,
        })
    }

    pub fn start(&mut self) {}

    pub fn update(&mut self) -> Result<(), esp_idf_svc::sys::EspError> {
        self.leds_controller.update()
    }
}

pub trait Task {
    fn start(&mut self);
}
