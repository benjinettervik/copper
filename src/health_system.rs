// use crate::engine::world::*;
// use crate::engine::*;
// use crate::{engine::system::*, query_for_components};
// use std::any::TypeId;

// struct Health {
//     current: i32,
//     max: i32,
// }

// struct Random {
//     rand: i32,
// }

// pub struct HealthSystem;
// impl System for HealthSystem {
//     query_for_components!(Health, Random);

//     fn _on_ready(&self, world: &mut World, _entities: Vec<Entity>) {
//     let entity = world.spawn();
//     world.add_component(
//         entity,
//         Health {
//             current: 50,
//             max: 100,
//         },
//     );
// }

// fn _process(&self, world: &mut World, _entities: Vec<Entity>) {
//     // TODO
// }

// fn _delta_process(&self, _world: &mut World, _entities: Vec<Entity>) {}
// }
