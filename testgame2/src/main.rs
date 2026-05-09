use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::grid::*;
use copper::resource::Resources;
use copper::renderer::render_sys::RenderSys;
use copper::resource::{convert_texture,extract_tileset,extract_layer_data};
use copper::resource::camera::*;
use std::collections::HashMap;
use copper::renderer::test_components_renderer::*;
use copper::*;
use std::any::TypeId;
use component_macro_derive::*;




fn main() {
	let texture = convert_texture("./src/sprite_assets/tiles.png").unwrap();
    let tile_set =extract_tileset(32,32,&texture); 
    println!("{}",tile_set.len());
    let mut tile_set_hash = TextureAsset{textures:HashMap::new()};

    for tile_index in 0..tile_set.len(){
        tile_set_hash.textures.insert(TextureHandle(tile_index as i32),tile_set[tile_index].clone());
    }

    // now run engine 

    // todo: 
    // DONE! attach a JSON loader or decoder 
    // attach something like a texture handle to a grid and change render sys.

    // how would they use it? 
    // for instance, how would rendersys understand that this is in a tileset, and not the generic texture asset with specified texture handles

    // grid -> has entities that has texture assets 
    // change renderer
    let json_read = extract_layer_data("./src/sprite_assets/test2.tmj").unwrap();
    
    // 1. attach to grid
    // 2. change rendersys to be able to draw this
    // add this to a tile_set

    println!("How many layers are there? -> {:?}",json_read.len());
    println!("{:?}",json_read);

    // 
    // 
    let mut grid: Grid = Grid::new(30,20,32.0);
    //
    let mut count = 0;
    let layer = &json_read[0];
    for y in 0..20 {
        for x in 0..30{
            grid.insert_grid(layer[count] as usize,GridPosition{x:x,y:y});
            count+=1;
        }
    }
    // grid.insert_grid(255,GridPosition{x:1,y:1,});
    println!("{:?}",grid.query_grid(GridPosition{x:0,y:0,}));
    
    // grid with associated texture handles
    pub struct TileComponent{
        text_handle: TextureHandle,
    }


    let mut engine = Engine::new();
    engine.resources.insert(tile_set_hash);
    engine.add_system(Update, RenderSys);
    engine.test_run();
    
    
    	
	
}
