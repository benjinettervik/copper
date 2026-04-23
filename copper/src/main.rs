mod test_stuff;
use copper::engine::*;
use test_stuff::*;


// rend
use copper::renderer::test_components_renderer::*;
use copper::renderer::render_sys::*;
use copper::engine::world::World;
use std::collections::HashMap;
use copper::rgba;


// fn main() {
//     println!("--- Main Run ---");
//     let mut engine = Engine::new();

//     engine.add_system(Startup, SpawnEntitiesSystem)
//         .add_system(Update, HealthSystem)
//         .run_cycles(5);
// }


// renderer test
fn main(){
    let r = 255;
    let g = 255;
    let b = 255;
    let a = 255;
        // texture
    let texture = Texture {width:10, height:10, pixel_data: rgba!(255,255,255,255,10,10),};
        
        // rgba!(255,255,255,255,10,10);
        // text_hash
    let mut text_hash = TextureAsset{textures: HashMap::new()};
        text_hash.textures.insert(TextureHandle(1),texture);
        // sprite
    let sprite = MockSprite {texture: TextureHandle(1)};
        // transform position
    let transform = Transform{x:0.0,y:0.0};

        // can be copied to the following
    let mut world= World::new();
        // 
    let entity = world.spawn();
        world.add_component(entity,sprite);
        world.add_component(entity,transform);

    // Ensure
        // println!("{:?}",world.get_component::<MockSprite>(entity));
        // println!("{:?}",world.get_component::<Transform>(entity));




    // engine time
    let mut engine = Engine::new();
    engine.world = world;
    engine.resources.texture_hash = text_hash;
    // engine.add_system(RenderSys);
    engine.add_system(Update, RenderSys);
    engine.test_run();


    

}



