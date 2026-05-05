//! Contains the definition of a system. 

use crate::{ComponentId};
use crate::engine::world::*;
use crate::resource::Resources;

// This can probably be done without boilerplate : )
#[macro_export]
macro_rules! components_read {
    ($( $t:ty ), *) => {
        fn components_read(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

#[macro_export]
macro_rules! components_write {
    ($( $t:ty ), *) => {
        fn components_write(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

#[macro_export]
macro_rules! components_with {
    ($( $t:ty ), *) => {
        fn components_with(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

#[macro_export]
macro_rules! components_without {
    ($( $t:ty ), *) => {
        fn components_without(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

pub trait System {
    // with this implementation we will have to trust the user doesn't fetch other components
    // than what's specified
    fn components_read(&self) -> Vec<ComponentId>;
    fn components_write(&self) -> Vec<ComponentId>;
    fn components_with(&self) -> Vec<ComponentId>;
    fn components_without(&self) -> Vec<ComponentId>;

    fn run(&mut self, world: &mut World, resources: &mut Resources);
}
