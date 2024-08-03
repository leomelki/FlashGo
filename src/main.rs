#![no_std]
#![no_main]

mod leds;
mod mic;
mod server;

use esp_idf_svc::sys::EspError;

#[no_mangle]
fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!3");

    leds::leds_controller::LedsController::new()?.update()
}
