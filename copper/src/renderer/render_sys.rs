use crate::renderer::test_components_renderer::{Transform, MockSprite, TextureAsset};
use crate::engine::world::World;
use crate::engine::{Startup, Update, SystemRoutine};
use std::any::TypeId;
use crate::engine::system::System;

pub struct RenderSys;

impl System for RenderSys {
    fn get_component_types(&self) -> Vec<TypeId> {
        vec![
            TypeId::of::<MockSprite>(),
            TypeId::of::<Transform>(),
        ]
    }

    fn run(&mut self, world: &mut World) {
        let entities = world.query(self.get_component_types());

        for entity in entities {
            let sprite = world.get_component::<MockSprite>(entity).unwrap();
            let transform = world.get_component::<Transform>(entity).unwrap();
            println!("Doing the rendering sys call for {:?}  at  {:?}",sprite,transform);
            // get and provide the right data for the renderer.
        }
    }
}