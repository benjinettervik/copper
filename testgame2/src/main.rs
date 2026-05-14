use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::grid::*;
use copper::resource::{Resources,RenderLayer,TM_Handle,TextureMap};
use copper::renderer::render_sys::{RenderSys,TileMapStorage,GridStorage,TileMap,GridRenderSys,TMapHandle,GridHandle,GridRenderMeta};
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

    
    // ###############################
    // ###############################
    // ###############################  New testing
    // ###############################
    // ###############################
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
    // let json_read = extract_layer_data("./src/sprite_assets/layer_test.tmj").unwrap();
    
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
    
    


    // ###############################
    // ###############################
    // ###############################  New testing
    // ###############################
    // ###############################



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
    


    // RenderCommands rehash 
    // RenderQueue new fix
    // Render-draw logic changed 


    // loads the tileset.png
    let texture = convert_texture("./src/sprite_assets/tiles.png").unwrap();
    // extract the pixel data
    let tile_set =extract_tileset(32,32,&texture); 
    println!("{}",tile_set.len());
    let mut tile_set_hash = TextureAsset{textures:HashMap::new()};

    // insert it as a textureAsset
    for tile_index in 0..tile_set.len(){
        tile_set_hash.textures.insert(TextureHandle(tile_index as i32),tile_set[tile_index].clone());
    }

    // // new function to be made --> json.layers to RenderGrid


    let json_read = extract_layer_data2("./src/sprite_assets/layer_test.tmj").unwrap();
    let mut grid: Grid = Grid::new(20,30,32.0);
    let mut count = 0;
    let layer = &json_read.layers[0];

    print!("The amount of layers are: {:?}",json_read.layers.len());
    println!("Dimensions are: ({:?},{:?})",json_read.height,json_read.width);
    
    // RenderMap 
    
    // What do we need to pass to rendersys?
    // In this new iteration Renderqueue should not have to differentiate between the commands, they will say what layer, what pos and what texturehandle.
    
    // RenderDraw till know:
    // pos 
    // text_handle, but not the key of how to access these. 
    // Could say that RenderGrid has a grid_handle stored, a key on how to access they texture pixel data
    
    // textureAssets are stored
    // What do we need to pass to render_draw()? PReviously it was needed grid handles etc.
    
    
    
    // we now have a simple RenderMap struct that we can work with -- this should be the goal for the rendersys to accomodate aswell as draw 
    
    // step 2: make a sprite with a what not.
    let sprite_texture = convert_texture("./src/sprite_assets/32_sprite.png").unwrap();
    let mut sprite_set_hash = TextureAsset{textures:HashMap::new()};
    sprite_set_hash.textures.insert(TextureHandle(1),sprite_texture);
    
    // All texture assets are stored here 
    // in this type
    
    // now all pixel data is stored in this
    let mut t_map_storage = TextureMap::new();
    t_map_storage.textures.insert(TM_Handle{id:"xo".to_string()},sprite_set_hash);
    t_map_storage.textures.insert(TM_Handle{id:"dko".to_string()},tile_set_hash);
    
    let render_map = json_to_rendermap("./src/sprite_assets/layer_test.tmj",32.0,TM_Handle{id:"dko".to_string()});
    
    println!("Rendermap has a total of {:?} rendergrids",render_map.unwrap().grids.len());
    // rendermap has the information of WHAT to draw and WHERE 
    // where do i place the render_map?

    // new step --> make render_data, add mocksprite and render_maps
    // render_data becomes -> render_commands, this is what render_sys does.

    // let render_data = RenderData{}
    // rendermap now know what to ask t_map_storage for.
    // 


    
    // the tool function should be: 
        // given a specified json -> make a RenderMap 
    // create a rendermap based on the yadyada of grid 
    // println!("{:?}",layer[0]);
    
    // for y in 0..20 {
    //     for x in 0..20{
    //         grid.insert_grid((layer[count]-1) as usize,GridPosition{x:x,y:y});
    //         count+=1;
    //     }
    // }

    // At the moment it is attached to a certain layer -- 
    // layer 0 is one grid, reasonable.
    // layer 1 will be yet another grid.


    //
    // let r_layer: RenderLayer = RenderLayer::Background;
    
    
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
