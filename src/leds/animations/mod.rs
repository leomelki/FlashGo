mod anim_rainbow;
mod anim_random_blink;
mod anim_strobe;
mod anim_wave;
pub mod controller;
pub mod state;
pub mod thread;

use crate::sync::DevicesSyncerState;

use super::controller::LedsController;

pub trait Animation {
    fn tick(&self, state: &DevicesSyncerState, leds: &mut LedsController);
}
