mod anim_rainbow;
pub mod controller;
pub mod state;
pub mod thread;

use anim_rainbow::RainbowAnimation;
use state::AnimationState;

use crate::protos::animations_::{AnimationType, SetAnimation, SetAnimation_};

use super::controller::LedsController;

pub trait Animation {
    type Config;
    fn tick(&self, state: &AnimationState, leds: &mut LedsController);
    fn new(config: &Self::Config) -> Self
    where
        Self: Sized;
}

pub trait DynAnimation {
    fn tick(&self, state: &AnimationState, leds: &mut LedsController);
}

impl<T: Animation> DynAnimation for T {
    fn tick(&self, state: &AnimationState, leds: &mut LedsController) {
        Animation::tick(self, state, leds)
    }
}

pub fn get_animation(set_animation: SetAnimation) -> Option<Box<dyn DynAnimation>> {
    if let Some(config) = set_animation.config {
        match config {
            SetAnimation_::Config::RainbowConfig(config) => {
                return Some(Box::new(RainbowAnimation::new(&config)));
            }
        }
    } else {
        None
    }
}
