use bbqueue::{Consumer, Producer};

use super::{
    super::animation_controller::AnimationController,
    communicator::{consume, send, BB, QUEUE_SIZE},
    messages::Message,
};
use crate::drivers::{driver, leds::Leds};

pub struct AnimationThread {
    prod: Producer<'static, QUEUE_SIZE>,
}

impl AnimationThread {
    // Initialize the thread and start it

    pub fn send(&mut self, packet: Message) {
        send(&mut self.prod, packet);
    }

    pub fn init(leds: impl Leds + 'static) -> Self {
        let (prod, cons) = BB.try_split().unwrap();

        #[cfg(feature = "esp")]
        return Self::init_esp(prod, cons, leds);

        #[cfg(feature = "wasm")]
        return Self::init_wasm(prod, cons, leds);
    }

    #[cfg(feature = "esp")]
    fn init_esp(
        prod: Producer<'static, QUEUE_SIZE>,
        cons: Consumer<'static, QUEUE_SIZE>,
        leds: impl Leds + 'static,
    ) -> Self {
        // Run the task in a separate thread without blocking
        std::thread::spawn(move || {
            let animation_task = async {
                run_loop(cons, leds).await;
            };

            esp_idf_svc::hal::task::block_on(animation_task);
        });
        Self { prod }
    }

    #[cfg(feature = "wasm")]
    fn init_wasm(
        prod: Producer<'static, QUEUE_SIZE>,
        cons: Consumer<'static, QUEUE_SIZE>,
        leds: impl Leds + 'static,
    ) -> Self {
        let animation_task = async move {
            run_loop(cons, leds).await;
        };
        wasm_bindgen_futures::spawn_local(animation_task);

        Self { prod }
    }
}

async fn run_loop(cons: Consumer<'static, QUEUE_SIZE>, leds: impl Leds) {
    let mut controller = AnimationController::new(leds);
    let mut cons = cons;
    loop {
        //Handle all messages waiting in the queue
        while let Some(message) = consume(&mut cons) {
            controller.handle_message(message);
        }
        controller.tick();
        driver::delay_ms(1).await;
    }
}
