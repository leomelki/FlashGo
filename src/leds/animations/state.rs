use crate::drivers::driver;

pub struct AnimationState {
    pub time_ms: u32,
    pub power: u8,
}

lazy_static::lazy_static! {
    static ref START_TIME: driver::Instant = driver::Instant::now();
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            time_ms: 0,
            power: 0,
        }
    }
    pub fn update(&mut self) {
        self.time_ms = START_TIME.elapsed().as_millis() as u32;
    }
}
