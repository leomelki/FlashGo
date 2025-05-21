use crate::drivers::ble::Service;
use crate::drivers::driver;
use crate::drivers::sync::SyncTrait;
use crate::protos::animations_::list_::rainbow_::RainbowAnimation;
use crate::protos::animations_::{SetAnimation, SetAnimation_};
use crate::protos::bpm_::SetBPM;
use crate::sync::PartialDeviceState;
use crate::{
    drivers::ble,
    leds::animations::thread::{messages::Message, AnimationThread},
    sync::DevicesSyncer,
};
use anyhow::Result;
use ble::Characteristic;
use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};
use std::vec::Vec;

pub struct AnimationsOrchestrator<S, T>
where
    S: Service,
    <S as Service>::Characteristic: Send + Sync + 'static,
    T: SyncTrait + Send + Sync + 'static,
{
    animation_characteristic: <S as Service>::Characteristic,
    bpm_characteristic: <S as Service>::Characteristic,
    animation_thread: AnimationThread,
    devices_syncer: DevicesSyncer<T>,
    master: bool,
}

impl<S, T> AnimationsOrchestrator<S, T>
where
    S: Service,
    <S as Service>::Characteristic: Send + Sync + 'static,
    T: SyncTrait + Send + Sync + 'static,
{
    pub fn new(
        mut ble_service: S,
        animation_thread: AnimationThread,
        devices_syncer: DevicesSyncer<T>,
    ) -> Result<Self> {
        let animation_characteristic =
            ble_service.register_characteristic("animation", true, true)?;

        let bpm_characteristic = ble_service.register_characteristic("bpm", true, true)?;

        let orchestrator = Self {
            animation_characteristic,
            bpm_characteristic,
            animation_thread,
            devices_syncer,
            master: driver::is_master(),
        };

        Ok(orchestrator)
    }

    pub async fn init(&'static self) -> Result<()> {
        self.devices_syncer.init().await;
        self.animation_thread.send(Message::Init(1)).unwrap();

        let animation_thread_clone = self.animation_thread.clone();
        self.devices_syncer.set_state_update_callback(move |state| {
            if animation_thread_clone
                .send(Message::SetState(state))
                .is_err()
            {
                log::error!("AnimationOrchestrator failed to send animation");
            }
        });

        self.animation_characteristic.set_callback(move |value| {
            let mut set_animation = SetAnimation::default();
            let mut decoder = PbDecoder::new(value);

            if let Err(e) = set_animation.decode(&mut decoder, value.len()) {
                log::error!("AnimationOrchestrator received invalid animation: {:?}", e);
                return Ok(());
            }

            if let Some(animation) = set_animation.animation {
                log::info!(
                    "AnimationOrchestrator received BLE animation: {:?}",
                    animation
                );

                self.devices_syncer
                    .partial_state_update(&PartialDeviceState {
                        animation: Some(animation),
                        ..Default::default()
                    });
            } else {
                log::error!("AnimationOrchestrator received invalid animation (no animation)");
            }
            Ok(())
        });

        self.bpm_characteristic.set_callback(move |value| {
            let mut set_bpm = SetBPM::default();
            let mut decoder = PbDecoder::new(value);
            set_bpm.decode(&mut decoder, value.len()).unwrap();

            log::info!("AnimationOrchestrator received BLE bpm: {:?}", set_bpm);
            self.devices_syncer
                .partial_state_update(&PartialDeviceState {
                    bpm: Some(set_bpm.bpm as u16),
                    ..Default::default()
                });
            Ok(())
        });
        if self.master {
            self.init_master_orchestrator().await;
        }

        Ok(())
    }

    async fn init_master_orchestrator(&'static self) {
        driver::run_async(async move {
            //todo master things
            Ok(())
        })
        .await;
    }
}
