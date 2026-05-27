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

        let size = self.pixels.context().texture_extent;
        let width = size.width as usize;
        let height = size.height as usize;

        let frame: &mut [u32] =
            bytemuck::cast_slice_mut(self.pixels.frame_mut());

        frame.fill(0xFF000000);

        let camera = resources.get::<Camera2D>().unwrap();

        let camera_x = camera.x as isize;
        let camera_y = camera.y as isize;

        let screen_center_x = (width as isize) / 2;
        let screen_center_y = (height as isize) / 2;


        let mut commands = {

            let render_queue =
                resources.get_mut::<RenderQueue>().unwrap();
            render_queue
                .commands
                .sort_by_key(|cmd| cmd.layer);
            std::mem::take(&mut render_queue.commands)
        };
        let texture_storage =
            resources.get::<TextureMap>().unwrap();

        for command in commands.drain(..) {

            let map_handle =
                command.texture_map_handle.clone().unwrap();

            let texture = texture_storage
                .textures
                .get(&map_handle)
                .unwrap()
                .textures
                .get(&command.texture)
                .unwrap();

            let tex_width = texture.width as usize;
            let tex_height = texture.height as usize;

            let base_x =
                command.x as isize - camera_x + screen_center_x;

            let base_y =
                command.y as isize - camera_y + screen_center_y;

            if (base_x + tex_width as isize) < 0 ||
                    base_x >= width as isize ||
                (base_y + tex_height as isize) < 0 ||
                    base_y >= height as isize {
                    continue;
            }

            for y in 0..tex_height {

                let screen_y = base_y + y as isize;

                if screen_y < 0 ||
                screen_y >= height as isize {
                    continue;
                }

                let screen_y = screen_y as usize;

                let tex_row = y * tex_width;
                let screen_row = screen_y * width;

                for x in 0..tex_width {

                    let screen_x = base_x + x as isize;

                    if screen_x < 0 ||
                    screen_x >= width as isize {
                        continue;
                    }

                    let screen_x = screen_x as usize;

                    let src =
                        texture.pixel_data[tex_row + x];

                    // alpha test
                    if (src >> 24) == 0 {
                        continue;
                    }
                    frame[screen_row + screen_x] = src;
                }
            }
        }

        self.pixels.render().unwrap();
    }
}
