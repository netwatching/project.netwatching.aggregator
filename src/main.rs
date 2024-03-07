use std::time::Duration;
use crossbeam_channel::unbounded;
use generic_runtime::executor::Executor;
use generic_runtime::handler::Handler;
use generic_runtime::module_runner::ModuleRunner;
use log::{info, warn};
use rand::Rng;

#[derive(Debug, Clone)]
struct Message {
    name: String,
    data: u32
}

#[derive(Debug, Clone)]
struct Worker {
    name: String
}

impl Worker {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

impl Executor<Message> for Worker{
    fn execute(&self) -> Option<Message> {
        let mut rng = rand::thread_rng();
        Some(Message {
            data: rng.gen_range(0..1000),
            name: self.name.clone()
        })
    }

    fn on_stop(&mut self) {
        warn!("Graceful shutdown of {}", self.name)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // enable logging of async
    let (sender, receiver) = unbounded::<Message>();
    let mut handler = Handler::new::<>();
    for i in 0..10000 {
        // Spawn 1000 runners because why not :D
        let worker = Worker::new(format!("Worker {:03}", i));
        let duration = if i % 2 == 0 {
            // Some of them should use a different delay
            Duration::from_secs(3)
        } else {
            Duration::from_secs(5)
        };
        let runner = ModuleRunner::new(Box::new(worker), duration, sender.clone());
        handler.spawn(i, runner);
    }

    let mut i = 0;
    receiver.iter().for_each(|message| {
        info!("I am {}. The following Number has been generated: {:03}", message.name, message.data);
        if i % 100 == 0 {
            // Kill next worker
            handler.send_stop_message(i/100);
        }
        i += 1;
    })
}
