use crate::{
    drivers::leds::{Color, Leds},
    leds::{
        animations::{self, config::AnimationConfig},
        controller::LedsController,
    },
};

use super::{state::AnimationState, thread::messages::Message, Animation};

pub struct AnimationController<L> {
    current: Option<Box<dyn Animation>>,
    leds_controller: LedsController,
    leds: L,
}

impl<L: Leds> AnimationController<L> {
    pub fn new(leds: L) -> Self {
        Self {
            current: None,
            leds_controller: LedsController::new().unwrap(),
            leds,
        }
    }

    pub fn tick(&mut self) {
        let mut state = AnimationState::new();
        state.update();
        if let Some(animation) = &mut self.current {
            animation.tick(&state, &mut self.leds_controller);
        }
        self.leds_controller.update(&mut self.leds).unwrap();
    }
    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::Init(animation_id) => {
                log::info!("AnimationController inited: {}", animation_id);
                self.leds_controller.set_color(0, 0, Color::green());
                self.leds_controller.update(&mut self.leds).unwrap();
            }
            Message::SetAnimation(anim_type) => {
                log::info!("AnimationController set animation: {:?}", anim_type);
                self.set_animation(animations::get_animation(anim_type).unwrap()(
                    &AnimationConfig::new(),
                ));
            }
        }
    }

    pub fn set_animation(&mut self, animation: Box<dyn Animation>) {
        self.current = Some(animation);
    }

    pub fn stop(&mut self) {
        self.current = None;
    }
}
