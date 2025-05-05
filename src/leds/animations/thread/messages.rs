use crate::protos::animations_::SetAnimation;

pub enum Message {
    Init(u8),
    SetAnimation(SetAnimation),
    Stop,
}
