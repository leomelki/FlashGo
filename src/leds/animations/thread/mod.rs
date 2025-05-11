pub mod messages;

// AnimationThread implementation

use super::controller::AnimationController;
use crate::drivers::{driver, leds::Leds};
use anyhow::Result;
use messages::Message;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone)]
pub struct AnimationThread {
    tx: Sender<Message>,
}

impl AnimationThread {
    // Initialize the thread and start it

    pub fn send(&self, packet: Message) -> Result<()> {
        self.tx.send(packet).unwrap();
        Ok(())
    }

    pub async fn init(leds: impl Leds + 'static) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        driver::run_async(async move {
            run_loop(rx, leds).await;
            Ok(())
        })
        .await;

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
