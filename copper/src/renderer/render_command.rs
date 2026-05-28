use crate::assets::texture_map::TM_Handle;
use crate::renderer::render_layer::RenderLayer;
use crate::renderer::texture::TextureHandle;

#[derive(Debug, Clone)]
pub struct RenderCommand {
    pub texture: TextureHandle,
    pub layer: RenderLayer,
    pub x: f32,
    pub y: f32,
    pub texture_map_handle: Option<TM_Handle>,
    //what texture, what layer it is in, where to place it, where to get it
}
