use crate::engine::world::*;
use std::any::TypeId;
type Entity = usize;

#[macro_export]
macro_rules! query_for_components {
    ($( $t:ty ), *) => {
        fn get_component_types(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

pub trait System {
    fn get_component_types(&self) -> Vec<TypeId>;

    fn _on_ready(&self, world: &mut World, entites: Vec<Entity>);

    fn _process(&self, world: &mut World, entites: Vec<Entity>);

    fn _delta_process(&self, world: &mut World, entites: Vec<Entity>);
}
