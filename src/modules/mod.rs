use std::ops::Add;
use std::pin::Pin;
use actix::prelude::*;
use crate::modules::dummy::Dummy;

pub mod dummy;

#[derive(Message)]
#[rtype(result = "()")]
pub struct GatherData();


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum ModuleType {
    Dummy
}

#[derive(Debug, Clone, Copy)]
pub struct ModuleData<T> {
    module_type: ModuleType,
    data: T
}

pub trait Module: Actor + Handler<GatherData> + Clone {
    fn new() -> Self;
}

// https://stackoverflow.com/questions/53805212/specifying-associated-type-in-trait-that-inherits-from-another-trait
// Maybe needed later on
pub trait ModuleProxy {

}

impl<T> ModuleProxy for T
    where
        T: Module,
{

}

// TODO: find a way to make this possibble!
/*pub fn create_module(module_type: ModuleType) -> Pin<Box<dyn Module>> {
    match module_type {
        ModuleType::Dummy => {
            Box::new(Dummy::new())
        }
    }

}*/