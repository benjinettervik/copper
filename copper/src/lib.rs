use std::any::TypeId;
pub mod assets;
pub mod core;
pub mod ecs;
pub mod grid;
pub mod input;
pub mod renderer;
pub mod resource;

pub type ComponentId = TypeId;
pub type EntityId = usize;

pub trait Component: 'static {
    fn name(&self) -> &str;
}
