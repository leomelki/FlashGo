use anyhow::Result;
use uuid::Uuid;

pub trait Characteristic: UUIDAble {
    fn set_callback(&mut self, callback: impl Fn(&[u8]) -> Result<()> + Send + Sync + 'static);
    fn send_value(&mut self, value: &'static [u8]);
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

pub fn get_uuid(element: &impl UUIDAble) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_X500, element.get_name().as_bytes())
}

pub fn get_uuid_from_name(name: &str) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_X500, name.as_bytes())
}
