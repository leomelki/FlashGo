pub trait SyncTrait {
    fn init(&self, recieve_callback: impl Fn(&[u8], &[u8]) + Send + Sync + 'static);
    fn send(&self, data: &[u8]);
}
