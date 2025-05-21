use crate::{
    drivers::{driver, leds::Color},
    leds::{animations::Animation, controller::LedsController},
    protos::animations_::list_::wave_,
    sync::DevicesSyncerState,
};

impl Animation for wave_::WaveAnimation {
    fn tick(&self, state: &DevicesSyncerState, leds: &mut LedsController) {
        let time_ms = if self.sync {
            state.now_millis()
        } else {
            // Add a large random offset once to desynchronize non-synced animations
            // Using device_id as seed for deterministic random offset per device
            let random_offset = driver::random_u32_seeded(state.now_millis() % 20_000) % 10000; // Max offset of 10 seconds
            state.now_millis().wrapping_add(random_offset as u64)
        };

        let wave_speed = self.speed as f32 / 1000.0; // Convert speed to a more usable range

        // Calculate intensity based on a sine wave
        // The sine wave oscillates between -1 and 1. We map this to 0-1.
        let sin_wave = f32::sin(time_ms as f32 * wave_speed * std::f32::consts::PI / 1000.0);
        let normalized_intensity = (sin_wave + 1.0) / 2.0; // Map to 0-1

        // Apply min and max intensity
        let intensity =
            self.min_intensity + (self.max_intensity - self.min_intensity) * normalized_intensity;

        let base_color = self.color.to_color(state);

        let r = (base_color.red as f32 * intensity) as u8;
        let g = (base_color.green as f32 * intensity) as u8;
        let b = (base_color.blue as f32 * intensity) as u8;

        leds.set_all_colors(Color {
            red: r,
            green: g,
            blue: b,
        });
    }
}
