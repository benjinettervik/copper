use copper::ecs::system::*;
use copper::ecs::world::*;
use copper::grid::*;
use copper::resource::*;
use copper::assets::texture_map::*;
use copper::assets::convert_texture::*;
use copper::assets::texture_asset::*;
use copper::renderer::render_sys::NewRenderSys;
use component_macro_derive::*;
use copper::assets::tiled_converter::*;
use copper::core::engine::*;
use copper::resource::config::CopperConfig;
use copper::renderer::camera::*;
use copper::renderer::components::MockSprite;
use copper::renderer::render_map::RenderMap;
use copper::renderer::components::Transform;
use copper::renderer::texture::TextureHandle;
use copper::*;
use std::any::TypeId;
use std::collections::HashMap;
use copper::input::Input;
use winit::keyboard::KeyCode::*;
use copper::resource::time::Time;
struct NPCSPRITEMove;
impl System for NPCSPRITEMove {
    components_read!(MockSprite);
    components_write!(Transform,NPCSPRITE);
    resources_write!();
    resources_read!(Time);
    system_id!();
    components_with!();
    components_without!();
    
    fn run(&mut self, world: &mut World, __resources: &mut Resources) {
        
        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        for entity in entities {
            let dir = {
                let mut steps = world.get_component_mut::<NPCSPRITE>(entity).unwrap();

                if steps.step_count >= 3 {
                    steps.dir = !steps.dir;
                    steps.step_count = 0;
                }
                
                steps.step_count += 1;
                steps.dir
            };

            let mut transform = world.get_component_mut::<Transform>(entity).unwrap();

            if dir {
                transform.x += 32.0;
            } else {
                transform.x -= 32.0;
            }
        }
    }
}
struct MoveIntentSystem;
impl System for MoveIntentSystem {
    components_read!(CameraTarget);
    components_write!(Velocity);
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

        for entity in entities {

            let mut velocity =
                world.get_component_mut::<Velocity>(entity).unwrap();

            velocity.x = 0.0;
            velocity.y = 0.0;

            let speed = 32.0;

            if w {
                velocity.y -= speed;
            }

            if s {
                velocity.y += speed;
            }

            if a {
                velocity.x -= speed;
            }

            if d {
                velocity.x += speed;
            }
        }
    }
}
// }
#[derive(Component)]
pub struct C1 {
    test: bool,
}
pub struct S1;
    impl System for S1 {
        components_read!(C1);
        components_write!();
        resources_write!();
        resources_read!();
        components_with!();
        system_id!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {}
}
struct MoveSystem;
impl System for MoveSystem {

    components_read!();
    components_write!(Transform, Velocity);

    resources_write!();
    resources_read!();

    system_id!();

    components_with!();
    components_without!();

    fn run(
        &mut self,
        world: &mut World,
        _resources: &mut Resources,
    ) {

        let movers = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        let colliders = world.query(
            &vec![TypeId::of::<Collider>()],
            &vec![TypeId::of::<Transform>()],
            &vec![],
            &vec![],
        );

        for entity in movers {

            let (current_x, current_y) = {
                let transform =
                    world.get_component::<Transform>(entity).unwrap();

                (transform.x, transform.y)
            };

            let (vel_x, vel_y) = {
                let velocity =
                    world.get_component::<Velocity>(entity).unwrap();

                (velocity.x, velocity.y)
            };

            let next_x = current_x + vel_x;
            let next_y = current_y + vel_y;

            let mut blocked = false;

            for collider_entity in &colliders {

                if *collider_entity == entity {
                    continue;
                }

                let (collider_x, collider_y) = {
                    let transform =
                        world.get_component::<Transform>(*collider_entity)
                        .unwrap();

                    (transform.x, transform.y)
                };

                let collider = world
                    .get_component::<Collider>(*collider_entity)
                    .unwrap();

                if intersects(
                    next_x,
                    next_y,
                    32.0,
                    32.0,
                    collider_x,
                    collider_y,
                    collider.width,
                    collider.height,
                ) {

                    blocked = true;
                    break;
                }
            }

            if !blocked {

                let mut transform =
                    world.get_component_mut::<Transform>(entity)
                    .unwrap();

                transform.x = next_x;
                transform.y = next_y;
            }
        }
    }
}

#[derive(Component)]
pub struct NPCSPRITE {
    test: bool,
    step_count: u32,
    dir: bool,
}

#[derive(Component)]
pub struct Velocity
{
    pub x:f32,
    pub y:f32,
}
#[derive(Component)]
pub struct Collider {
    pub rigid: bool,
    pub width: f32,
    pub height: f32,
}

fn intersects(
    ax: f32,
    ay: f32,
    aw: f32,
    ah: f32,
    bx: f32,
    by: f32,
    bw: f32,
    bh: f32,
) -> bool {

    ax < bx + bw &&
    ax + aw > bx &&
    ay < by + bh &&
    ay + ah > by
}

pub struct TerrainCollisionStartupSys;
impl System for TerrainCollisionStartupSys{
    components_write!(Collider, Transform);
    components_read!();
    resources_write!();
    resources_read!(RenderMap);
    components_with!();
    system_id!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        let grids = {
            let render_map = resources.get::<RenderMap>().unwrap();
            render_map.grids.clone()

        };
        let object_layer = &grids[1];

