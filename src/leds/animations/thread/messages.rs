use crate::sync::DevicesSyncerState;

pub enum Message {
    Init(u8),
    SetState(DevicesSyncerState),
}
