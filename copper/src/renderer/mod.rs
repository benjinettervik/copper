pub mod test_components_renderer;
pub mod render_sys;
use std::sync::Arc;
use winit::window::Window;
use pixels::{Pixels, SurfaceTexture};
use crate::resource::{Resources,RenderCommand};
use crate::resource::RenderQueue;
use crate::resource::camera::Camera2D;
use crate::renderer::test_components_renderer::TextureAsset;
use crate::renderer::render_sys::TileMapStorage;


// window made issues with lifetime complexities, but found a solution

pub struct Renderer {
    window: &'static Window, //reference to a Window that will live for program entirety 
    pixels: Pixels<'static>, //framebuffer for RGBA pixels, which we can fill - tied to window. 
}

// Renderer
impl Renderer {
    pub fn new(window: Window) -> Self {

        let window = Box::leak(Box::new(window)); 
        // box::new -> allocate window on heap 
        // box::leak -> reference that heap data, not being freed. called "intentional leak"
        // acceptable since 1 window for entire app lifetime
        // we ensure window will live for program entirety 


        let size = window.inner_size();
        

        // window is tied to surface texture
        // surface texture is "what we draw on"
        let surface_texture = SurfaceTexture::new(
            size.width,
            size.height,
            &*window, 
        );

        // surface texture is tied to pixels
        let pixels = Pixels::new(
            size.width,
            size.height,
            surface_texture,
        ).expect("Failed to create Pixels");

        Self {
            window, 
            pixels,
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }


   pub fn draw(&mut self, resources: &mut Resources) {
   
        // width and size of the pixelbuffer 
        let size = self.pixels.context().texture_extent;
        let width = size.width as usize;
        let height = size.height as usize;


        // mutable slice of the pixels - so we can modify the pixels
        // frame [u8]
        let frame = self.pixels.frame_mut();

        // basically sets the background
        // iterate over pixel 4 bytes at a time- a chunk represent one pixel
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }



        // from the commands that we stored through RenderSys, we go through them
        for render_command in &resources.get::<RenderQueue>().unwrap().commands{
        // for render_command in &resources.render_queue.commands {

                    // texture has {height,width and data (pixel data --> rgba)}
                    // let texture = resources.texture_hash.textures.get(&render_command.texture).unwrap();
                    
                    let texture = if resources.get::<RenderQueue>().unwrap().is_grid != None {
            println!("This render queue is a grid thingy.");

            let tile_map_storage =
                resources.get::<TileMapStorage>().unwrap();

            let t_map_key = resources
                .get::<RenderQueue>()
                .unwrap()
                .t_map
                .clone()
                .unwrap();

            let tile_map = tile_map_storage
                .storage
                .get(&t_map_key)
                .unwrap();

            let text_assets = &tile_map.texture_asset;

            println!("before texture!");

            let texture = text_assets
                .textures
                .get(&render_command.texture)
                .unwrap();

            println!("WE HAVE texture after texture was gotten from hashmap!");

            texture.clone()
        } else {
            resources
                .get::<TextureAsset>()
                .unwrap()
                .textures
                .get(&render_command.texture)
                .unwrap()
                .clone()
        };
            let tex_width = texture.width as usize;
            let tex_height = texture.height as usize;

            let camera = resources.get::<Camera2D>().unwrap();

            // let camera_x = resources.Camera2D.x as isize;
            // let camera_y = resources.Camera2D.y as isize;
            // let camera_x = camera.x as isize;
            // let camera_y = camera.y as isize;

            // test debug
            let camera_x = (camera.x+450.0) as isize;
            let camera_y = (camera.y+300.0) as isize;

            let screen_center_x = (width as isize) / 2;
            let screen_center_y = (height as isize) / 2;
            
            let base_x = render_command.x as isize - camera_x + screen_center_x;
            let base_y = render_command.y as isize - camera_y + screen_center_y;

            for y in 0..tex_height {
                for x in 0..tex_width {
                    let screen_x = base_x + x as isize;
                    let screen_y = base_y + y as isize;
                    
                    if screen_x < 0 || screen_y < 0 {
                       continue;
                    }
                    let screen_x = screen_x as usize;
                    let screen_y = screen_y as usize;
                    // skip pixels outside screen
                    if screen_x >= width || screen_y >= height {
                        continue;
                    }
                    // e.g. (10.0 * 800 + 10) * 4
                    // skips the first ten rows
                    // screen index -> 32040 is -> frame[32040..32044] -> is (10, 10)
                    let screen_index = (screen_y * width + screen_x) * 4;
                    let texture_index = (y * tex_width + x) * 4;

                    // importantly -> screen-index 
                    // copy_from_slice() copies 4byte (RGBA)
                    // it is stored in 1d memory 
                    frame[screen_index..screen_index + 4].copy_from_slice(&texture.pixel_data[texture_index..texture_index + 4]);
                }
            }
        }

        self.pixels.render().unwrap();
        // resources.render_queue.commands.clear();
        &resources.get_mut::<RenderQueue>().unwrap().commands.clear();
    }
}
