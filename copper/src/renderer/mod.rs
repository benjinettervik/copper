pub mod test_components_renderer;
pub mod render_sys;
use std::sync::Arc;
use winit::window::Window;
use pixels::{Pixels, SurfaceTexture};
use crate::resource::{TextureMap,Resources,RenderCommand};
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
        // print!("\n\nIn render draw!\n\n");
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


        // make a queue ish

        let mut commands = {
            let render_queue = resources.get_mut::<RenderQueue>().unwrap();
                render_queue.commands.sort_by_key(|cmd| cmd.layer.clone());
                render_queue.commands.clone()
        }; 

        let t_map_storage = resources.get::<TextureMap>().unwrap();

        for render_command in commands.drain(..) {
            
        // let t_map_storage = resources.get::<TextureMap>().unwrap();
        // let render_queue = resources.get_mut::<RenderQueue>().unwrap();
        // render_queue.commands.sort_by_key(|cmd| cmd.layer.clone());
        
        // for render_command in render_queue.commands.clone(){
            
            // at the current moment, it draws either sprites or rendermaps -- which kind of is terrain like.
            // this part has to be changed since it no longer depends on the same structs.
            
            // Okay, now we have the storage of the pixeldata.
            // println!("{:?}",render_command);
            let map_handle = render_command.texture_map_handle.clone().unwrap();
            let texture_handle = render_command.texture;
            let texture_asset = t_map_storage.textures.get(&map_handle);
            let texture = texture_asset.unwrap().textures.get(&texture_handle).unwrap();
            // println!("\nGets pass the breakpoint!\n");


            // Now what is remaining is the simple layering
            // draw background first
            // draw rest after?
            // draw first layer
            // draw second layer
            

            // i simply have to sort the render queue in what layer it draws
            // Render queue has .commands which is a list, 
            // render commands has a .layer
            

            // ska göras i denna ordning
            // pub enum RenderLayer {
            //     Background,
            //     Terrain,
            //     Objects,
            //     Sprite,
            //     Effects
            // }

            let tex_width = texture.width as usize;
            let tex_height = texture.height as usize;

            let camera = resources.get::<Camera2D>().unwrap();

            // let camera_x = resources.Camera2D.x as isize;
            // let camera_y = resources.Camera2D.y as isize;
            // let camera_x = camera.x as isize;
            // let camera_y = camera.y as isize;

            // test debug
            
            let camera_x = camera.x as isize;
            let camera_y = camera.y as isize;

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

                    // if alpha is 0, skip drawing the pixel, since the renderqueue is sorted it will not overwrite the background layer pixel                    
                    let alpha = texture.pixel_data[texture_index + 3];
                    if alpha == 0 {
                        continue;
                    }

                    frame[screen_index..screen_index + 4]
                        .copy_from_slice(
                            &texture.pixel_data[texture_index..texture_index + 4]
                        );
                }
            }
        }

        self.pixels.render().unwrap();
        // resources.render_queue.commands.clear();
        &resources.get_mut::<RenderQueue>().unwrap().commands.clear();
    }
}
