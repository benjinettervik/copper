// use copper::renderer::test_components_renderer::*;
use crate::renderer::test_components_renderer::*;
use crate::renderer::render_sys::*;
use std::collections::HashMap;
pub struct Resources {
    pub texture_hash: TextureAsset
}

impl Resources{
    pub fn new() -> Self{
        Self{
            texture_hash: TextureAsset{textures: HashMap::new(),}
        }
    }
}
