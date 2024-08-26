use esp32_nimble::{
    utilities::mutex::Mutex, uuid128, BLEAdvertisementData, BLEAdvertising, BLEDevice, BLEServer,
    NimbleProperties,
};

use crate::consts;

pub struct BluetoothServer {
    device: &'static mut BLEDevice,
    pub server: &'static mut BLEServer,
    advertiser: &'static Mutex<BLEAdvertising>,
}

impl BluetoothServer {
    pub fn new() -> BluetoothServer {
        //init
        let ble_device = BLEDevice::take();
        let ble_advertiser = ble_device.get_advertising();
        let server = ble_device.get_server();
        BluetoothServer {
            server,
            device: ble_device,
            advertiser: ble_advertiser,
        }
    }

    pub fn start(&mut self) {
        log::info!("Starting Bluetooth Server");
        let uid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_X500, b"Salut ca va ?");
        log::info!("{:?}", uid);
        let uid1 = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_X500, b"Salut ca va ?");
        log::info!("{:?}", uid1);
        let uid2 = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_X500, b"Salut ca va !");
        log::info!("{:?}", uid2);

        self.server.on_connect(|server, clntdesc| {
            // Print connected client data
            println!("{:?}", clntdesc);
            // Update connection parameters
            server
                .update_conn_params(clntdesc.conn_handle(), 24, 48, 0, 60)
                .unwrap();
        });
        self.server.on_disconnect(|_desc, _reason| {
            println!("Disconnected, back to advertising");
        });

        let my_service = self
            .server
            .create_service(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1"));

        // Create a characteristic to associate with created service
        let my_service_characteristic = my_service.lock().create_characteristic(
            uuid128!("681285a6-247f-48c6-80ad-68c3dce18585"),
            NimbleProperties::READ | NimbleProperties::NOTIFY,
        );

        // Modify characteristic value
        my_service_characteristic.lock().set_value(b"Start Value");

        // Configure Advertiser Data
        self.advertiser
            .lock()
            .set_data(
                BLEAdvertisementData::new()
                    .name(consts::NAME)
                    .add_service_uuid(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1")),
            )
            .unwrap();

        // Start Advertising
        self.advertiser.lock().start().unwrap();

        // (Optional) Print dump of local GATT table
        // server.ble_gatts_show_local();

        // Init a value to pass to characteristic
        let mut val = 0;

        /*loop {
            FreeRtos::delay_ms(1000);
            my_service_characteristic.lock().set_value(&[val]).notify();
            val = val.wrapping_add(1u8);
        }*/
    }
}

impl Default for BluetoothServer {
    fn default() -> Self {
        BluetoothServer::new()
    }
}
