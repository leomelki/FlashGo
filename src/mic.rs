use esp_idf_svc::hal::{
    adc::continuous::{config::Config, *},
    gpio::ADCPin,
    peripheral::Peripheral,
};

pub struct Mic {}

impl Mic {
    pub fn new<C: Adc>(
        channel: impl Peripheral<P = C>,
        pin: impl Peripheral<P = impl ADCPin>,
    ) -> Self {
        let mut adc = AdcDriver::new(pin, &Config::new().calibration(true))?;
        Mic {}
    }

    fn start(&mut self) {
        loop {
            // Do something
        }
    }

    fn read_data(&self) {
        //analog read
        // Read data from the microphone
    }
}

pub extern "C" fn start_task(mic: *mut core::ffi::c_void) {
    mic.start();
}
