use crate::renderer::test_components_renderer::{Transform, MockSprite, TextureAsset};
use crate::engine::world::World;
use crate::resource::RenderQueue;
use crate::engine::{Startup, Update};
use std::any::TypeId;
use crate::grid::{Grid,GridPosition};
use crate::renderer::test_components_renderer::TextureHandle;
use std::collections::HashMap;
use crate::engine::system::System;
// use crate::engine::system::{components_read,components_with,components_without,components_write};
use crate::{components_read, components_with, components_without, components_write};
use crate::resource::{Resources,RenderCommand};
use crate::Component;
use component_macro_derive::Component;





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
            // println!("Doing the rendering sys call for {:?}  at  {:?}",sprite,transform);
            // resources.render_queue.commands.push(RenderCommand{texture:sprite.texture,x:transform.x,y:transform.y});
            resources.get_mut::<RenderQueue>().unwrap().commands.push(RenderCommand{texture:sprite.texture,x:transform.x,y:transform.y})
        }
    }
}

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub struct TMapHandle
{
    pub id:String,
}
#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub struct GridHandle
{
    pub id:String,
}
pub struct TileMapStorage{
    pub storage: HashMap<TMapHandle,TileMap>
}

impl TileMapStorage{
    pub fn new () -> Self{
        Self{storage:HashMap::new(),}
    }
}

pub struct TileMap{
    pub grid:Grid,
    pub texture_asset:TextureAsset,    
}

impl TileMap{
    pub fn new(grid:Grid,texture_assets:TextureAsset) ->Self {
        Self{
            grid:grid,
            texture_asset:texture_assets
        }

    }
}

pub struct GridStorage {
    pub storage: HashMap<GridHandle,Grid>,
}

impl GridStorage{
    pub fn new  () -> Self{
        Self{storage:HashMap::new(),}
    }
}

#[derive(Component)]
pub struct GridRenderMeta{
    pub handle: TMapHandle,
    pub grid: GridHandle,
}

pub struct GridRenderSys;
impl System for GridRenderSys {
    components_write!();
    components_read!(GridRenderMeta);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        println!("This is the entity with gridrendermeta {:?}",entities);

        for entity in entities {
        let render_meta = world.get_component::<GridRenderMeta>(entity).unwrap();

        let (width, height, cells) = {
            let grid_storage = resources.get::<GridStorage>().unwrap();
            let grid = grid_storage
                .storage
                .get(&render_meta.grid)
                .unwrap();

            (
                grid.width,
                grid.height,
                grid.clone(),
            )
        };

        let TILE_SIZE: f32 = width as f32;

        // render_queue.commands.push(RenderCommand {
        //     texture: TextureHandle(texture_handle[0] as i32),
        //     x: x as f32 * TILE_SIZE,
        //     y: y as f32 * TILE_SIZE,
        // });
        let render_queue = resources.get_mut::<RenderQueue>().unwrap();
        render_queue.is_grid = Some(render_meta.grid.clone());
        render_queue.t_map = Some(render_meta.handle.clone());
        for y in 0..height {
            for x in 0..width {
                // println!("{},{}", x, y);

                let texture_handle =
                    cells.query_grid(GridPosition { x, y });

                render_queue.commands.push(RenderCommand {
                    texture: TextureHandle(texture_handle[0] as i32),
                    x: x as f32 * TILE_SIZE,
                    y: y as f32 * TILE_SIZE,
                });
            }
        }
    }
    // println!("{:?}",resources.get::<RenderQueue>().unwrap());
    }
}

