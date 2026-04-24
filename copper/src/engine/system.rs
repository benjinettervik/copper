use crate::engine::world::*;
use crate::resource::Resources;
use std::any::TypeId;
// type Entity = usize;

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
    fn components_read(&self) -> Vec<TypeId>;
    fn components_write(&self) -> Vec<TypeId>;
    fn components_with(&self) -> Vec<TypeId>;
    fn components_without(&self) -> Vec<TypeId>;

    fn run(&mut self, world: &mut World, resources: &mut Resources);
}
