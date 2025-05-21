use crate::{
    driver,
    drivers::leds::Color,
    leds::{animations::Animation, controller::LedsController},
    protos::animations_::list_::bpm_color_,
    sync::DevicesSyncerState,
};

impl Animation for bpm_color_::BPMColorAnimation {
    fn tick(&self, state: &DevicesSyncerState, leds: &mut LedsController) {
        let now = state.now_millis();
        let beat_duration = state.bpm_to_ms();
        let beat_number = now / beat_duration;

        let rgb = driver::random_u32_seeded(beat_number * 100 * beat_duration); // multiplication to offset the random number from other animations
        let r = (rgb & 0xFF) as u8;
        let g = ((rgb >> 8) & 0xFF) as u8;
        let b = ((rgb >> 16) & 0xFF) as u8;
        let new_color = Color {
            red: r,
            green: g,
            blue: b,
        };
        leds.set_all_colors(new_color);
    }
}
