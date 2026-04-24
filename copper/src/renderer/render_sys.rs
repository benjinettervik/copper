use crate::renderer::test_components_renderer::{Transform, MockSprite, TextureAsset};
use crate::engine::world::World;
use crate::engine::{Startup, Update, SystemRoutine};
use std::any::TypeId;
use crate::engine::system::System;
// use crate::engine::system::{components_read,components_with,components_without,components_write};
use crate::{components_read, components_with, components_without, components_write};
use crate::resource::{Resources,RenderCommand};






// render sys like system trait demands
pub struct RenderSys;
impl System for RenderSys {
    components_write!();
    components_read!(MockSprite, Transform);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        for entity in entities {
            let sprite = world.get_component::<MockSprite>(entity).unwrap();
            let transform = world.get_component::<Transform>(entity).unwrap();
            println!("Doing the rendering sys call for {:?}  at  {:?}",sprite,transform);
            resources.render_queue.commands.push(RenderCommand{texture:sprite.texture,x:transform.x,y:transform.y});
        }
    }
}