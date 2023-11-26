use std::time::Duration;
use actix::{Actor, Addr, AsyncContext, Context, Handler};
use crate::modules::{GatherData, Module};

#[derive(Clone)]
pub struct Dummy {
    pub address: Option<Addr<Dummy>>
}

impl Module for Dummy {
    fn new() -> Self {
        Self {
            address: None
        }
    }
}

impl Actor for Dummy {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started dummy module!");
        ctx.run_interval(Duration::from_secs(5),  |act, ctx| ctx.address().do_send(GatherData()));
    }
}

impl Handler<GatherData> for Dummy {
    type Result = ();

    fn handle(&mut self, msg: GatherData, ctx: &mut Self::Context) -> Self::Result {
        println!("Dummy data gathering data!");
    }
}