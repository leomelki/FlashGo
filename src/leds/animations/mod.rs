mod anim_rainbow;
mod anim_strobe;
pub mod controller;
pub mod state;
pub mod thread;

use state::AnimationState;

use super::controller::LedsController;

pub trait Animation {
    fn tick(&self, state: &AnimationState, leds: &mut LedsController);
}

