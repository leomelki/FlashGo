use crate::leds::leds_controller::LedsController;
use crate::mic::micreader::MicReader;
use esp_idf_svc::hal::gpio::ADCPin;
use esp_idf_svc::sys::EspError;

pub struct Core<'a, Pin>
where
    Pin: ADCPin,
{
    leds_controller: &'a mut LedsController,
    mic: &'a MicReader<Pin>,
}

impl<'a, Pin> Core<'a, Pin>
where
    Pin: ADCPin,
{
    pub fn new(
        leds_controller: &'a mut LedsController,
        mut mic: &'a MicReader<Pin>,
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
