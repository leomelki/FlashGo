use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::drivers::{driver, esp::driver::WIFI_CHANNEL};

use super::super::sync::SyncTrait;
use anyhow::Result;
use esp_idf_svc::espnow::{EspNow, PeerInfo, BROADCAST};
use esp_idf_svc::wifi::WifiDriver;

pub struct EspSync {
    pub espnow: Arc<Mutex<EspNow<'static>>>,
    pub wifi: WifiDriver<'static>,
    pub peers: Arc<Mutex<HashMap<[u8; 6], Instant>>>,
}

impl EspSync {
    pub fn new(wifi: WifiDriver<'static>) -> Result<Self> {
        let espnow = EspNow::take().unwrap();
        let espnow = Arc::new(Mutex::new(espnow));
        let peers = Arc::new(Mutex::new(HashMap::new()));

        let esp_sync = Self {
            espnow: espnow.clone(),
            wifi,
            peers,
        };

        Ok(esp_sync)
    }

    fn init_slave(
        &self,
        mut recieve_callback: impl FnMut(&[u8], &[u8]) + Send + Sync + 'static,
    ) -> Result<()> {
        log::info!("Initializing slave");

        // Register callbacks and add peer
        {
            let espnow = self.espnow.lock().unwrap();

            // Add broadcast peer
            let broadcast = esp_idf_svc::sys::esp_now_peer_info {
                channel: WIFI_CHANNEL,
                ifidx: esp_idf_svc::sys::wifi_interface_t_WIFI_IF_STA,
                encrypt: false,
                peer_addr: BROADCAST,
                ..Default::default()
            };
            espnow.add_peer(broadcast).unwrap();

            // Register the receive callback to handle incoming messages
            espnow
                .register_recv_cb(move |mac_address, data| {
                    recieve_callback(mac_address, data);
                })
                .unwrap();

            // Register the send callback
            espnow
                .register_send_cb(|_mac_addres, status| {
                    // log::info!("Send status: {:?}", status);
                })
                .unwrap();

            // Add the master peer
            let peer = PeerInfo {
                peer_addr: MASTER_MAC,
                channel: WIFI_CHANNEL,
                ifidx: esp_idf_svc::sys::wifi_interface_t_WIFI_IF_STA,
                encrypt: false,
                ..Default::default()
            };
            espnow.add_peer(peer).unwrap();

            // Track the master peer
            let mut peers = self.peers.lock().unwrap();
            peers.insert(MASTER_MAC, Instant::now());

            log::info!("Peer added: {:?}", MASTER_MAC);

            espnow.send(MASTER_MAC, &[0x01]).unwrap();
            log::info!("Sent message to master");
        }

        Ok(())
    }

    fn init_master(
        &self,
        recieve_callback: impl Fn(&[u8], &[u8]) + Send + Sync + 'static,
    ) -> Result<()> {
        log::info!("Initializing master");

        // Set up ESP-NOW with broadcast peer and callback
        {
            let espnow = self.espnow.lock().unwrap();

            // Add broadcast peer
            let broadcast = esp_idf_svc::sys::esp_now_peer_info {
                channel: WIFI_CHANNEL,
                ifidx: esp_idf_svc::sys::wifi_interface_t_WIFI_IF_STA,
                encrypt: false,
                peer_addr: BROADCAST,
                ..Default::default()
            };
            espnow.add_peer(broadcast).unwrap();
            let espnow_clone = self.espnow.clone();
            let peers_clone = self.peers.clone();

            espnow.register_recv_cb(move |mac_address, data| {
                let mac_address_array = mac_address.try_into().unwrap();

                // Check if we should add this peer
                let should_add_peer = {
                    let mut peers = peers_clone.lock().unwrap();
                    let now = Instant::now();

                    match peers.get(&mac_address_array) {
                        Some(last_added) => {
                            // Only re-add if it's been more than 10 seconds
                            if now.duration_since(*last_added) > Duration::from_secs(10) {
                                peers.insert(mac_address_array, now);
                                true
                            } else {
                                false
                            }
                        }
                        None => {
                            // New peer, add it
                            peers.insert(mac_address_array, now);
                            true
                        }
                    }
                };

                // If we should add the peer, do so
                if should_add_peer {
                    let espnow = espnow_clone.lock().unwrap();
                    if let Ok(false) = espnow.peer_exists(mac_address_array) {
                        // Add the peer
                        let peer = PeerInfo {
                            peer_addr: mac_address_array,
                            channel: WIFI_CHANNEL,
                            ifidx: esp_idf_svc::sys::wifi_interface_t_WIFI_IF_STA,
                            encrypt: false,
                            ..Default::default()
                        };
                        espnow.add_peer(peer).unwrap();
                        log::info!("Peer added: {:?}", mac_address_array);
                    }
                }

                recieve_callback(mac_address, data);
            })?;
        }

        Ok(())
    }
}

// main board mac : c8:f0:9e:52:f3:e0
pub const MASTER_MAC: [u8; 6] = [0xc8, 0xf0, 0x9e, 0x52, 0xf3, 0xe0];

impl SyncTrait for EspSync {
    fn init(&self, recieve_callback: impl Fn(&[u8], &[u8]) + Send + Sync + 'static) {
        if driver::is_master() {
            self.init_master(recieve_callback).unwrap();
        } else {
            self.init_slave(recieve_callback).unwrap();
        }
    }
    fn broadcast(&self, data: &[u8]) {
        let espnow = self.espnow.lock().unwrap();
        espnow.send(BROADCAST, data).unwrap();
    }

    fn send_private(&self, mac_address: [u8; 6], data: &[u8]) {
        let espnow = self.espnow.lock().unwrap();
        espnow.send(mac_address, data).unwrap();
    }
}
