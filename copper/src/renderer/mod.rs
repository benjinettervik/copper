pub mod test_components_renderer;
pub mod render_sys;
use std::sync::Arc;
use winit::window::Window;
use pixels::{Pixels, SurfaceTexture};

pub struct Renderer<'a> {
    pixels: Pixels<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(window: &'a Window, width: u32, height: u32) -> Self {
        let surface_texture = SurfaceTexture::new(width, height, window);
        let pixels = Pixels::new(width, height, surface_texture).unwrap();

        Self { pixels }
    }
}