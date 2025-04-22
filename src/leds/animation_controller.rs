use crate::{
    drivers::leds::{Color, Leds},
    leds::leds_controller::LedsController,
};

use super::{animations::animation::Animation, thread::messages::Message}; 

pub struct AnimationController<L> {
    current: Option<Box<dyn Animation>>,
    leds_controller: LedsController<L>,
}

impl<L: Leds> AnimationController<L> {
    pub fn new(leds: L) -> Self {
        Self {
            current: None,
            leds_controller: LedsController::new(leds).unwrap(),
        }
    }

    pub fn tick(&mut self) {
        if let Some(animation) = &mut self.current {
            animation.tick();
        }
    }
    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::Init(animation_id) => {
                log::info!("AnimationController inited: {}", animation_id);
                self.leds_controller.set_color(0, 0, Color::green());
                self.leds_controller.update().unwrap();
            }
        }
    }

    pub fn stop(&mut self) {
        self.current = None;
    }
}
