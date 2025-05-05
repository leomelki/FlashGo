mod anim_rainbow;
pub mod controller;
pub mod state;
pub mod thread;

use anim_rainbow::RainbowAnimation;
use state::AnimationState;

use crate::protos::animations_::{SetAnimation, SetAnimation_};

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
    if let Some(animation) = set_animation.animation {
        match animation {
            SetAnimation_::Animation::RainbowAnimation(animation) => {
                return Some(Box::new(RainbowAnimation::new(&animation)));
            }
        }
    } else {
        None
    }
}
