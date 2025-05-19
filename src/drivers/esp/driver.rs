use super::mic::MicESPImpl;
use super::{leds::LedsESPImpl, sync::EspSync};

pub use super::ble::EspServer;
use crate::drivers::ble::Server;
use crate::drivers::esp::sync::MASTER_MAC;
use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::{gpio::Gpio35, prelude::Peripherals};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sys::esp_wifi_get_mac;
use esp_idf_svc::wifi::{ClientConfiguration, Configuration, WifiDeviceId, WifiDriver};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref OWN_MAC: Mutex<[u8; 6]> = Mutex::new([0; 6]);
    static ref IS_MASTER: Mutex<bool> = Mutex::new(false);
}
pub const WIFI_CHANNEL: u8 = 7;

pub fn new() -> Result<(LedsESPImpl, MicESPImpl<Gpio35>, EspSync)> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let leds = LedsESPImpl::new(peripherals.rmt.channel0, peripherals.pins.gpio23)?;
    let mic = MicESPImpl::new(peripherals.pins.gpio35, peripherals.adc1)?;

    log::info!("Wi-Fi starting...");
    // Setup the Wi-Fi driver
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    // Create a WifiDriver instance.
    let mut wifi_driver = WifiDriver::new(peripherals.modem, sys_loop, Some(nvs)).unwrap();

    // Set the Wi-Fi configuration as a client
    wifi_driver
        .set_configuration(&Configuration::Client(ClientConfiguration::default()))
        .unwrap();

    // Wi-Fi start
    wifi_driver.start().unwrap();

    set_channel(WIFI_CHANNEL);

    log::info!("Wi-Fi started!");

    // Get and print the device's own MAC address
    let mac_address = get_mac(WifiDeviceId::Sta).unwrap();
    log::info!(
        "Device MAC address -> {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac_address[0],
        mac_address[1],
        mac_address[2],
        mac_address[3],
        mac_address[4],
        mac_address[5]
    );

    // Update MAC address in thread-safe manner
    if let Ok(mut mac) = OWN_MAC.lock() {
        *mac = mac_address;
    }

    if let Ok(mut is_master) = IS_MASTER.lock() {
        *is_master = mac_address == MASTER_MAC;
    }

    let sync = EspSync::new(wifi_driver).unwrap();

    Ok((leds, mic, sync))
}
pub fn get_mac(interface: WifiDeviceId) -> Result<[u8; 6]> {
    let mut mac = [0u8; 6];

    esp_idf_svc::sys::esp!(unsafe {
        esp_wifi_get_mac(interface.into(), mac.as_mut_ptr() as *mut _)
    })
    .unwrap();

    Ok(mac)
}

pub fn is_master() -> bool {
    let is_master = IS_MASTER.lock().unwrap();
    *is_master
}

pub fn create_ble_server() -> EspServer {
    EspServer::new()
}
pub fn set_channel(channel: u8) {
    unsafe {
        let second = esp_idf_svc::sys::wifi_second_chan_t_WIFI_SECOND_CHAN_NONE;
        esp_idf_svc::sys::esp_wifi_set_channel(channel, second);
    }
}
