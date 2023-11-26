use std::collections::HashMap;
use actix::prelude::*;
use crate::device::Device;
use crate::api::objects::APIDevice;
use itertools::Itertools;
use crate::device::StartStopModules;


#[derive(Message)]
#[rtype(result = "()")]
pub struct UpdateRunningDevices(pub Vec<APIDevice>);

pub struct DeviceHandler {
    devices: HashMap<usize, Addr<Device>>,

}

impl DeviceHandler {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new()
        }
    }
}

impl Actor for DeviceHandler {
    type Context = Context<Self>;
}

impl Handler<UpdateRunningDevices> for DeviceHandler {
    type Result = ();

    fn handle(&mut self, msg: UpdateRunningDevices, ctx: &mut Self::Context) -> Self::Result {
        // Start new devices, update old ones!
        for c_device_api in msg.0.into_iter() {
            let c_device_result = self.devices.get_mut(&c_device_api.id);
            let update_device = StartStopModules(c_device_api.modules);
            match c_device_result {
                None => {
                    // device does not exist and has to be created!
                    let c_device = Device::new(c_device_api.id, c_device_api.name).start();
                    c_device.do_send((update_device));
                    self.devices.insert(c_device_api.id, c_device);
                }
                Some(c_device) => {
                    // device already exists, updating modules!
                    c_device.do_send((update_device));
                }
            }
        }
        // TODO: clean devices that have been stopped!
    }
}