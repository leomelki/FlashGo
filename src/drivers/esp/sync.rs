use std::sync::{Arc, Mutex};
use std::thread::Builder;

use crate::drivers::{driver, esp::driver::WIFI_CHANNEL};

use super::super::sync::Sync;
use anyhow::Result;
use esp_idf_svc::{
    espnow::{EspNow, PeerInfo, BROADCAST},
    sys::wifi_second_chan_t_WIFI_SECOND_CHAN_NONE,
    wifi::{BlockingWifi, Configuration, EspWifi},
};

pub struct EspSync {
    pub espnow: Arc<Mutex<EspNow<'static>>>,
}

impl EspSync {
    pub fn new_master() -> Result<Self> {
        let espnow = EspNow::take()?;
        let espnow = Arc::new(Mutex::new(espnow));

        let esp_sync = Self {
            espnow: espnow.clone(),
        };

        esp_sync.init_master()?;

        Ok(esp_sync)
    }

    pub fn new_slave() -> Result<Self> {
        let espnow = EspNow::take()?;
        let espnow = Arc::new(Mutex::new(espnow));

        let esp_sync = Self {
            espnow: espnow.clone(),
        };

        esp_sync.init_slave()?;

        Ok(esp_sync)
    }

    fn init_slave(&self) -> Result<()> {
        println!("Initializing slave");

        // Register callbacks and add peer
        {
            let mut espnow = self.espnow.lock().unwrap();

            // Register the receive callback to handle incoming messages
            espnow.register_recv_cb(|mac_address, data| {
                println!("Received message from MAC: {:?}", mac_address);
                println!("Data length: {} bytes", data.len());
            })?;

            // Register the send callback
            espnow.register_send_cb(|_mac_addres, status| {
                println!("Send status: {:?}", status);
            })?;

            // Add the master peer
            let peer = PeerInfo {
                peer_addr: MASTER_MAC,
                channel: 1,
                ifidx: 1,
                encrypt: false,
                ..Default::default()
            };
            espnow.add_peer(peer).unwrap();
            println!("Peer added: {:?}", MASTER_MAC);

            espnow.send(MASTER_MAC, &[0x01]).unwrap();
        }

        Ok(())
    }

    fn init_master(&self) -> Result<()> {
        println!("Initializing master");

        // Set up ESP-NOW with broadcast peer and callback
        {
            let mut espnow = self.espnow.lock().unwrap();

            // Add broadcast peer
            let broadcast = esp_idf_svc::sys::esp_now_peer_info {
                channel: WIFI_CHANNEL,
                ifidx: esp_idf_svc::sys::wifi_interface_t_WIFI_IF_AP,
                encrypt: false,
                peer_addr: BROADCAST,
                ..Default::default()
            };
            espnow.add_peer(broadcast).unwrap();

            espnow.register_recv_cb(|mac_address, _data| {
                // Convert slice to array
                let mac_address_array = mac_address.try_into().unwrap();
                // If peer does not exist, add it
                if let Ok(false) = espnow.peer_exists(mac_address_array) {
                    // Add the peer
                    let peer = PeerInfo {
                        peer_addr: mac_address_array,
                        ..Default::default()
                    };
                    espnow.add_peer(peer).unwrap();
                }

                println!("Peer added: {:?}", mac_address_array);
            })?;
        }

        // Create a thread for broadcasting ping messages
        let espnow_clone = self.espnow.clone();

        Builder::new()
            .name("sync_master".into())
            .spawn(move || {
                let task = async move {
                    loop {
                        log::info!("Broadcasting ping message");
                        if let Ok(mut espnow) = espnow_clone.lock() {
                            if let Err(e) = espnow.send(BROADCAST, &[0x01]) {
                                log::warn!("Failed to send broadcast ping message: {:?}", e);
                            }
                        }
                        driver::delay_ms(2000).await;
                    }
                };
                esp_idf_svc::hal::task::block_on(task);
            })
            .expect("failed to spawn thread");

        Ok(())
    }
}

// main board mac : c8:f0:9e:52:f3:e0
pub const MASTER_MAC: [u8; 6] = [0xc8, 0xf0, 0x9e, 0x52, 0xf3, 0xe0];

impl Sync for EspSync {}
