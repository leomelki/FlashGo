pub mod driver;
pub mod espnow;
pub mod leds;
pub mod mic;

#[cfg(not(target_arch = "wasm32"))]
mod esp;
#[cfg(target_arch = "wasm32")]
mod web;
