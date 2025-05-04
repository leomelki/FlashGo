use crate::{
    drivers::leds::Color,
    leds::{animations::Animation, controller::LedsController},
};

use super::state::AnimationState;

pub struct RainbowAnimation {
    pub speed: f32,
    pub progressive: bool,
}

//config
// speed: multiplier for the speed of the animation
// progressive: if true, the animation will be from left to right, otherwise it will be the whole square
impl Animation for RainbowAnimation {
    type Config = crate::protos::animations_::list_::RainbowAnimation;
    fn tick(&self, state: &AnimationState, leds: &mut LedsController) {
        if self.progressive {
            for i in 0..leds.width {
                for j in 0..leds.height {
                    let color = Color::from_hsv(
                        (state.time_ms as f32 / 1000.0 * self.speed + (i + j) as f32) % 360.0,
                        1.0,
                        1.0,
                    );
                    leds.set_color(i, j, color);
                }
            }
        } else {
            let color = Color::from_hsv(
                (state.time_ms as f32 / 1000.0 * self.speed) % 360.0,
                1.0,
                1.0,
            );
            leds.set_all_colors(color);
        }
    }

    fn new(config: &Self::Config) -> Self {
        Self {
            speed: config.speed,
            progressive: config.progressive,
        }
    }
}
