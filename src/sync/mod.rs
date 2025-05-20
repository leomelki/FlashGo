use micropb::MessageDecode;
use micropb::MessageEncode;
use micropb::PbDecoder;
use micropb::PbEncoder;

use crate::drivers::driver;
use crate::drivers::sync::SyncTrait;
use crate::protos;
use crate::protos::animations_::list_::rainbow_::RainbowAnimation;
use crate::protos::animations_::SetAnimation;
use crate::protos::animations_::SetAnimation_::Animation;
use crate::protos::sync_::sync_::Sync_;
use crate::protos::sync_::Packet;
use crate::time::now_micros;
use crate::time::now_millis;
use futures::channel::mpsc::{self, Receiver};
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

struct ReceivedData {
    mac_address: [u8; 6],
    data: Vec<u8>,
}

enum SyncMessage {
    ReceivedData(ReceivedData),
}

struct DeviceDelay {
    delay: u64,
    time_offset_ms: i128,
    last_update: u64,
}

pub struct DevicesSyncer<T: SyncTrait + 'static> {
    state: Arc<Mutex<DevicesSyncerState>>,
    sync: T,
    device_id: u32,
    master: bool,
    state_update_callback: Mutex<Option<Box<dyn Fn(DevicesSyncerState) + Send + Sync + 'static>>>,
    best_delay: Mutex<DeviceDelay>,
}

#[derive(Clone)]
pub struct DevicesSyncerState {
    pub time_offset_ms: i128,
    pub animation: Animation,
}

impl Default for DevicesSyncerState {
    fn default() -> Self {
        Self {
            time_offset_ms: 0,
            animation: Animation::RainbowAnimation(RainbowAnimation::default()),
        }
    }
}

impl DevicesSyncerState {
    pub fn now_micros(&self) -> u64 {
        (now_micros() as i128 + self.time_offset_ms * 1000) as u64
    }

    pub fn now_millis(&self) -> u64 {
        (now_millis() as i128 + self.time_offset_ms) as u64
    }
}

