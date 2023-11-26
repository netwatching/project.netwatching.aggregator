use crate::modules::ModuleType;

#[derive(Debug, Clone)]
pub struct APIDevice {
    pub id: usize,
    pub name: String,
    pub modules: Vec<ModuleType>,
}