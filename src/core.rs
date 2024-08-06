use crate::leds::leds_controller::LedsController;
use crate::mic::Mic;
use crate::{consts, mic};
use core::ffi::{c_void, CStr};
use cstr_core::CString;
use esp_idf_svc::hal::cpu;
use esp_idf_svc::hal::task;
use esp_idf_svc::sys::EspError;

pub struct Core<'a> {
    leds_controller: LedsController<'a>,
    mic: Mic,
}

impl<'a> Core<'a> {
    pub fn new(leds_controller: LedsController<'a>, mut mic: Mic) -> Result<Self, EspError> {
        unsafe {
            let mic_ptr: *mut c_void = &mut mic as *mut _ as *mut c_void;

            task::create(
                mic::start_task,
                CStr::from_ptr(
                    CString::new("MIC_ANALYSIS_THREAD")
                        .expect("Cannot create string")
                        .as_ptr(),
                ),
                3000,
                mic_ptr,
                10,
                Option::Some(cpu::Core::from(consts::MIC_THREAD_ID)),
            )?;
        }
        Ok(Core {
            leds_controller,
            mic,
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
