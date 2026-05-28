use std::collections::HashMap;
use crate::renderer::texture::TextureHandle;
use crate::renderer::texture::Texture;
#[derive(Clone)]
pub struct TextureAsset {
    pub textures: HashMap<TextureHandle, Texture>,
}