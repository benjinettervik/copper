//! Welcome to Copper!

use std::any::TypeId;

pub mod engine;
pub mod renderer;
pub mod resource;
pub mod input;


pub type ComponentId = TypeId;
pub type EntityId = usize;

/// Use #[derive(Component)] to implement this trait onto custom components.
pub trait Component: 'static {
    fn name(&self) -> &str;
}
