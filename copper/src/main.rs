mod test_stuff;
use copper::engine::*;
use test_stuff::*;

// rend
use copper::*;
use copper::renderer::test_components_renderer::*;
use copper::renderer::render_sys::*;
use copper::resource::camera::*;
use copper::resource::{convert_texture};
use copper::engine::world::World;
use copper::renderer::render_sys::*;
use copper::engine::system::System;
use copper::resource::Resources;
use copper::renderer::test_components_renderer::*;
use copper::rgba;
// use copper::engine::system::System;
use std::collections::HashMap;
use copper::components_with;
use copper::components_without;
use copper::components_write;
use copper::components_read;

use std::any::TypeId;
use copper::query;
use winit::keyboard::KeyCode;
use copper::input::Action;
use copper::input::Input;
use copper::input::input::*;
use winit::keyboard::*;
use winit::keyboard::KeyCode::*;
pub struct MySupportResources;
// renderer test
// needs to be on main threads, thus not in tests.

struct MoveSpriteSystem;
// impl System for MoveSpriteSystem {
//     components_read!(CameraTarget);
//     components_write!(Transform);
//     components_with!();
//     components_without!();

//     fn run(&mut self, world: &mut World, resources: &mut Resources) {
//         let sprite = query!(self, world)
//             .first()
//             .unwrap()
//             .clone();

//         if let Some(mut transform) = world.get_component_mut::<Transform>(sprite) {

//             if resources.get::<Input>().unwrap().state.is_key_pressed(KeyW) {
//                 transform.y -= 20.0;
//             }
//             if resources.get::<Input>().unwrap().state.is_key_pressed(KeyS) {
//                 transform.y += 20.0;
//             }
//             if resources.get::<Input>().unwrap().state.is_key_pressed(KeyA) {
//                 transform.x -= 20.0;
//             }
//             if resources.get::<Input>().unwrap().state.is_key_pressed(KeyD) {
//                 transform.x += 20.0;
//             }
//         }
//     }
// }

fn main() {
    // rgba 0-255val
    let r = 255;
    let g = 255;
    let b = 255;
    let a = 255;
    
    let mut engine = Engine::new();
    let ent_1 = engine.world.spawn();
    let ent_2 = engine.world.spawn();
    // texture
    let texture1 = convert_texture("./src/sprite_assets/test.png").unwrap();
    let texture2 = Texture {width: 10, height: 10, pixel_data: rgba!(155,155,155,255,10,10),};
    // player_sprite
    let sprite = MockSprite{texture:TextureHandle(1)};
    let transform = Transform{x:0.0,y:90.0};
    // dot sprite
    let dot_sprite = MockSprite{texture:TextureHandle(2)};
    let transform2 = Transform{x:100.0,y:50.0};
    // 
    let mut text_hash = TextureAsset{textures: HashMap::new()};
    text_hash.textures.insert(TextureHandle(1),texture1);
    text_hash.textures.insert(TextureHandle(2), texture2);
    *engine.resources.get_mut::<TextureAsset>().unwrap() = text_hash;
    // add components to world.
    engine.world.add_component(ent_1, sprite);
    engine.world.add_component(ent_1, transform);
    engine.world.add_component(ent_2, dot_sprite);
    engine.world.add_component(ent_2, transform2);
    engine.world.add_component(ent_1,CameraTarget);
    
    // basic sys
    engine.add_system(Update, RenderSys);
    engine.add_system(Update, MoveSpriteSystem);
    engine.add_system(Update, CameraFollowSystem);
    engine.test_run();


    //#################
    //#################
    //################# DO NOT TOUCH FILE ABOVE THIS!
    //#################
    //#################
    //#################



    // här gör man
    // @@ Step 5: Add your own struct to engine.resources 
    pub struct MyOwnResources;
    // @@ Step 6: Make MoveSpriteSystem work 
    // System is beneath main

}

impl System for MoveSpriteSystem{
        // What components will the system query? 
        components_read!();
        components_write!();
        components_with!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {
        

        // What does the system require from resource?
        }       

}

