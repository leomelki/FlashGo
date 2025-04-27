use anyhow::Result;

pub trait EspNow {
    fn send_message(&self, message: &str) -> Result<()>;
}
