use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::grid::*;
use copper::resource::{Resources,RenderLayer};
use copper::renderer::render_sys::{RenderSys,TileMapStorage,GridStorage,TileMap,GridRenderSys,TMapHandle,GridHandle,GridRenderMeta};
use copper::resource::{convert_texture,extract_tileset,extract_layer_data};
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


struct Observe;
impl System for Observe {
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
	// let texture = convert_texture("./src/sprite_assets/tiles.png").unwrap();
    // let tile_set =extract_tileset(32,32,&texture); 
    // println!("{}",tile_set.len());
    // let mut tile_set_hash = TextureAsset{textures:HashMap::new()};

    // for tile_index in 0..tile_set.len(){
    //     tile_set_hash.textures.insert(TextureHandle(tile_index as i32),tile_set[tile_index].clone());
    // }

    

    // // now run engine 

    // // todo: 
    // // DONE! attach a JSON loader or decoder 
    // // attach something like a texture handle to a grid and change render sys.

    // // how would they use it? 
    // // for instance, how would rendersys understand that this is in a tileset, and not the generic texture asset with specified texture handles

    // // grid -> has entities that has texture assets 
    // // change renderer
    // let json_read = extract_layer_data("./src/sprite_assets/test2.tmj").unwrap();
    
    // // 1. attach to grid
    // // 2. change rendersys to be able to draw this
    // // add this to a tile_set

    // println!("How many layers are there? -> {:?}",json_read.len());
    // println!("{:?}",json_read);

    // // 
    // // 
    // let mut grid: Grid = Grid::new(30,20,32.0);
    // //
    // let mut count = 0;
    // let layer = &json_read[0];
    // println!("{:?}",layer[0]);
    // for y in 0..20 {
    //     for x in 0..30{
    //         grid.insert_grid((layer[count]-1) as usize,GridPosition{x:x,y:y});
    //         count+=1;
    //     }
    // }

    // // println!("Now some part of the tile map is inserted into grid.{:?}",grid.query_grid(GridPosition{x:0,y:0,}));
    
    // // grid with associated texture handles


    // let mut t_map = TileMap::new(grid.clone(),tile_set_hash.clone());
    // let t_handle = TMapHandle{id:"test".to_string()};
    // let mut tmap_storage = TileMapStorage::new();
    // tmap_storage.storage.insert(t_handle.clone(),t_map);
    // let mut grid_storage = GridStorage::new();
    
    // let grid_handle= GridHandle{id:"test".to_string()};
    // grid_storage.storage.insert(grid_handle.clone(),grid.clone());


    // let mut engine = Engine::new();
    // // engine.resources.insert(tile_set_hash);

    // engine.resources.insert(tmap_storage);
    // engine.resources.insert(grid_storage);
    // // Where does the tilemap even go? is it a part of grid?
    // let x = engine.world.spawn();
    // engine.world.add_component(x, GridRenderMeta{handle:t_handle.clone(), grid:grid_handle.clone(),tile_size:32.0,});
    // // engine.add_system(Update, RenderSys);
    // engine.add_system(Update, Observe);
    // engine.add_system(Update, GridRenderSys);
    // engine.test_run();
    
    
    // /*
    // // layer_test.tmj now exists if we look in the sprite_asset directory 
    // To do:
    // This supports one layer. 
    // DONE 1. A camera that can move around    
    // 2. Layering 
    //     layer_test.tmj added to sprite thingy 
    // What do I need for layering? 
    // 3. Do a new tilemap, more detailed 
    // 4. Be able todo both rendering of grid and Sprite 
    // 5. Animation basics 
    // */
    


    let r_layer: RenderLayer = RenderLayer::Background;
    // (0) Presentationsdel av mina områden
    // (1) todo: add layer to rendercommand to be able to change rendering and the commands.
    // (2) change the render.draw() so it takes into account this. 
    // (3) create better extractors for layers and shit
    // (4) make it possible to add textures from sprite aswell. how does these conflict eachother? 
    // (5) make sprite controllable in a layer grid.
    // (6) spawn alot of sprite that want to move and pass messages to eachother to check concurrency
    // (7) write out all the tests that are required 
    // (8) clean up the directory and dependencies that are used
    // (9) group todo
    // (10) animations

}
