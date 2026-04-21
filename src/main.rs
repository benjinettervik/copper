use copper::devtools::*;
use copper::engine::system::*;
use copper::engine::*;
use copper::user_code::*;

fn main() {
    let mut engine = Engine::new();

    let texture = engine.load_png("troll.png");
    let entity = engine.spawn();
    engine.add_component(entity, MockSprite {
        name: "Bob".to_string(),
        texture_handle: texture,
        layer: 1,
    });
    engine.add_component(entity, Transform {
        x: 0.0,
        y: 0.0,
    });
    engine.add_component(entity, Player);
    engine.add_component(entity, CameraTarget);

    add_grid_background(&mut engine, 20);

    engine.register_system(PlayerMovementSystem);
    engine.register_system(CameraFollowSystem);
    engine.register_system(RenderSystem);

    engine.run();
}
