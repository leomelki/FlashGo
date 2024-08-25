use crate::leds::leds_controller::LedsController;
use crate::mic::micreader::MicReader;
use crate::{consts, mic};
use core::ffi::{c_void, CStr, FromBytesWithNulError};
use esp_idf_svc::hal::cpu;
use esp_idf_svc::hal::gpio::ADCPin;
use esp_idf_svc::hal::task;
use esp_idf_svc::sys::{EspError, TaskHandle_t};

pub struct Core<'a: 'b, 'b, Pin>
where
    Pin: ADCPin,
{
    leds_controller: &'b mut LedsController<'a>,
    mic: &'b MicReader<'a, Pin>,
}

impl<'a: 'b, 'b, Pin> Core<'a, 'b, Pin>
where
    Pin: ADCPin,
{
    pub fn new(
        leds_controller: &'b mut LedsController<'a>,
        mut mic: &'b MicReader<'a, Pin>,
    ) -> Result<Self, EspError> {
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
