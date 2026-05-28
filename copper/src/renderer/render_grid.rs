use crate::assets::texture_map::TM_Handle;
use crate::renderer::render_layer::RenderLayer;
use crate::grid::Grid;
#[derive(Clone)]
pub struct RenderGrid{
    pub layer: RenderLayer,
    pub grid: Grid,
    pub handle: TM_Handle,
    pub tile_size: f32
}