use crate::{
    drivers::leds::Color,
    leds::{animations::Animation, controller::LedsController},
    protos::animations_::list_::strobe_,
};

use super::state::AnimationState;

impl Animation for strobe_::StrobeAnimation {
    fn tick(&self, state: &AnimationState, leds: &mut LedsController) {
        let cycle_time = self.on_ms + self.off_ms;
        let time_in_cycle = (state.time_ms as i32) % cycle_time;

        if time_in_cycle < self.on_ms {
            // On phase - show the color
            leds.set_all_colors(self.color.to_color());
        } else {
            // Off phase - show black (off)
            leds.set_all_colors(Color::black());
        }
    }
}
