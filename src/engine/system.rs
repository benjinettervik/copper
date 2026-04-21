use crate::engine::world::*;
use crate::engine::meta::*;
use crate::engine::camera::{Camera2D, Transform};
use crate::resource::resources::Resources;
use crate::user_code::*;
use crate::resource::asset_manager::{AssetManager,TextureHandle};
use crate::resource::renderer::Renderer;
pub type Entity = usize;
use std::any::TypeId;
use std::any::Any;

use std::collections::HashSet;


pub trait System: Any {
    fn run(&mut self, world: &mut World, resources: &mut Resources);
    fn meta(&self, meta: &mut SystemMeta);
}



#[derive(Debug)]
pub struct DrawCommand {
    pub texture: TextureHandle,
    pub position: (f32, f32),
    pub layer: i32,
}

#[derive(Debug)]
pub struct GridRenderData {
    pub width: i32,
    pub height: i32,
    pub camera_position: (f32, f32),
    pub camera_zoom: f32,
    pub tiles: Vec<DrawCommand>,
}
impl GridRenderData {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            camera_position: (0.0, 0.0),
            camera_zoom: 1.0,
            tiles: Vec::new(),
        }
    }
}




pub struct RenderSystem;
impl System for RenderSystem {
    fn run(&mut self, world: &mut World, resources: &mut Resources) {

        let mut render_data = GridRenderData::new();

        if let Some(camera) = resources.get::<Camera2D>() {
            render_data.camera_position = (camera.x, camera.y);
            render_data.camera_zoom = camera.zoom;
        }

        for (_entity, (sprite, transform)) in world
            .query::<(&MockSprite, &Transform)>()
            .iter()
        {
            render_data.tiles.push(DrawCommand {
                texture: sprite.texture_handle,
                position: (transform.x, transform.y),
                layer: sprite.layer,
            });
        }

        render_data.tiles.sort_by_key(|tile| tile.layer);

        // renderer (resource write)
        let mut renderer = resources.remove::<Renderer>()
            .expect("Renderer missing");

        let asset_manager = resources.get::<AssetManager>()
            .expect("AssetManager missing");

        renderer.draw(render_data, asset_manager);

        resources.insert(renderer);
    }
        fn meta(&self, meta: &mut SystemMeta) {
        <(&MockSprite, &Transform)>::meta(meta);

        meta.resource_reads.insert(TypeId::of::<Camera2D>());
        meta.resource_reads.insert(TypeId::of::<AssetManager>());
        meta.resource_writes.insert(TypeId::of::<Renderer>());
    }
}


