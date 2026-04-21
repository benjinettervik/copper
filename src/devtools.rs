use crate::engine::camera::Transform;
use crate::engine::Engine;
use crate::user_code::MockSprite;

pub fn add_grid_background(engine: &mut Engine, radius: i32) {
    let grid_handle = engine.load_png("gridsquare.png");

    for x in -radius..=radius {
        for y in -radius..=radius {
            let grid = engine.spawn();
            engine.add_component(grid, MockSprite { name: "Grid".to_string(), texture_handle: grid_handle, layer: 0 });
            engine.add_component(grid, Transform { x: x as f32, y: y as f32 });
        }
    }
}
