use anyhow::Result;

use crate::{
    drivers::leds::{Color, Leds},
    leds::controller::LedsController,
    protos::animations_::{
        list_::rainbow_::RainbowAnimation,
        SetAnimation_::{self},
    },
};

use super::{state::AnimationState, thread::messages::Message};
use crate::leds::animations::Animation;

pub struct AnimationController<L> {
    current: SetAnimation_::Animation,
    leds_controller: LedsController,
    leds: L,
}

impl<L: Leds> AnimationController<L> {
    pub fn new(leds: L) -> Self {
        Self {
            current: SetAnimation_::Animation::RainbowAnimation(RainbowAnimation {
                ..Default::default()
            }),
            leds_controller: LedsController::new().unwrap(),
            leds,
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let mut state = AnimationState::new();
        state.update();

        self.tick_animation(&state);
        self.leds_controller.update(&mut self.leds)?;
        Ok(())
    }

    fn tick_animation(&mut self, state: &AnimationState) {
        match &mut self.current {
            SetAnimation_::Animation::RainbowAnimation(rainbow) => {
                rainbow.tick(state, &mut self.leds_controller)
            }
            SetAnimation_::Animation::StrobeAnimation(strobe) => {
                strobe.tick(state, &mut self.leds_controller)
            }
            SetAnimation_::Animation::RandomBlinkAnimation(random_blink) => {
                // random_blink.tick(state, &mut self.leds_controller)
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
                self.current = state.animation;
            }
        }
    }
}
