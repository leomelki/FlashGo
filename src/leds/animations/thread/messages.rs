use crate::animations::protos::set_animation::{AnimationType, SetAnimation};

pub enum Message {
    Init(u8),
    SetAnimation(SetAnimation),
}
