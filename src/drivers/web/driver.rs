use anyhow::Result;
use wasm_bindgen::prelude::wasm_bindgen;

use super::super::ble::Server;
pub use super::ble::BLEServerSimImpl;
use super::leds::LedsSimImpl;
use super::mic::MicSimImpl;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn console_log(message: &str);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn start() -> Result<(), wasm_bindgen::JsValue> {
    crate::init()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        console_log(&format!("{}: {}", record.level(), record.args()));
    }

    fn flush(&self) {}
}

pub fn new() -> Result<(LedsSimImpl, MicSimImpl)> {
    //set logger to redirect to console
    log::set_max_level(log::LevelFilter::Debug);
    log::set_logger(&ConsoleLogger).unwrap();

    Ok((LedsSimImpl::new(), MicSimImpl::new()))
}

pub fn create_ble_server() -> BLEServerSimImpl {
    BLEServerSimImpl::new()
}

#[wasm_bindgen]
extern "C" {
    pub fn log_data_js(key: &str, value: f32);
}
