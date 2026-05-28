// use crate::renderer::test_components_renderer::{Transform, MockSprite, TextureAsset};
use crate::ecs::world::World;
// use crate::resource::RenderQueue;
use crate::renderer::render_queue::RenderQueue;
use crate::core::engine::{Startup, Update};
use std::any::TypeId;
use crate::grid::{Grid,GridPosition};
use crate::renderer::render_command::RenderCommand;
use crate::renderer::components::Transform;
use crate::renderer::components::MockSprite;
use crate::renderer::texture::TextureHandle;
use crate::resource::Resources;
use crate::assets::texture_asset::TextureAsset;
// use crate::renderer::test_components_renderer::TextureHandle;
use std::collections::HashMap;
// use std::any::TypeId;
use crate::ecs::system::System;
// use crate::ecs::system::{components_read,components_with,components_without,components_write};
// use crate::{components_read, components_with, components_without, components_write};
use crate::{components_read, components_with, components_without, components_write,system_id,resources_read,resources_write};
// use crate::resource::{Resources,RenderCommand};
use crate::Component;
// use crate::resource::RenderLayer;
use component_macro_derive::Component;
use crate::renderer::render_map::RenderMap;
use crate::renderer::render_layer::RenderLayer;


pub struct NewRenderSys;
impl System for NewRenderSys {
    components_write!();
    components_read!(MockSprite,Transform);
    resources_write!(RenderQueue);
    resources_read!(RenderMap);
    components_with!();
    system_id!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
    let entities = world.query(
        &self.components_read(),
        &self.components_write(),
        &self.components_with(),
        &self.components_without(),
    );

    // println!("\n\nIn NewRenderSys\n\n");

    let grids = {
        let render_map = resources.get::<RenderMap>().unwrap();

        // println!(
        //     "Length of rendermaps is {:?}",
        //     render_map.grids.len()
        // );

        render_map.grids.clone()
    };

    let render_queue = resources.get_mut::<RenderQueue>().unwrap();

    for layers in &grids {
        let grid = layers.grid.clone();

        let width = grid.width;
        let height = grid.height;

        // println!("{:?}", grid);

        let tile_size: f32 = layers.tile_size;
        // println!("Tile size is: {:?}", tile_size);

        for y in 0..height {
            for x in 0..width {
                
                let texture_handle =
                grid.query_grid(GridPosition { x, y });

                if texture_handle.is_empty() {
                    continue;
                }

                render_queue.commands.push(RenderCommand {
                    texture: TextureHandle(texture_handle[0] as i32),
                    layer: layers.layer.clone(),
                    x: x as f32 * tile_size,
                    y: y as f32 * tile_size,
                    texture_map_handle: Some(layers.handle.clone()),
                });
            }
        }
    }

     for entity in entities {
        // println!("Found an entity");
            
        let sprite = world.get_component::<MockSprite>(entity).unwrap();
        let transform = world.get_component::<Transform>(entity).unwrap();
        // println!("Doing the rendering sys call for {:?}  at  {:?}",sprite,transform);
        // resources.render_queue.commands.push(RenderCommand{texture:sprite.texture,x:transform.x,y:transform.y});
        resources.get_mut::<RenderQueue>().unwrap().commands.push(RenderCommand{
            texture:sprite.texture,
            layer: RenderLayer::Sprite,
            x:transform.x,
            y:transform.y,
            texture_map_handle:Some(sprite.map_handle.clone())})
 
    }
}
}

