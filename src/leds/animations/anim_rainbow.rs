use crate::{
    drivers::leds::Color,
    leds::{animations::Animation, controller::LedsController},
    protos::animations_::list_::rainbow_,
    sync::DevicesSyncerState,
};

//config
// speed: multiplier for the speed of the animation
// progressive: if true, the animation will be from left to right, otherwise it will be the whole square
impl Animation for rainbow_::RainbowAnimation {
    fn tick(&self, state: &DevicesSyncerState, leds: &mut LedsController) {
        if self.progressive {
            for i in 0..leds.width {
                for j in 0..leds.height {
                    let hue = ((state.now_millis() as u32 + ((i + j) * 100) as u32) as f32
                        / self.speed
                        % 3600.0)
                        / 10.0; // Complete cycle every 3.6 seconds
                    let color = Color::from_hsv(hue, 1.0, 1.0);
                    leds.set_color(i, j, color);
                }
            }
        } else {
            let hue = (state.now_millis() % 3600) as f32 / 10.0; // Complete cycle every 3.6 seconds
            let color = Color::from_hsv(hue, 1.0, 1.0);
            leds.set_all_colors(color);
        }
    }
}