impl<T: SyncTrait + 'static> DevicesSyncer<T> {
    pub fn new(sync: T) -> Self {
        let device_id = driver::random_u32();

        Self {
            state: Arc::new(Mutex::new(DevicesSyncerState {
                time_offset_ms: 0,
                animation: Animation::RainbowAnimation(RainbowAnimation {
                    speed: 100.0,
                    progressive: true,
                }),
            })),
            sync,
            device_id,
            master: driver::is_master(),
            state_update_callback: Mutex::new(None),
            best_delay: Mutex::new(DeviceDelay {
                delay: u64::MAX,
                time_offset_ms: 0,
                last_update: 0,
            }),
        }
    }

    fn call_state_update_callback(&self, state: DevicesSyncerState) {
        if let Some(callback) = self.state_update_callback.lock().unwrap().as_ref() {
            callback(state);
        }
    }

    pub fn update_state(&self, state: &DevicesSyncerState) {
        let mut state_mut = self.state.lock().unwrap();
        state_mut.animation = state.animation.clone();
        self.call_state_update_callback(state_mut.clone());
    }

    pub fn set_state_update_callback(
        &self,
        callback: impl Fn(DevicesSyncerState) + Send + Sync + 'static,
    ) {
        *self.state_update_callback.lock().unwrap() = Some(Box::new(callback));
    }

    pub async fn init(&'static self) {
        let (sender, receiver) = mpsc::channel::<SyncMessage>(32);

        let sender_for_callback = sender.clone();

        self.sync.init({
            move |_mac_address, data| {
                let mut sender = sender_for_callback.clone();
                let data_vec = data.to_vec();

                if let Err(e) = sender.try_send(SyncMessage::ReceivedData(ReceivedData {
                    mac_address: _mac_address.try_into().unwrap(),
                    data: data_vec,
                })) {
                    log::warn!("Failed to send received data: {:?}", e);
                }
            }
        });

        let message_handler = self.spawn_message_handler(receiver);

        if self.master {
            self.init_master().await;
        } else {
            self.init_slave().await;
        }

        message_handler.await;
    }

    async fn spawn_message_handler(&'static self, mut receiver: Receiver<SyncMessage>) {
        driver::run_async(async move {
            while let Some(message) = receiver.next().await {
                match message {
                    SyncMessage::ReceivedData(data) => {
                        self.handle_packet(&data);
                    }
                }
            }
            Ok(())
        })
        .await
    }

    async fn init_master(&'static self) {
        driver::run_async(async move {
            loop {
                let sync_packet =
                    protos::sync_::Packet_::Packet::Sync(protos::sync_::sync_::Sync {
                        set_animation: SetAnimation {
                            animation: Some(self.state.lock().unwrap().animation.clone()),
                        },
                        _has: Sync_::_Hazzer::default().init_set_animation(),
                    });

                self.broadcast_packet(sync_packet);
                driver::delay_ms(50).await;
            }
        })
        .await;
    }

    async fn init_slave(&'static self) {
        driver::run_async(async move {
            loop {
                self.broadcast_packet(protos::sync_::Packet_::Packet::PingPong(
                    protos::sync_::ping_::PingPong {
                        slave_device_id: self.device_id,
                        slave_timestamp: now_millis(),
                        master_timestamp: 0,
                    },
                ));
                driver::delay_ms(350).await;
            }
        })
        .await;
    }

    fn encode_packet(&self, packet: protos::sync_::Packet_::Packet) -> Vec<u8> {
        let packet = Packet {
            packet: Some(packet),
        };
        let mut encoder = PbEncoder::new(Vec::new());
        packet.encode(&mut encoder).unwrap();
        encoder.into_writer()
    }

    fn broadcast_packet(&self, packet: protos::sync_::Packet_::Packet) {
        let data = self.encode_packet(packet);
        self.sync.broadcast(&data);
    }
    fn send_private_packet(&self, packet: protos::sync_::Packet_::Packet, mac_address: [u8; 6]) {
        let data = self.encode_packet(packet);
        self.sync.send_private(mac_address, &data);
    }

    fn handle_packet(&self, received_data: &ReceivedData) {
        let data = received_data.data.as_slice();
        if data.len() == 1 && data[0] == 0x01 {
            // ping packet
            return;
        }

        let mut packet = Packet::default();
        let mut decoder = PbDecoder::new(data);
        packet.decode(&mut decoder, data.len()).unwrap();
        if let Some(packet) = packet.packet {
            match packet {
                protos::sync_::Packet_::Packet::Sync(sync) => {
                    self.handle_sync(sync);
                }
                protos::sync_::Packet_::Packet::PingPong(ping_pong) => {
                    self.handle_ping_pong(ping_pong, received_data.mac_address);
                }
            }
        }
    }

    //TODO update seulement sur le plus petit delay des 10 dernieres secondes puis reset et recommencer

    fn handle_sync(&self, sync: protos::sync_::sync_::Sync) {
        // If we are the slave, we update our state
        let mut state_mut: std::sync::MutexGuard<'_, DevicesSyncerState> =
            self.state.lock().unwrap();

        if let Some(animation) = sync.set_animation.animation {
            state_mut.animation = animation;
        }

        if let Some(callback) = self.state_update_callback.lock().unwrap().as_ref() {
            callback(state_mut.clone());
        }
    }

    fn handle_ping_pong(&self, mut packet: protos::sync_::ping_::PingPong, mac_address: [u8; 6]) {
        // If we are the master, we send a pong packet
        if self.master {
            packet.master_timestamp = now_millis();
            self.send_private_packet(
                protos::sync_::Packet_::Packet::PingPong(packet),
                mac_address,
            );
        } else if packet.slave_device_id == self.device_id {
            let now_ms = now_millis();

            let mut state_mut = self.state.lock().unwrap();
            let mut best_delay = self.best_delay.lock().unwrap();

            let delay = (now_ms - packet.slave_timestamp) / 2;
            if delay < best_delay.delay {
                best_delay.delay = delay;

                let master_time_when_received = packet.master_timestamp + delay;
                best_delay.time_offset_ms = (master_time_when_received as i128) - (now_ms as i128);
            }

            if now_ms > best_delay.last_update + 15_000 {
                state_mut.time_offset_ms = best_delay.time_offset_ms;
                self.call_state_update_callback(state_mut.clone());

                log::info!(
                    "Time offset updated to {:?} ms (delay : {:?} ms)",
                    state_mut.time_offset_ms,
                    best_delay.delay
                );

                best_delay.last_update = now_ms;
                best_delay.delay = u64::MAX;
            }
        }
    }
}
