use crate::{
    drivers::{driver, leds::Color},
    leds::{animations::Animation, controller::LedsController},
    protos::animations_::list_::random_blink_,
    sync::DevicesSyncerState,
};

impl Animation for random_blink_::RandomBlinkAnimation {
    fn tick(&self, state: &DevicesSyncerState, leds: &mut LedsController) {
        let cycle_time = self.on_ms + self.off_ms;
        let off_sync_ms = driver::random_u32_seeded(state.now_millis() / 800) % cycle_time;
        let time_in_cycle = (state.now_millis() as u32 + off_sync_ms) % cycle_time;

        if time_in_cycle < self.on_ms {
            // On phase - show the color
            leds.set_all_colors(self.color.to_color(state));
        } else {
            // Off phase - show black (off)
            leds.set_all_colors(Color::black());
        }
    }
}
