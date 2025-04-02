use super::driver::DriverError;

pub trait EspNow {
    fn send_message(&self, message: &str) -> Result<(), DriverError>;
}
