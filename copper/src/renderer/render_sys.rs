use crate::renderer::test_components_renderer::{Transform, MockSprite, TextureAsset};
use crate::engine::world::World;
use crate::engine::{Startup, Update, SystemRoutine};
use std::any::TypeId;
use crate::engine::system::System;
use crate::resource::{Resources,RenderCommand};






// render sys like system trait demands
pub struct RenderSys;
impl System for RenderSys {
    fn get_component_types(&self) -> Vec<TypeId> {
        vec![
            TypeId::of::<MockSprite>(),
            TypeId::of::<Transform>(),
        ]
    }

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = world.query(self.get_component_types());

        for entity in entities {
            let sprite = world.get_component::<MockSprite>(entity).unwrap();
            let transform = world.get_component::<Transform>(entity).unwrap();
            println!("Doing the rendering sys call for {:?}  at  {:?}",sprite,transform);
            resources.render_queue.commands.push(RenderCommand{texture:sprite.texture,x:transform.x,y:transform.y});
        }
    }
}