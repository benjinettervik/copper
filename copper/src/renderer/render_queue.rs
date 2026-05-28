use crate::renderer::render_sys::TMapHandle;
use crate::renderer::render_sys::GridHandle;
use crate::renderer::render_command::RenderCommand;

#[derive(Debug,Clone)]
pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
    pub is_grid: Option<GridHandle>,
    pub t_map: Option<TMapHandle>,
}