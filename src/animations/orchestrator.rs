use crate::protos::animations_::SetAnimation;
use crate::{
    drivers::ble,
    leds::animations::thread::{messages::Message, AnimationThread},
};
use anyhow::Result;
use ble::Characteristic;
use micropb::{MessageDecode, PbDecoder};

pub struct AnimationsOrchestrator<S: ble::Service> {
    ble_service: S,
    animation_thread: AnimationThread,
}

impl<S: ble::Service> AnimationsOrchestrator<S> {
    pub fn new(mut ble_service: S, animation_thread: AnimationThread) -> Result<Self> {
        let animation_thread_clone = animation_thread.clone();

        let animation_characteristic =
            ble_service.register_characteristic("animation", true, true)?;
        animation_characteristic.set_callback(move |value| {
            let mut set_animation = SetAnimation::default();
            let mut decoder = PbDecoder::new(value);
            set_animation.decode(&mut decoder, value.len()).unwrap();

            animation_thread_clone.send(Message::SetAnimation(set_animation))?;
            Ok(())
        });
        Ok(Self {
            ble_service,
            animation_thread,
        })
    }

    pub fn set_animation(&self, animation: SetAnimation) -> Result<()> {
        self.animation_thread
            .send(Message::SetAnimation(animation))?;
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.animation_thread.send(Message::Stop)?;
        Ok(())
    }
}
