// use copper::renderer::test_components_renderer::*;
pub mod camera;
use crate::renderer::test_components_renderer::*;
use crate::resource::camera::*;
use crate::renderer::render_sys::*;
use crate::input::Input;
use std::collections::HashMap;
pub struct Resources {
    pub texture_hash: TextureAsset,
    pub render_queue: RenderQueue,
    pub Camera2D: Camera2D,
    pub input: Input,
}

// basic resources 
impl Resources{
    pub fn new() -> Self{
        Self{
            texture_hash: TextureAsset{textures: HashMap::new(),},
            render_queue: RenderQueue{commands: Vec::new(),},
            Camera2D: Camera2D::new(),
            input: Input::new(),
        }
    }
}

pub struct RenderCommand {
    pub texture: TextureHandle,
    pub x: f32,
    pub y: f32,
}

pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
}



pub fn convert_texture(path: &str) -> Result<Texture,String> {
        

        let img = image::open(path)
            .map_err(|e| format!("Failed to load image: {}", e))?
            .to_rgba8();
        
        let (width, height) = img.dimensions();
        let pixel_data = img.into_raw();
        Ok(Texture {
            width,
            height,
            pixel_data,
        })
    }
// camera sys

