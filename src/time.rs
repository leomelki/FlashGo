use crate::drivers::driver;

lazy_static::lazy_static! {
    pub static ref START_TIME: driver::Instant = driver::Instant::now();
}

pub fn now_micros() -> u64 {
    START_TIME.elapsed().as_micros() as u64
}

pub fn now_millis() -> u64 {
    START_TIME.elapsed().as_millis() as u64
}
