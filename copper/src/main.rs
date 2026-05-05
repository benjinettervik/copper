mod test_stuff;
use copper::engine::*;
use test_stuff::*;

// rend
use copper::renderer::test_components_renderer::*;
use copper::renderer::render_sys::*;
use copper::resource::camera::*;
use copper::resource::{convert_texture};
use copper::engine::world::World;
use copper::renderer::render_sys::*;
use copper::renderer::test_components_renderer::*;
use copper::rgba;
use std::collections::HashMap;
use winit::keyboard::KeyCode;
use copper::input::Action;
use copper::input::Input;

// renderer test
// needs to be on main threads, thus not in tests.
fn main() {
    // rgba 0-255val
    let r = 255;
    let g = 255;
    let b = 255;
    let a = 255;
    
    let mut world= World::new(); 
    let entity = world.spawn();
    // texture
    // using macro for some pixel-data, just white square
    let texture = Texture {width:10, height:10, pixel_data: rgba!(255,255,255,255,10,10),};
    let texture2 = Texture {width: 10, height: 10, pixel_data: rgba!(155,155,155,255,10,10),};
    let texture3 = convert_texture("./src/sprite_assets/test.png").unwrap();

    println!("{:?}",texture3);
    let entity2 = world.spawn();
    let sprite2 = MockSprite { texture: TextureHandle(2) };
    let transform2 = Transform { x: 100.0, y: 50.0 }; // different position

    world.add_component(entity2, sprite2);
    world.add_component(entity2, transform2);


        
    
    // text_hash
    let mut text_hash = TextureAsset{textures: HashMap::new()};
    text_hash.textures.insert(TextureHandle(1),texture);
    text_hash.textures.insert(TextureHandle(2), texture3);
    
    // sprite
    let sprite = MockSprite {
        texture: TextureHandle(1),
    };

    // transform position
    let transform = Transform { x: 10.0, y: 10.0 };

    world.add_component(entity,sprite);
    world.add_component(entity,transform);
    world.add_component(entity,CameraTarget);

    // engine time
    let mut engine = Engine::new();
    engine.world = world;
    engine.resources.texture_hash = text_hash;

     //bind inputs here
    engine.resources.input.binds.bind_key(KeyCode::KeyW, Action::Up);
    engine.resources.input.binds.bind_key(KeyCode::KeyS, Action::Down);
    engine.resources.input.binds.bind_key(KeyCode::KeyA, Action::Left);
    engine.resources.input.binds.bind_key(KeyCode::KeyD, Action::Right);

    // specify RenderSys as an update system
    engine.add_system(Update, RenderSys);
    engine.add_system(Update, CameraFollowSystem);
    engine.test_run();

}
