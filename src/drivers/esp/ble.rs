use crate::{
    consts,
    drivers::ble::{self, UUIDAble},
};
use anyhow::Result;
use esp32_nimble::{
    utilities::{mutex::Mutex, BleUuid},
    BLEAdvertisementData, BLEAdvertising, BLECharacteristic, BLEDevice, BLEServer, BLEService,
    NimbleProperties,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct EspCharacteristic {
    name: String,
    readable: bool,
    writable: bool,
    ble_characteristic: Arc<Mutex<BLECharacteristic>>,
}

impl UUIDAble for EspCharacteristic {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl EspCharacteristic {
    fn new(
        ble_characteristic: Arc<Mutex<BLECharacteristic>>,
        name: &str,
        is_read: bool,
        is_write: bool,
    ) -> Self {
        EspCharacteristic {
            name: name.to_string(),
            readable: is_read,
            writable: is_write,
            ble_characteristic,
        }
    }
}

impl ble::Characteristic for EspCharacteristic {
    fn set_callback(&self, callback: impl Fn(&[u8]) -> Result<()> + Send + Sync + 'static) {
        // Only set the callback if the characteristic is already initialized
        let callback = Box::new(callback);
        self.ble_characteristic.lock().on_write(move |args| {
            let data = args.recv_data();
            let _ = callback(data);
        });
    }

    fn send_value<'a>(&self, value: &'a [u8]) {
        // If characteristic is already initialized, update the value
        self.ble_characteristic.lock().set_value(value);
        self.ble_characteristic.lock().notify();
    }
}

pub struct EspService {
    name: String,
    ble_service: Arc<Mutex<BLEService>>,
}

impl UUIDAble for EspService {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl EspService {
    fn new(name: &str, ble_service: Arc<Mutex<BLEService>>) -> Self {
        EspService {
            name: name.to_string(),
            ble_service,
        }
    }
}

impl ble::Service for EspService {
    type Characteristic = EspCharacteristic;
    fn register_characteristic(
        &mut self,
        name: &str,
        is_read: bool,
        is_write: bool,
    ) -> Result<EspCharacteristic> {
        // Create a new characteristic
        let mut properties = NimbleProperties::NOTIFY;

        if is_read {
            properties |= NimbleProperties::READ;
        }

        if is_write {
            properties |= NimbleProperties::WRITE;
        }

        let ble_char = self
            .ble_service
            .lock()
            .create_characteristic(get_uuid_from_name(name), properties);

        ble_char.lock().create_2904_descriptor();

        ble_char.lock().on_subscribe(|a, _args, ccc| {
            log::info!(
                "Client subscribed to characteristic '{}'",
                a.uuid(), // Use the name from the outer scope
            );
        });
        log::info!(
            "BLE characteristic created: {} ({})",
            name,
            get_uuid_from_name(name)
        );

        Ok(EspCharacteristic::new(ble_char, name, is_read, is_write))
    }
}

pub struct EspServer {
    device: &'static mut BLEDevice,
    server: &'static mut BLEServer,
    advertiser: &'static Mutex<BLEAdvertising>,
    services: Vec<EspService>,
    advertising_started: bool,
}

impl ble::Server for EspServer {
    type Service = EspService;
    fn new() -> Self {
        let device = BLEDevice::take();
        let server = device.get_server();
        let advertiser = device.get_advertising();

        server.on_connect(|server, desc| {
            log::info!("Client connected: {:?}", desc);

            server
                .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
                .unwrap();

            if server.connected_count() < (esp_idf_svc::sys::CONFIG_BT_NIMBLE_MAX_CONNECTIONS as _)
            {
                ::log::info!("Multi-connect support: start advertising");
                advertiser.lock().start().unwrap();
            }
        });

        EspServer {
            device,
            server,
            advertiser,
            services: Vec::new(),
            advertising_started: false,
        }
    }

    fn register_service(&mut self, name: &str) -> Result<EspService> {
        // Create BLE service
        let ble_service = self.server.create_service(get_uuid_from_name(name));

        log::info!(
            "BLE service created: {} ({})",
            name,
            get_uuid_from_name(name)
        );

        // Create a new service
        let service = EspService::new(name, ble_service);

        // Add to services list
        self.services.push(service.clone());

        Ok(service)
    }
    fn start_advertisement(&mut self) {
        let mut new_data = BLEAdvertisementData::new();
        let mut adv_data = new_data.name(consts::NAME);

        // Add service UUIDs to advertisement
        for service in &self.services {
            println!("Adding service UUID: {}", get_uuid(service));
            adv_data = adv_data.add_service_uuid(get_uuid(service));
        }
        let mut adv = self.advertiser.lock();

        adv.scan_response(true);
        adv.set_data(adv_data).unwrap();

        // Start advertising if not already started
        adv.start().unwrap();
        self.advertising_started = true;
    }
}

impl Clone for EspCharacteristic {
    fn clone(&self) -> Self {
        EspCharacteristic {
            name: self.name.clone(),
            readable: self.readable,
            writable: self.writable,
            ble_characteristic: self.ble_characteristic.clone(),
        }
    }
}

impl Clone for EspService {
    fn clone(&self) -> Self {
        EspService {
            name: self.name.clone(),
            ble_service: self.ble_service.clone(),
        }
    }
}
fn get_uuid128(element: &impl UUIDAble) -> BleUuid {
    get_uuid_128_from_name(element.get_name())
}

fn get_uuid_128_from_name(name: &str) -> BleUuid {
    BleUuid::from_uuid128(*Uuid::new_v5(&Uuid::NAMESPACE_X500, name.as_bytes()).as_bytes())
}

fn get_uuid(element: &impl UUIDAble) -> BleUuid {
    get_uuid_from_name(element.get_name())
}

fn get_uuid_from_name(name: &str) -> BleUuid {
    get_uuid_128_from_name(name)
    // let mut hash: u32 = 5381;
    // for byte in name.bytes() {
    //     hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u32);
    // }
    // BleUuid::from_uuid32(hash)
}
