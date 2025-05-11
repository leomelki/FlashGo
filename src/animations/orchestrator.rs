use crate::drivers::ble::Service;
use crate::protos::animations_::list_::RainbowAnimation;
use crate::protos::animations_::{SetAnimation, SetAnimation_};
use crate::{
    drivers::ble,
    leds::animations::thread::{messages::Message, AnimationThread},
};
use anyhow::Result;
use ble::Characteristic;
use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};
use std::vec::Vec;

pub struct AnimationsOrchestrator<S: Service> {
    animation_characteristic: <S as Service>::Characteristic,
    animation_thread: AnimationThread,
}

impl<S: Service> AnimationsOrchestrator<S> {
    pub fn new(mut ble_service: S, animation_thread: AnimationThread) -> Result<Self> {
        let animation_characteristic =
            ble_service.register_characteristic("animation", true, true)?;

        let animation_thread_clone = animation_thread.clone();
        animation_characteristic.set_callback(move |value| {
            log::info!("AnimationOrchestrator received animation: {:?}", value);
            let mut set_animation = SetAnimation::default();
            let mut decoder = PbDecoder::new(value);
            set_animation.decode(&mut decoder, value.len()).unwrap();

            animation_thread_clone.send(Message::SetAnimation(set_animation))?;
            Ok(())
        });

        Ok(Self {
            animation_characteristic,
            animation_thread,
        })
    }

    pub fn init(&self) -> Result<()> {
        self.animation_thread.send(Message::Init(1)).unwrap();
        self.set_animation(SetAnimation_::Animation::RainbowAnimation(
            RainbowAnimation {
                speed: 1.0,
                progressive: true,
            },
        ))
        .unwrap();
        Ok(())
    }

    pub fn set_animation(&self, animation: SetAnimation_::Animation) -> Result<()> {
        let set_animation = SetAnimation {
            animation: Some(animation),
        };

        let mut encoder = PbEncoder::new(Vec::new());
        set_animation.encode(&mut encoder).unwrap();
        let data = encoder.into_writer();
        self.animation_characteristic.send_value(&data);

        self.animation_thread
            .send(Message::SetAnimation(set_animation))?;
        Ok(())
    }
}
