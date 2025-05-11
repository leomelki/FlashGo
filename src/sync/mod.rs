use crate::drivers::driver;
use crate::drivers::sync::SyncTrait;
use std::sync::Arc;
use std::sync::Mutex;

pub struct DevicesSyncer {
    state: Arc<Mutex<DevicesSyncerState>>,
}

struct DevicesSyncerState {
    time: i64,
}

impl DevicesSyncer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(DevicesSyncerState { time: 0 })),
        }
    }

    pub async fn init(&self, sync: impl SyncTrait + 'static) {
        let state = self.state.clone();
        sync.init({
            move |mac_address, data| {
                let mut state_mut = state.lock().unwrap();
                state_mut.time += 1;
                println!(
                    "Received data from {:?}: {:?} / time: {}",
                    mac_address, data, state_mut.time
                );
            }
        });

        driver::run_async(async move {
            loop {
                log::info!("Sending data");
                sync.send(b"Hello");
                driver::delay_ms(1000).await;
            }
        })
        .await;
    }
}
