use crate::renderer::test_components_renderer::{Transform, MockSprite, TextureAsset};
use crate::engine::world::World;

use std::any::TypeId;

pub struct RenderSys;

impl RenderSys {
    pub fn run(
        pixels: &mut pixels::Pixels,
        world: &World,
        textures: &TextureAsset,
        screen_width: u32,
        screen_height: u32,
    ) {
        let frame = pixels.frame_mut();

        let width = screen_width as usize;
        let height = screen_height as usize;

        for px in frame.chunks_exact_mut(4) {
            px.copy_from_slice(&[0, 0, 0, 255]);
        }

        let mut components = Vec::new();
        components.push(TypeId::of::<MockSprite>());
        components.push(TypeId::of::<Transform>());

        let entities = world.query(components);

        for entity in entities {
            let sprite = world.get_component::<MockSprite>(entity).unwrap();
            let transform = world.get_component::<Transform>(entity).unwrap();

            let texture = textures
                .textures
                .get(&sprite.texture)
                .unwrap();

            let x = transform.x as usize;
            let y = transform.y as usize;

            for ty in 0..texture.height as usize {
                for tx in 0..texture.width as usize {
                    let screen_x = x + tx;
                    let screen_y = y + ty;

                    if screen_x >= width || screen_y >= height {
                        continue;
                    }

                    let src_idx = (ty * texture.width as usize + tx) * 4;
                    let dst_idx = (screen_y * width + screen_x) * 4;

                    frame[dst_idx..dst_idx + 4]
                        .copy_from_slice(&texture.pixel_data[src_idx..src_idx + 4]);
                }
            }
        }

        pixels.render().unwrap();
    }
}