mod devicehandler;
mod device;
mod modules;
mod api;

use std::future::{Future, IntoFuture};
use actix::prelude::*;
use actix_rt::signal;
use crate::api::objects::APIDevice;
use crate::device::Device;
use crate::devicehandler::{DeviceHandler, UpdateRunningDevices};
use futures::future::join_all;
use crate::modules::ModuleType::Dummy;

#[actix_rt::main]
async fn main() {
    let device_handler = DeviceHandler::new().start();
    let running_devices = vec![APIDevice {modules: vec![Dummy], name: String::from("Test"), id: 5}];
    let mut handles = vec![];
    for i in 0..1 {
        handles.push(device_handler.send(UpdateRunningDevices(running_devices.clone())));
    }
    signal::ctrl_c().await;
}
