use std::time::Duration;
use crossbeam_channel::unbounded;
use generic_runtime::*;
use generic_runtime::executor::Executor;
use generic_runtime::handler::Handler;
use generic_runtime::message::OutgoingMessage;
use generic_runtime::module_runner::ModuleRunner;
use log::info;
use rand::Rng;

struct Message {
    name: String,
    data: u32
}

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
    fn execute(&self) -> Message {
        let mut rng = rand::thread_rng();
        Message {
            data: rng.gen_range(0..10),
            name: self.name.clone()
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // enable logging of async
    let (sender, receiver) = unbounded::<OutgoingMessage<Message>>();
    let mut handler = Handler::new::<>();
    let worker = Worker::new(String::from("Worker!"));
    let runner = ModuleRunner::new(Box::new(worker), Duration::from_secs(3), sender);
    handler.spawn(0, runner);

    receiver.iter().for_each(|message| {
        match message {
            OutgoingMessage::CollectedData(data) => {
                info!("I am {}. The following Number has been generated: {}", data.name, data.data);
            }
        }
    })
}
