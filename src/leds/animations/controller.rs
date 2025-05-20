use anyhow::Result;

use crate::{
    drivers::leds::{Color, Leds},
    leds::controller::LedsController,
    protos::animations_::{
        list_::rainbow_::RainbowAnimation,
        SetAnimation_::{self},
    },
    sync::DevicesSyncerState,
};

use super::thread::messages::Message;
use crate::leds::animations::Animation;

pub struct AnimationController<L> {
    // current: SetAnimation_::Animation,
    leds_controller: LedsController,
    leds: L,
    state: DevicesSyncerState,
}

impl<L: Leds> AnimationController<L> {
    pub fn new(leds: L) -> Self {
        Self {
            leds_controller: LedsController::new().unwrap(),
            leds,
            state: DevicesSyncerState::default(),
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        self.tick_animation();
        self.leds_controller.update(&mut self.leds)?;
        Ok(())
    }

    fn tick_animation(&mut self) {
        match &self.state.animation {
            SetAnimation_::Animation::RainbowAnimation(rainbow) => {
                rainbow.tick(&self.state, &mut self.leds_controller)
            }
            SetAnimation_::Animation::StrobeAnimation(strobe) => {
                strobe.tick(&self.state, &mut self.leds_controller)
            }
            SetAnimation_::Animation::RandomBlinkAnimation(random_blink) => {
                random_blink.tick(&self.state, &mut self.leds_controller)
            }
            SetAnimation_::Animation::WaveAnimation(wave) => {
                // wave.tick(state, &mut self.leds_controller)
            }
        }
    }
    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::Init(animation_id) => {
                log::info!("AnimationController inited: {}", animation_id);
                self.leds_controller.set_color(0, 0, Color::green());
                self.leds_controller.update(&mut self.leds).unwrap();
            }
            Message::SetState(state) => {
                // log::info!("AnimationController set animation");
                self.state = state;
            }
        }
    }
}
