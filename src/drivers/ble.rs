use anyhow::Result;
use uuid::Uuid;

pub trait Characteristic: UUIDAble {
    fn set_callback(&self, callback: impl Fn(&[u8]) -> Result<()> + Send + Sync + 'static);
    fn send_value<'a>(&self, value: &'a [u8]);
}

pub trait Service: UUIDAble {
    type Characteristic: Characteristic;
    fn register_characteristic(
        &mut self,
        name: &str,
        is_read: bool,
        is_write: bool,
    ) -> Result<Self::Characteristic>;
}

pub trait Server {
    type Service: Service;
    fn new() -> Self;

    fn register_service(&mut self, name: &str) -> Result<Self::Service>;
    fn start_advertisement(&mut self);
}

pub trait UUIDAble {
    fn get_name(&self) -> &str;
}
