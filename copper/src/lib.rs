
use std::any::TypeId;
pub mod core;
pub mod ecs;
pub mod renderer;
pub mod resource;
pub mod input;
pub mod grid;
pub mod assets;


pub type ComponentId = TypeId;
pub type EntityId = usize;

pub trait Component: 'static {
    fn name(&self) -> &str;
}
