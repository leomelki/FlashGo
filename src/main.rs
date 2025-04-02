#![allow(clippy::needless_lifetimes)]
mod consts;
mod drivers;
mod leds;
mod mic;
mod server;
use drivers::leds::Color;
use esp_idf_svc::sys::EspError;
use leds::leds_controller::LedsController;

fn main() -> Result<(), EspError> {
    let driver = Box::leak(crate::drivers::driver::create_driver()?);

    let mut led_controller = LedsController::new(driver.take_leds())?;

    led_controller.set_color(3, 1, Color::red());
    led_controller.set_color(3, 2, Color::red());
    led_controller.set_color(3, 3, Color::red());
    led_controller.set_color(3, 4, Color::red());
    led_controller.set_color(3, 5, Color::red());
    led_controller.set_color(2, 5, Color::red());
    led_controller.set_color(1, 5, Color::red());
    led_controller.set_color(0, 0, Color::red());
    led_controller.update()?;

    // log::info!("1!");

    // unsafe {
    //     let remaining_ram = esp_idf_svc::sys::esp_get_free_heap_size();
    //     log::info!("Remaining RAM 1: {}", remaining_ram);
    // }
    // let mut mic_reader = mic::micreader::MicReader::new(peripherals.pins.gpio35, peripherals.adc1)?;

    // unsafe {
    //     let remaining_ram = esp_idf_svc::sys::esp_get_free_heap_size();
    //     log::info!("Remaining RAM 2: {}", remaining_ram);
    // }

    // loop {
    //     mic_reader.read_buffer_process()?;
    // }

    let mic = driver.take_mic();
    let mut mic_reader = mic::micreader::MicReader::new(mic);
    loop {
        mic_reader.read_buffer_process()?;
    }
    Ok(())
    // let mut mic = mic::mic::Mic::new()?;
    // mic.start_task(peripherals.pins.gpio33, peripherals.adc1)?;
    // log::info!("done");
}