        // println!("{:?}",object_layer.grid);
        for y in 0..object_layer.grid.height {
                for x in 0..object_layer.grid.width {
                    let texture_handle = object_layer.grid.query_grid(GridPosition { x, y });

                    if texture_handle.is_empty() {
                        continue;
                    }
                    else{
                        let entity = world.spawn();
                        world.add_component(entity, Collider{rigid:true,height:32.0,width:32.0});
                        world.add_component(entity, Transform {
                                                x: x as f32 * 32.0,
                                                y: y as f32 * 32.0,
                                            });
                    }
                }
            }
    }
}

fn main() {
    // ###############################
    // ###############################
    // ###############################  New testing
    // ###############################
    // ###############################

    let texture = convert_texture("./copper/assets/sprites/tiles.png").unwrap();
    let tile_set = extract_tileset(32, 32, &texture);
    let mut tile_set_hash = TextureAsset {
        textures: HashMap::new(),
    };


    for tile_index in 0..tile_set.len() {
        tile_set_hash.textures.insert(
            TextureHandle(tile_index as i32),
            tile_set[tile_index].clone(),
        );
    }


    let grid: Grid = Grid::new(20, 30, 32.0);
    let sprite_texture = convert_texture("./copper/assets/sprites/32_sprite.png").unwrap();
    let knight_texture1 = convert_texture("./copper/assets/sprites/knight1.png").unwrap();
    let knight_texture2 = convert_texture("./copper/assets/sprites/knight2.png").unwrap();
    let player = convert_texture("./copper/assets/sprites/farm_guy.png").unwrap();
    let mut sprite_set_hash = TextureAsset {
        textures: HashMap::new(),
    };
    sprite_set_hash
        .textures
        .insert(TextureHandle(1), sprite_texture);
    sprite_set_hash
        .textures
        .insert(TextureHandle(2), knight_texture1);
    sprite_set_hash
        .textures
        .insert(TextureHandle(3), knight_texture2);
    sprite_set_hash.textures.insert(TextureHandle(4), player);

    let mut t_map_storage = TextureMap::new();
    t_map_storage.textures.insert(
        TM_Handle {
            id: "xo".to_string(),
        },
        sprite_set_hash,
    );
    t_map_storage.textures.insert(
        TM_Handle {
            id: "dko".to_string(),
        },
        tile_set_hash,
    );

    let render_map = json_to_rendermap(
        "./copper/assets/sprites/big_layer.tmj",
        32.0,
        TM_Handle {
            id: "dko".to_string(),
        },
    )
    .unwrap();

    let mut engine = Engine::new();
    let sprite = MockSprite {
        texture: TextureHandle(1),
        map_handle: TM_Handle {
            id: "xo".to_string(),
        },
    };
    let npcsprite_1 = MockSprite {
        texture: TextureHandle(2),
        map_handle: TM_Handle {
            id: "xo".to_string(),
        },
    };
    let npcsprite_2 = MockSprite {
        texture: TextureHandle(3),
        map_handle: TM_Handle {
            id: "xo".to_string(),
        },
    };
    let player_sprite = MockSprite {
        texture: TextureHandle(4),
        map_handle: TM_Handle {
            id: "xo".to_string(),
        },
    };
    let transform = Transform { x: 1.0, y: 2.0 };
    let transform2 = Transform {
        x: 32.0 * 62.0,
        y: 32.0 * 148.0,
    };
    let transform3 = Transform {
        x: 32.0 * 67.0,
        y: 32.0 * 148.0,
    };
    let transform4 = Transform {
        x: 32.0 * 67.0,
        y: 32.0 * 150.0,
    };

    //
    let entity = engine.world.spawn();
    let entity2 = engine.world.spawn();
    let entity3 = engine.world.spawn();
    let entity4 = engine.world.spawn();
    engine.world.add_component(entity, sprite);
    engine.world.add_component(entity, transform);
    //
    engine.world.add_component(entity2, npcsprite_1);
    engine.world.add_component(entity2, transform2);
    engine.world.add_component(
        entity2,
        NPCSPRITE {
            test: true,
            step_count: 0,
            dir: true,
        },
    );

    //
    engine.world.add_component(entity3, npcsprite_2);
    engine.world.add_component(entity3, transform3);
    engine.world.add_component(
        entity3,
        NPCSPRITE {
            test: true,
            step_count: 0,
            dir: true,
        },
    );
    engine.world.add_component(entity4, player_sprite);
    engine.world.add_component(entity4, transform4);
    engine.world.add_component(entity4, Collider{rigid:true,height:32.0,width:32.0});
    engine.world.add_component(entity4, Velocity{x:0.0,y:0.0});
    engine.world.add_component(entity4, CameraTarget);

    engine.resources.insert(t_map_storage);
    engine.resources.insert(render_map);
    // engine.spawn()

    let mut config = engine.resources.get_mut::<CopperConfig>().unwrap();
    config.scheduler_guard_rails = true;

    engine.add_system(Startup,TerrainCollisionStartupSys);
    engine.add_system(Update, MoveIntentSystem);
    engine.add_system(Update, MoveSystem);
    engine.add_system(Update, NPCSPRITEMove);
    engine.add_system(Update, CameraFollowSystem);
    engine.add_system(Update, NewRenderSys);

    engine.run_render(true);
}
