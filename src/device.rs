use std::collections::HashMap;
use std::ops::Add;
use actix::prelude::*;
use crate::modules::{GatherData, Module, ModuleProxy, ModuleType};
use crate::modules::dummy::Dummy;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartStopModules(pub Vec<ModuleType>);

pub struct Device {
    // TODO: save Addr instead of ModuleProxy!!!!
    running_modules: HashMap<ModuleType, Box<dyn ModuleProxy>>,
    name: String,
    id: usize
}

impl Device {
    pub fn new(id: usize, name: String) -> Self {
        Device {
            running_modules: HashMap::new(),
            name,
            id
        }
    }
}

impl Actor for Device {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Device {} successfully started!", self.name);
    }
}

impl Handler<StartStopModules> for Device {
    type Result = ();

    fn handle(&mut self, msg: StartStopModules, ctx: &mut Self::Context) -> Self::Result {
        println!("Updating running modules of {}", self.name);
        // Start new modules; update old modules
        for c_module_that_should_run in msg.0.iter() {
            let c_running_modules_result = self.running_modules.get_mut(&c_module_that_should_run);
            match c_running_modules_result {
                None => {
                    // Add new Module!
                    match c_module_that_should_run {
                        ModuleType::Dummy => {
                            let mut dummy = Box::new(Dummy::new());
                            let address = dummy.clone().start();
                            // TODO: remove this workaround, not very clean!
                            dummy.address = Some(address);
                            //dummy.address.clone().unwrap().do_send(GatherData());
                            self.running_modules.insert(*c_module_that_should_run, dummy);
                        }
                    };

                }
                Some(_) => {
                    // Module already exists!
                    // TODO: update configuration when implmented later on!
                }
            }
        }
        // TODO: clean modules that have been stopped!
    }
}