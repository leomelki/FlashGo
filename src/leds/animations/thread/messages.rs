use crate::leds::animations::AnimationType;

pub enum Message {
    Init(u8),
    SetAnimation(AnimationType),
}
