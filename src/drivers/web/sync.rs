use super::super::sync::SyncTrait;
use anyhow::Result;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

pub struct WebSync {}

#[wasm_bindgen]
extern "C" {
    fn init_espnow();
    fn send_espnow(data: &[u8]);
}

#[wasm_bindgen]
pub fn receive_espnow(channel: &[u8], data: &[u8]) {
    CALLBACK.with(|cb| {
        if let Some(callback) = &*cb.borrow() {
            callback(channel, data);
        }
    });
}

thread_local! {
    static CALLBACK: RefCell<Option<Box<dyn Fn(&[u8], &[u8]) + Send + Sync + 'static>>> = RefCell::new(None);
}

impl WebSync {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl SyncTrait for WebSync {
    fn init(&self, callback: impl Fn(&[u8], &[u8]) + Send + Sync + 'static) {
        CALLBACK.with(|cb| {
            *cb.borrow_mut() = Some(Box::new(callback));
        });
        init_espnow();
    }

    fn send(&self, data: &[u8]) {
        send_espnow(data);
    }
}
