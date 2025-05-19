use std::{
    fmt::Debug,
    sync::{Arc, Mutex, OnceLock},
};

use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use log::info;

static GLOBAL_STATE: OnceLock<Arc<GlobalState>> = OnceLock::new();

/// Global state of the program.
/// Initialized once at the beginning.
/// Can be accessed from any part of the program.
pub struct GlobalState {
    pub(crate) nvs_connect_configs_ns: Mutex<EspNvs<NvsDefault>>,
}

impl Debug for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalState").finish()
    }
}

impl GlobalState {
    /// Initialize the global state with default values.
    /// Must be initialized only once.
    pub fn init(nvs_partition: EspDefaultNvsPartition) {
        // NVS config
        let namespace = "Connect configs";
        let nvs = match EspNvs::new(nvs_partition, namespace, true) {
            Ok(nvs) => {
                info!("Got namespace {:?} from default partition", namespace);
                nvs
            }
            Err(e) => panic!("Could't get namespace {:?}", e),
        };

        let gs = GlobalState {
            nvs_connect_configs_ns: Mutex::new(nvs),
        };
        GLOBAL_STATE
            .set(Arc::new(gs))
            .expect("Global state already initialized");
    }

    /// Get the global state.
    pub fn get() -> Arc<GlobalState> {
        GLOBAL_STATE.get().unwrap().clone()
    }
}
