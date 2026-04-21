use crate::engine::world::*;
use crate::engine::camera::{Camera2D, Transform};
use crate::resource::resources::Resources;
use crate::user_code::*;
use crate::resource::asset_manager::{AssetManager,TextureHandle};
use crate::resource::renderer::Renderer;
use std::any::TypeId;
pub type Entity = usize;
use std::any::Any;


pub trait System: Any {
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}

pub struct RenderSystem;

impl System for RenderSystem {
    fn run(&mut self, world: &mut World, resources: &mut Resources) {

        let mut render_data = GridRenderData::new();
        if let Some(camera) = resources.get::<Camera2D>() {
            render_data.camera_position = (camera.x, camera.y);
            render_data.camera_zoom = camera.zoom;
        }

        for (entity, sprite) in world.query::<MockSprite>() {
            if let Some(transform) = world.get_component::<Transform>(entity) {
                render_data.tiles.push(DrawCommand {
                    texture: sprite.texture_handle,
                    position: (transform.x, transform.y),
                    layer: sprite.layer,
                });
            }
        }
        render_data.tiles.sort_by_key(|tile| tile.layer);

        let mut renderer = resources.remove::<Renderer>()
            .expect("Renderer missing");

        let asset_manager = resources.get::<AssetManager>()
            .expect("AssetManager missing");

        renderer.draw(render_data, asset_manager);

        resources.insert(renderer);
    }
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