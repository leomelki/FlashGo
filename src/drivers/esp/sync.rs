use super::super::sync::Sync;
use anyhow::Result;
use core::sync::atomic::Ordering;
use esp_idf_svc::espnow::{EspNow, PeerInfo, SendStatus};
use std::{sync::atomic::AtomicBool, time::Duration};

pub struct EspSync {
    pub espnow: EspNow<'static>,
}

static IS_SEARCHING_CHANNEL: AtomicBool = AtomicBool::new(true);

impl EspSync {
    pub fn new() -> Result<Self> {
        let espnow = EspNow::take()?;

        Ok(Self { espnow })
    }
}

impl Sync for EspSync {
    fn init_slave(&self) -> Result<()> {
        // Notification to search for a master board on other channels
        let (channel_search_sender, channel_search_notifier) = std::sync::mpsc::sync_channel(1);

        self.espnow.register_recv_cb(|mac_address, _data| {
            // Convert slice to array
            let mac_address_array = mac_address.try_into().unwrap();
            // If peer does not exist, add it
            if let Ok(false) = self.espnow.peer_exists(mac_address_array) {
                // Add the peer
                let peer = PeerInfo {
                    peer_addr: mac_address_array,
                    ..Default::default()
                };
                self.espnow.add_peer(peer).unwrap();
            }
        })?;

        // Register the send callback, this is used to detect if the master is not reachable
        let mut num_fail: usize = 0;
        self.espnow
            .register_send_cb(|_mac_addres, status| {
                // if a send fails for more than 10 times, start searching for the master board on other channels
                if let SendStatus::SUCCESS = status {
                    num_fail = 0;
                    IS_SEARCHING_CHANNEL.store(false, Ordering::Relaxed)
                } else {
                    num_fail = num_fail.checked_add(1).unwrap_or(0);
                }

                if num_fail > 10 {
                    IS_SEARCHING_CHANNEL.store(true, Ordering::Relaxed);
                    let _ = channel_search_sender.try_send(());
                }
            })
            .unwrap();

        // Scan for an evailable channel
        std::thread::spawn(move || {
            let mut channel = 6;
            loop {
                while IS_SEARCHING_CHANNEL.load(Ordering::Relaxed) {
                    set_channel(channel);
                    // channels 1, 6 and 11 are the most common channels
                    channel = (channel + 5) % 15;
                    std::thread::sleep(Duration::from_secs(5));
                }
                // channel found, wait untill a notification is received
                channel_search_notifier.recv().unwrap();
            }
        });
        Ok(())
    }
    fn init_master(&self) -> Result<()> {
        Ok(())
    }
}

pub fn set_channel(channel: u8) {
    unsafe {
        let second = esp_idf_svc::sys::wifi_second_chan_t_WIFI_SECOND_CHAN_NONE;
        esp_idf_svc::sys::esp_wifi_set_channel(channel, second);
    }
}
