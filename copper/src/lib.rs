use std::any::TypeId;

pub mod engine;

#[allow(unused)]
type ComponentId = TypeId; 

pub trait Component: 'static {
    fn name(&self) -> &str;
}