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


struct NPCSPRITEMove;
impl System for NPCSPRITEMove {
    components_read!(MockSprite, NPCSPRITE);
    components_write!(Transform);
    resources_write!();
    resources_read!();
    system_id!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {

        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        for entity in entities{
            // println!("Finds an entity");
            let dir = {
                let mut steps = world
                    .get_component_mut::<NPCSPRITE>(entity)
                    .unwrap();

                if steps.step_count >= 3 {
                    steps.dir = !steps.dir;
                    steps.step_count = 0;
                }

                steps.step_count += 1;

                steps.dir
            };

            let mut transform = world.get_component_mut::<Transform>(entity).unwrap();
            
            
            if dir
            {
                transform.x+=32.0;
            }
            else{
                transform.x-=32.0;
            }
        }

}
}
struct MovePlayer;
impl System for MovePlayer {
    components_read!(CameraTarget);
    components_write!(Transform);
    resources_write!();
    resources_read!(Input);
    system_id!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {

        let input = resources.get::<Input>().unwrap();

        let w = input.state.is_key_pressed(KeyW);
        let s = input.state.is_key_pressed(KeyS);
        let a = input.state.is_key_pressed(KeyA);
        let d = input.state.is_key_pressed(KeyD);



        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );


        for entity in entities
        {
            let mut transform = world.get_component_mut::<Transform>(entity).unwrap();
            if w {
                transform.y -= 32.0;
            }
            if s {
                transform.y += 32.0;
                // camera.y += 20.0;
            }
            if a {
                transform.x -= 32.0;
                // camera.x -= 20.0;
            }
            if d {
                transform.x += 32.0;
                // camera.x += 20.0;
            }
        }



    }
}
// }
#[derive(Component)]
    pub struct NPCSPRITE {
        test: bool,
        step_count: u32, 
        dir: bool,
    }

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
    let knight_texture1 = convert_texture("./src/sprite_assets/knight1.png").unwrap();
    let knight_texture2 = convert_texture("./src/sprite_assets/knight2.png").unwrap();
    let player = convert_texture("./src/sprite_assets/farm_guy.png").unwrap();
    let mut sprite_set_hash = TextureAsset{textures:HashMap::new()};
    sprite_set_hash.textures.insert(TextureHandle(1),sprite_texture);
    sprite_set_hash.textures.insert(TextureHandle(2),knight_texture1);
    sprite_set_hash.textures.insert(TextureHandle(3),knight_texture2);
    sprite_set_hash.textures.insert(TextureHandle(4),player);
    

    let mut t_map_storage = TextureMap::new();
    t_map_storage.textures.insert(TM_Handle{id:"xo".to_string()},sprite_set_hash);
    t_map_storage.textures.insert(TM_Handle{id:"dko".to_string()},tile_set_hash);
    
    let render_map = json_to_rendermap("./src/sprite_assets/big_layer.tmj",32.0,TM_Handle{id:"dko".to_string()}).unwrap();
    

    let mut engine = Engine::new();
    let sprite = MockSprite{texture:TextureHandle(1),map_handle:TM_Handle{id:"xo".to_string()}};
    let NPCSPRITE_sprite1 = MockSprite{texture:TextureHandle(2),map_handle:TM_Handle{id:"xo".to_string()}};
    let NPCSPRITE_sprite2 = MockSprite{texture:TextureHandle(3),map_handle:TM_Handle{id:"xo".to_string()}};
    let player_sprite = MockSprite{texture:TextureHandle(4),map_handle:TM_Handle{id:"xo".to_string()}};
    let transform = Transform{x:1.0,y:2.0};
    let transform2 = Transform{x:32.0*62.0,y:32.0*148.0};
    let transform3 = Transform{x:32.0*67.0,y:32.0*148.0};
    let transform4 = Transform{x:32.0*67.0,y:32.0*150.0};

    //
    let entity = engine.world.spawn();
    let entity2 = engine.world.spawn();
    let entity3 = engine.world.spawn();
    let entity4 = engine.world.spawn();
    engine.world.add_component(entity,sprite);
    engine.world.add_component(entity,transform);
    // 
    engine.world.add_component(entity2,NPCSPRITE_sprite1);
    engine.world.add_component(entity2,transform2);
    engine.world.add_component(entity2, NPCSPRITE{test:true,step_count:0,dir:true});
    
    // 
    engine.world.add_component(entity3,NPCSPRITE_sprite2);
    engine.world.add_component(entity3,transform3);
    engine.world.add_component(entity3, NPCSPRITE{test:true,step_count:0,dir:true});
    // insert the texture data stored in t_map_storage.
    // 
    engine.world.add_component(entity4,player_sprite);
    engine.world.add_component(entity4,transform4);
    engine.world.add_component(entity4,CameraTarget);

    engine.resources.insert(t_map_storage);
    engine.resources.insert(render_map);

    engine.add_system(Update,MovePlayer);
    engine.add_system(Update,NPCSPRITEMove);
    engine.add_system(Update,CameraFollowSystem);
    engine.add_system(Update,NewRenderSys);


    //
    
    
    // run the engine
    // 
    // To do: Scheduler and game
    // 

    engine.test_run(16);
}
