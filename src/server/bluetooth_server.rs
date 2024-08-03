pub struct BluetoothServer {}

impl BluetoothServer {
    pub fn new() -> BluetoothServer {
        BluetoothServer {}
    }

    pub fn start(&self) {
        log::info!("BluetoothServer started");
    }
}

impl Default for BluetoothServer {
    fn default() -> Self {
        BluetoothServer::new()
    }
}
