mod anim_rainbow;
pub mod configs;
pub mod controller;
pub mod state;
pub mod thread;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AnimationType {
    Rainbow,
}

use std::{collections::HashMap, sync::LazyLock};

use anim_rainbow::RainbowAnimation;
use configs::rainbow_config::RainbowAnimationConfig;
use state::AnimationState;

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

type AnimationFactory = fn() -> Box<dyn DynAnimation>;

lazy_static::lazy_static! {
    static ref ANIMATION_REGISTRY: HashMap<AnimationType, AnimationFactory> = {
        let mut registry: HashMap<AnimationType, AnimationFactory> = HashMap::new();
        registry.insert(AnimationType::Rainbow, || {
            Box::new(RainbowAnimation::new(&RainbowAnimationConfig {
                speed: 100.0,
                progressive: true,
            }))
        });
        registry
    };
}

pub fn get_animation(anim_type: AnimationType) -> Option<&'static AnimationFactory> {
    ANIMATION_REGISTRY.get(&anim_type)
}
