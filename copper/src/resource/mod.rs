// use copper::renderer::test_components_renderer::*;
use crate::renderer::test_components_renderer::*;
use crate::renderer::render_sys::*;
use std::collections::HashMap;
pub struct Resources {
    pub texture_hash: TextureAsset,
    pub render_queue: RenderQueue,
}

// basic resources 
impl Resources{
    pub fn new() -> Self{
        Self{
            texture_hash: TextureAsset{textures: HashMap::new(),},
            render_queue: RenderQueue{commands: Vec::new(),}
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