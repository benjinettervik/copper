use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::grid::*;
use copper::resource::{Resources,RenderLayer,TM_Handle,TextureMap};
use copper::renderer::render_sys::{NewRenderSys,RenderSys,TileMapStorage,GridStorage,TileMap,GridRenderSys,TMapHandle,GridHandle,GridRenderMeta};
use copper::resource::{convert_texture,extract_tileset,extract_layer_data2,json_to_rendermap};
use copper::resource::camera::*;
// use copper::input::Input;
use std::collections::HashMap;
use copper::renderer::test_components_renderer::*;
use copper::*;
use std::any::TypeId;
use component_macro_derive::*;


use winit::keyboard::KeyCode;
// use copper::input::Action;
use copper::input::Input;
use copper::input::input::*;
use winit::keyboard::*;
use winit::keyboard::KeyCode::*;


struct MoveCamera;
impl System for MoveCamera {
    components_read!();
    components_write!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {

        let input = resources.get::<Input>().unwrap();

        let w = input.state.is_key_pressed(KeyW);
        let s = input.state.is_key_pressed(KeyS);
        let a = input.state.is_key_pressed(KeyA);
        let d = input.state.is_key_pressed(KeyD);

        let mut camera = resources.get_mut::<Camera2D>().unwrap();

        if w {
            camera.y -= 20.0;
        }
        if s {
            camera.y += 20.0;
        }
        if a {
            camera.x -= 20.0;
        }
        if d {
            camera.x += 20.0;
        }
    }
}
// }

fn main() {

    
    


    // ###############################
    // ###############################
    // ###############################  New testing
    // ###############################
    // ###############################



    // loads the tileset.png
    let texture = convert_texture("./src/sprite_assets/tiles.png").unwrap();
    // extract the pixel data
    let tile_set =extract_tileset(32,32,&texture); 
    // println!("{}",tile_set.len());
    let mut tile_set_hash = TextureAsset{textures:HashMap::new()};

    // insert it as a textureAsset
    for tile_index in 0..tile_set.len(){
        tile_set_hash.textures.insert(TextureHandle(tile_index as i32),tile_set[tile_index].clone());
    }

    // // new function to be made --> json.layers to RenderGrid


    let json_read = extract_layer_data2("./src/sprite_assets/big_layer.tmj").unwrap();
    let mut grid: Grid = Grid::new(20,30,32.0);
    let mut count = 0;
    let layer = &json_read.layers[0];


    let sprite_texture = convert_texture("./src/sprite_assets/32_sprite.png").unwrap();
    let mut sprite_set_hash = TextureAsset{textures:HashMap::new()};
    sprite_set_hash.textures.insert(TextureHandle(1),sprite_texture);
    

    let mut t_map_storage = TextureMap::new();
    t_map_storage.textures.insert(TM_Handle{id:"xo".to_string()},sprite_set_hash);
    t_map_storage.textures.insert(TM_Handle{id:"dko".to_string()},tile_set_hash);
    
    let render_map = json_to_rendermap("./src/sprite_assets/big_layer.tmj",32.0,TM_Handle{id:"dko".to_string()}).unwrap();
    

    let mut engine = Engine::new();
    let sprite = MockSprite{texture:TextureHandle(1),map_handle:TM_Handle{id:"xo".to_string()}};
    let transform = Transform{x:1.0,y:2.0};


    let entity = engine.world.spawn();
    engine.world.add_component(entity,sprite);
    engine.world.add_component(entity,transform);
    // insert the texture data stored in t_map_storage.
    engine.resources.insert(t_map_storage);
    // where is the rendermap stored?
    engine.resources.insert(render_map);
    engine.add_system(Update,MoveCamera);
    engine.add_system(Update,NewRenderSys);
    
    // run the engine
    // 
    // To do: Scheduler and game
    // 

    engine.test_run(16);
}
