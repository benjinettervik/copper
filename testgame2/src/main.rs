use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::resource::Resources;
use copper::resource::convert_texture;
use copper::renderer::test_components_renderer::*;
use copper::*;
use std::any::TypeId;
use component_macro_derive::*;


// #[derive(Component)]
// struct TransformComponent {
//     pos_x: i32,
//     pos_y: i32,
// }



// 30 min
// pub pixel_data: Vec<u8>, //rgba 0-255 
fn extract_tileset(height:u32,width:u32, texture:Texture){
    println!("In extract tile_set");
    // 
    println!("Extracting a tileset of dimensions: height-> {}, width {}",height,width);
    // 
    let tile_amount = (texture.height/height) * (texture.width/width);
    println!("Amount of tiles will be {}",tile_amount);
    println!("Length of pixel data is: {}", texture.pixel_data.len());
    // 
}

fn main() {
    // 
    println!("Hello, world!");
    let mut engine = Engine::new();
	let texture = convert_texture("./src/sprite_assets/tileset.png").unwrap();
    // println!("{:?}",texture);
    extract_tileset(16,16,texture);
    engine.test_run();
    // 
    // 
    // 
    // Step1. Load a tileset, or be able to read JSON for a tiletset atleast.
	// Two choices -- either be able to read the formats that Tiled provides. E.g. its compatibel with tiled, or make your own cuts.
	
	
}
