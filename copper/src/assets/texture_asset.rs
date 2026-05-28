use crate::renderer::texture::Texture;
use crate::renderer::texture::TextureHandle;
use std::collections::HashMap;
#[derive(Clone)]
pub struct TextureAsset {
    pub textures: HashMap<TextureHandle, Texture>,
}

