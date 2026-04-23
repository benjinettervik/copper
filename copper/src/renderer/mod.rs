pub mod test_components_renderer;
pub mod render_sys;
use std::sync::Arc;
use winit::window::Window;
use pixels::{Pixels, SurfaceTexture};

pub struct Renderer;

impl Renderer{

    pub fn new() ->Self{
        Self{}
    }

    pub fn draw(&mut self){
        println!("Renderer drawing");
    }
    // draw takes a window
}

/*


*/