use crate::renderer::render_grid::RenderGrid;
pub struct RenderMap {
    pub grids: Vec<RenderGrid>,
}
impl RenderMap {
    pub fn new() -> Self {
        Self { grids: Vec::new() }
    }
}

