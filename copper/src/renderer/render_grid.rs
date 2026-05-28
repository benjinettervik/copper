use crate::assets::texture_map::TM_Handle;
use crate::grid::Grid;
use crate::renderer::render_layer::RenderLayer;
#[derive(Clone)]
pub struct RenderGrid {
    pub layer: RenderLayer,
    pub grid: Grid,
    pub handle: TM_Handle,
    pub tile_size: f32,
}

