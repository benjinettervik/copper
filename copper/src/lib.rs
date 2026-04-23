use std::any::TypeId;

pub mod engine;
pub mod renderer;
pub mod resource;

#[allow(unused)]
type ComponentId = TypeId; 

pub trait Component: 'static {
    fn name(&self) -> &str;
}

