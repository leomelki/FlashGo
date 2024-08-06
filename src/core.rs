use crate::leds::leds_controller::LedsController;

pub struct Core<'a> {
    leds_controller: LedsController<'a>,
}

impl<'a> Core<'a> {
    pub fn new(leds_controller: LedsController<'a>) -> Result<Self, esp_idf_svc::sys::EspError> {
        Ok(Core { leds_controller })
    }

    pub fn start(&mut self) {}

    pub fn update(&mut self) -> Result<(), esp_idf_svc::sys::EspError> {
        self.leds_controller.update()
    }
}
