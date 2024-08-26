use std::sync::Arc;

use esp32_nimble::{utilities::mutex::Mutex, BLEService};

use crate::server::bluetooth::{
    server::BluetoothServer,
    services::service::{create_service, Service},
};

struct Identity {
    service: Arc<Mutex<BLEService>>,
}

const NAME: &[u8] = b"identity";

impl Service for Identity {
    fn new(server: BluetoothServer) -> Self {
        Identity {
            service: create_service(server, NAME),
        }
    }

    fn init(&mut self) {
        log::info!("Identity service initialized");
    }
}
