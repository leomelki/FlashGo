use std::sync::Arc;

use esp32_nimble::{utilities::mutex::Mutex, BLECharacteristic, BLEService};

use crate::server::bluetooth::services::service::{create_characteristic, Characteristic};

const NAME: &[u8] = b"flashgo";

pub struct Identity {
    characteristic: Arc<Mutex<BLECharacteristic>>,
}

impl Characteristic for Identity {
    fn new(service: Arc<Mutex<BLEService>>) -> Self {
        Identity {
            characteristic: create_characteristic(service, NAME, false),
        }
    }
}
