use super::super::sync::Sync;
use anyhow::Result;
use wasm_bindgen::prelude::wasm_bindgen;

pub struct WebSync {}

#[wasm_bindgen]
extern "C" {
    fn init_web_sync_master_js();
    fn init_web_sync_slave_js();
}

impl WebSync {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl Sync for WebSync {
    fn init_slave(&self) -> Result<()> {
        init_web_sync_slave_js();
        Ok(())
    }

    fn init_master(&self) -> Result<()> {
        init_web_sync_master_js();
        Ok(())
    }
}
