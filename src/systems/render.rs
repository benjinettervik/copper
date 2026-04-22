#![allow(unused)]

// render sys
use crate::resource::resources::Resources;
// pub fn render_system(world: &World, resources: &mut Resources) {
//     let renderer = resources.renderer.as_mut().unwrap();
// }




// Have to change architecture of systems to incorporate this.



// use crate::engine::world::World;
// use crate::resource::resources::Resources;

// pub struct RenderSystem;

// impl crate::engine::system::System for RenderSystem {
//     fn run(&mut self, world: &mut World, resources: &mut Resources) {
//         let renderer = resources.renderer.as_mut().unwrap();
//         let asset_manager = &resources.asset_manager;

//         for (entity, sprite) in world.query::<MockSprite>() {
//             if let Some(transform) = world.get_component::<Transform>(entity) {
//                 let texture = asset_manager.get(sprite.texture_handle);
//                 renderer.draw(texture, transform.x, transform.y);
//             }
//         }
//     }
// }


