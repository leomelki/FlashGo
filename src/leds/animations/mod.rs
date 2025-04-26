mod anim_rainbow;
pub mod config;
pub mod controller;
pub mod state;
pub mod thread;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AnimationType {
    Rainbow,
}

use std::{collections::HashMap, sync::LazyLock};

use anim_rainbow::RainbowAnimation;
use config::AnimationConfig;
use state::AnimationState;

use super::controller::LedsController;

pub trait Animation {
    fn tick(&self, state: &AnimationState, leds: &mut LedsController);
    fn new(config: &AnimationConfig) -> Self
    where
        Self: Sized;
}

type AnimationFactory = fn(config: &AnimationConfig) -> Box<dyn Animation>;

static ANIMATION_REGISTRY: LazyLock<HashMap<AnimationType, AnimationFactory>> =
    LazyLock::new(|| {
        let mut registry: HashMap<AnimationType, AnimationFactory> = HashMap::new();
        registry.insert(AnimationType::Rainbow, |config| {
            Box::new(RainbowAnimation::new(config))
        });
        registry
    });

pub fn get_animation(anim_type: AnimationType) -> Option<&'static AnimationFactory> {
    ANIMATION_REGISTRY.get(&anim_type)
}
