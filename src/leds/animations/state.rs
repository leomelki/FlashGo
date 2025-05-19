use crate::time::START_TIME;

pub struct AnimationState {
    pub time_ms: u32,
    pub power: u8,
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
