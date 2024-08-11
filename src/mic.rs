use esp_idf_svc::hal::{
    adc::{
        continuous::{config::Config, *},
        Adc, ADC1,
    },
    delay::Delay,
    i2s::I2S0,
    peripheral::Peripheral,
    units::Hertz,
};

pub struct Mic<'a> {
    adc: AdcDriver<'a>,
}

impl<'a> Mic<'a> {
    pub fn new(
        adc: impl Peripheral<P = ADC1> + 'a,
        i2s: impl Peripheral<P = I2S0> + 'a,
        channels: impl AdcChannels<Adc = ADC1> + 'a,
    ) -> Self {
        let freq = Hertz(10000);
        log::info!("Creating mic");
        let adc = AdcDriver::new(
            adc,
            i2s,
            &Config::new(), /*.sample_freq(freq)*/
            channels,
        )
        .unwrap();

        log::info!("Created mic");
        Mic { adc }
    }

    fn start(&mut self) {
        // loop {
        //     // Do something
        // }
    }

    fn read_data(&self) {
        //analog read
        // Read data from the microphone
    }
}

pub extern "C" fn start_task(micPtr: *mut core::ffi::c_void) {
    log::info!("yay");
    // mic.start();
    let mic: &mut Mic = unsafe { &mut *(micPtr as *mut Mic) };
    let delay = Delay::new(1);
    log::info!("st");
    mic.adc.start();
    log::info!("started");
    let mut buf = [AdcMeasurement::default(); 100];
    loop {
        delay.delay_ms(500);
        log::info!("read");
        let value = mic.adc.read(&mut buf, 10000).unwrap();
        log::info!("Value is {}", value);
    }
}
