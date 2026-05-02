use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::resource::Resources;
use copper::renderer::render_sys::RenderSys;
use copper::resource::convert_texture;
use copper::resource::camera::*;
use std::collections::HashMap;
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
// #[derive(Copy,Clone)]
pub fn extract_tileset(tile_h: u32, tile_w: u32, texture: &Texture,) -> Vec<Texture> {
    let bpp: usize = 4;

    let width_u32 = texture.width;
    let height_u32 = texture.height;

    let width = width_u32 as usize;
    let height = height_u32 as usize;

    let tile_w_usize = tile_w as usize;
    let tile_h_usize = tile_h as usize;

    assert!(width % tile_w_usize == 0);
    assert!(height % tile_h_usize == 0);

    let tiles_x = width / tile_w_usize;
    let tiles_y = height / tile_h_usize;

    let row_stride = width * bpp;
    let tile_row_bytes = tile_w_usize * bpp;

    let mut tiles = Vec::with_capacity(tiles_x * tiles_y);

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let mut tile = vec![0u8; tile_w_usize * tile_h_usize * bpp];

            for row in 0..tile_h_usize {
                let src_y = ty * tile_h_usize + row;
                let src_x = tx * tile_w_usize;

                let src_start = src_y * row_stride + src_x * bpp;
                let dst_start = row * tile_row_bytes;

                tile[dst_start..dst_start + tile_row_bytes]
                    .copy_from_slice(
                        &texture.pixel_data
                            [src_start..src_start + tile_row_bytes],
                    );
            }

            tiles.push(Texture {
                height: tile_h, // keep u32
                width: tile_w,  // keep u32
                pixel_data: tile,
            });
        }
    }

    tiles
}
    

fn main() {
    // 
    // println!("Hello, world!");
    // let mut engine = Engine::new();
    // // println!("{:?}",texture);
	let texture = convert_texture("./src/sprite_assets/tileset.png").unwrap();
    let tile_set =extract_tileset(16,16,&texture); 
    // // println!("{:?}",extract_tileset(16,16,&texture).len());
    
    // let mut text_hash = TextureAsset{textures: HashMap::new()};
    // text_hash.textures.insert(TextureHandle(1),tile_set[0]);

    // engine.test_run();
    
    
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
    text_hash.textures.insert(TextureHandle(1), tile_set[0].clone());
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

    // specify RenderSys as an update system
    engine.add_system(Update, RenderSys);
    engine.add_system(Update, CameraFollowSystem);
    engine.test_run();
    // 
    // 
    // 
    // Step1. Load a tileset, or be able to read JSON for a tiletset atleast.
	// Two choices -- either be able to read the formats that Tiled provides. E.g. its compatibel with tiled, or make your own cuts.
	
	
}
