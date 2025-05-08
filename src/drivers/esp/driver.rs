use super::mic::MicESPImpl;
use super::{leds::LedsESPImpl, sync::EspSync};

pub use super::ble::EspServer;
use crate::drivers::ble::Server;
use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::{gpio::Gpio35, prelude::Peripherals};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{ClientConfiguration, Configuration, WifiDriver};

pub fn new() -> Result<(LedsESPImpl, MicESPImpl<Gpio35>, EspSync)> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let leds = LedsESPImpl::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?;
    let mic = MicESPImpl::new(peripherals.pins.gpio35, peripherals.adc1)?;

    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs: esp_idf_svc::nvs::EspNvsPartition<esp_idf_svc::nvs::NvsDefault> =
        EspDefaultNvsPartition::take().unwrap();

    // Create a WifiDriver instance.
    let mut wifi_driver = WifiDriver::new(peripherals.modem, sys_loop, Some(nvs)).unwrap();

    // Set the Wi-Fi configuration as a client
    wifi_driver
        .set_configuration(&Configuration::Client(ClientConfiguration::default()))
        .unwrap();

    // Wi-Fi start
    wifi_driver.start().unwrap();

    let sync = EspSync::new()?;
    Ok((leds, mic, sync))
}

pub fn create_ble_server() -> EspServer {
    EspServer::new()
}
