pub mod messages;

// AnimationThread implementation

use super::controller::AnimationController;
use crate::drivers::{driver, leds::Leds};
use anyhow::Result;
use messages::Message;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::Builder;

#[derive(Clone)]
pub struct AnimationThread {
    tx: Sender<Message>,
}

impl AnimationThread {
    // Initialize the thread and start it

    pub fn send(&self, packet: Message) -> Result<()> {
        self.tx.send(packet)?;
        Ok(())
    }

    pub fn init(leds: impl Leds + 'static) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        #[cfg(feature = "esp")]
        return Self::init_esp(tx, rx, leds);

        #[cfg(feature = "wasm")]
        return Self::init_wasm(tx, rx, leds);
    }

    #[cfg(feature = "esp")]
    fn init_esp(tx: Sender<Message>, rx: Receiver<Message>, leds: impl Leds + 'static) -> Self {
        // Run the task in a separate thread without blocking

        Builder::new()
            .name("animation_thread".into())
            // .stack_size(1000)
            .spawn(move || {
                let animation_task = async {
                    run_loop(rx, leds).await;
                };

                esp_idf_svc::hal::task::block_on(animation_task);
            })
            .expect("failed to spawn thread");
        Self { tx }
    }

    #[cfg(feature = "wasm")]
    fn init_wasm(tx: Sender<Message>, rx: Receiver<Message>, leds: impl Leds + 'static) -> Self {
        let animation_task = async move {
            run_loop(rx, leds).await;
        };
        wasm_bindgen_futures::spawn_local(animation_task);

        Self { tx }
    }
}

async fn run_loop(rx: Receiver<Message>, leds: impl Leds) {
    let mut controller = AnimationController::new(leds);
    loop {
        // Handle all messages waiting in the queue
        while let Ok(message) = rx.try_recv() {
            controller.handle_message(message);
        }
        controller.tick();
        driver::delay_ms(1).await;
    }
}
