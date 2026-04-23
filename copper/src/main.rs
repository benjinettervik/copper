mod test_stuff;
use copper::engine::*;
use test_stuff::*;



// rend
use copper::renderer::test_components_renderer::*;
use copper::renderer::render_sys::*;
use copper::engine::world::World;
use std::collections::HashMap;
use copper::rgba;


// renderer test
// needs to be on main threads, thus not in tests.
fn main(){
    // rgba 0-255val
    let r = 255;
    let g = 255;
    let b = 255;
    let a = 255;
    
    // texture
    // using macro for some pixel-data, just white square
    let texture = Texture {width:10, height:10, pixel_data: rgba!(255,255,255,255,10,10),};
        
    
    // text_hash
    let mut text_hash = TextureAsset{textures: HashMap::new()};
    text_hash.textures.insert(TextureHandle(1),texture);
    
    // sprite
    let sprite = MockSprite {texture: TextureHandle(1)};
    
    // transform position
    let transform = Transform{x:10.0,y:10.0};

    let mut world= World::new(); 
    let entity = world.spawn();
    world.add_component(entity,sprite);
    world.add_component(entity,transform);

    // engine time
    let mut engine = Engine::new();
    engine.world = world;
    engine.resources.texture_hash = text_hash;
    
    // specify RenderSys as an update system
    engine.add_system(Update, RenderSys);
    engine.test_run();

}



