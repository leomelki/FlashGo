pub trait SyncTrait {
    fn init(&self, recieve_callback: impl Fn(&[u8], &[u8]) + Send + Sync + 'static);
    fn broadcast(&self, data: &[u8]);
    fn send_private(&self, mac_address: [u8; 6], data: &[u8]);
}
