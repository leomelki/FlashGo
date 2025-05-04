use anyhow::Result;

use crate::{
    drivers::ble,
    leds::animations::thread::{messages::Message, AnimationThread},
};
use ble::Characteristic;

pub struct AnimationsOrchestrator<S: ble::Service> {
    ble_service: S,
    animation_thread: AnimationThread,
}

impl<S: ble::Service> AnimationsOrchestrator<S> {
    pub fn new(mut ble_service: S, animation_thread: AnimationThread) -> Result<Self> {
        let animation_characteristic =
            ble_service.register_characteristic("animation", true, true)?;
        animation_characteristic.set_callback(move |value| {
            let animation_type = value[0];
            animation_thread.send(Message::SetAnimation(animation_type));
            Ok(())
        });
        Ok(Self {
            ble_service,
            animation_thread,
        })
    }
}
