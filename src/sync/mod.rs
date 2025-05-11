use micropb::MessageDecode;
use micropb::MessageEncode;
use micropb::PbDecoder;
use micropb::PbEncoder;

use crate::drivers::driver;
use crate::drivers::driver::Instant;
use crate::drivers::sync::SyncTrait;
use crate::protos;
use crate::protos::animations_::list_::rainbow_::RainbowAnimation;
use crate::protos::animations_::SetAnimation;
use crate::protos::animations_::SetAnimation_::Animation;
use crate::protos::sync_::ack_::Ack;
use crate::protos::sync_::sync_::Sync;
use crate::protos::sync_::Packet;
use futures::channel::mpsc::{self, Receiver};
use futures::StreamExt;
use std::sync::Arc;
use std::sync::Mutex;

enum SyncMessage {
    ReceivedData(Vec<u8>),
}

pub struct DevicesSyncer<T: SyncTrait + 'static> {
    state: Arc<Mutex<DevicesSyncerState>>,
    sync: Box<T>,
    device_id: u32,
    delay: Mutex<u64>,
}

struct DevicesSyncerState {
    time: u64,
    animation: Animation,
}

fn now_micros() -> u64 {
    Instant::now().elapsed().as_micros() as u64
}

impl<T: SyncTrait + 'static> DevicesSyncer<T> {
    pub fn new(sync: Box<T>) -> Self {
        // Create a futures-based channel with a buffer size of 32
        let device_id = Instant::now().elapsed().as_nanos() as u32;

        Self {
            state: Arc::new(Mutex::new(DevicesSyncerState {
                time: 0,
                animation: Animation::RainbowAnimation(RainbowAnimation {
                    speed: 1.0,
                    progressive: true,
                }),
            })),
            sync,
            device_id,
            delay: Mutex::new(0),
        }
    }

    pub async fn init(&'static self) {
        let (sender, receiver) = mpsc::channel::<SyncMessage>(32);

        let sender_for_callback = sender.clone();

        self.sync.init({
            move |_mac_address, data| {
                let mut sender = sender_for_callback.clone();
                let data_vec = data.to_vec();

                if let Err(e) = sender.try_send(SyncMessage::ReceivedData(data_vec)) {
                    log::warn!("Failed to send received data: {:?}", e);
                }
            }
        });

        let message_handler = self.spawn_message_handler(receiver);

        if driver::is_master() {
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
                let sync_packet = protos::sync_::Packet_::Packet::Sync(Sync {
                    set_animation: SetAnimation {
                        animation: Some(Animation::RainbowAnimation(RainbowAnimation {
                            speed: 1.0,
                            progressive: true,
                        })),
                    },
                    timestamp: now_micros(),
                    ..Default::default()
                });

                self.send_packet(sync_packet);
                driver::delay_ms(10).await;
            }
        })
        .await;
    }

    async fn init_slave(&'static self) {
        driver::run_async(async move {
            loop {
                self.send_packet(protos::sync_::Packet_::Packet::Sync(Sync {
                    timestamp: now_micros(),
                    ..Default::default()
                }));
                driver::delay_ms(700).await;
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

    fn send_packet(&self, packet: protos::sync_::Packet_::Packet) {
        let data = self.encode_packet(packet);
        self.sync.send(&data);
    }

    fn handle_packet(&self, data: &[u8]) {
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
                protos::sync_::Packet_::Packet::Ack(ack) => {
                    self.handle_ack(ack);
                }
            }
        }
    }

    fn handle_sync(&self, sync: protos::sync_::sync_::Sync) {
        // First thing: we respond
        self.send_packet(protos::sync_::Packet_::Packet::Ack(Ack {
            rcv_timestamp: sync.timestamp,
            device_id: self.device_id,
        }));

        // If we are the master, we don't need to update our state
        // We use the same sync packet so that we have a natural delay
        if driver::is_master() {
            return;
        }

        // Then we update our state
        let mut state_mut = self.state.lock().unwrap();
        state_mut.time = sync.timestamp + *self.delay.lock().unwrap();
        if let Some(animation) = sync.set_animation.animation {
            state_mut.animation = animation;
        }
    }

    fn handle_ack(&self, ack: protos::sync_::ack_::Ack) {
        let delay = now_micros() - ack.rcv_timestamp;
        // is less than 1 second ( not a bug )
        if delay < 1000000 {
            if !driver::is_master() {
                let mut delay_mut = self.delay.lock().unwrap();
                *delay_mut = delay;
            }
            log::info!("Received ack delay: {:?}", delay);
        }
    }
}
