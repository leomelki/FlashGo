use std::sync::Arc;

use esp32_nimble::{
    utilities::{mutex::Mutex, BleUuid},
    BLECharacteristic, BLEService, NimbleProperties,
};
use uuid::Uuid;

use crate::server::bluetooth::server::BluetoothServer;

pub trait Service {
    fn new(server: BluetoothServer) -> Self;
    fn init(&mut self);
}

pub fn create_service(server: BluetoothServer, name: &[u8]) -> Arc<Mutex<BLEService>> {
    let uuid: Uuid = Uuid::new_v5(&Uuid::NAMESPACE_X500, name);
    return server
        .server
        .create_service(BleUuid::from_uuid128(*uuid.as_bytes()));
}

pub trait Characteristic: Sync + Send {
    fn new(service: Arc<Mutex<BLEService>>) -> Self
    where
        Self: Sized;
}
pub trait WritableCharacteristic: Sync + Send {
    fn on_write(&mut self, data: &[u8]);
}

pub fn create_characteristic(
    service: Arc<Mutex<BLEService>>,
    name: &[u8],
    write: bool,
) -> Arc<Mutex<BLECharacteristic>> {
    let uuid: Uuid = Uuid::new_v5(&Uuid::NAMESPACE_X500, name);
    let mut properties = NimbleProperties::READ | NimbleProperties::NOTIFY;

    if write {
        properties |= NimbleProperties::WRITE;
    }

    service
        .lock()
        .create_characteristic(BleUuid::from_uuid128(*uuid.as_bytes()), properties)
}

pub fn init_writable_characteristic(
    ble_char: Arc<Mutex<BLECharacteristic>>,
    mut characteristic: Box<dyn WritableCharacteristic>,
) {
    ble_char.lock().on_write(move |args| {
        characteristic.on_write(args.recv_data());
    });
}
