use copper::devtools::*;
use copper::engine::system::*;
use copper::engine::*;
use copper::user_code::*;
use copper::engine::world::World;

// 
// 
// test shit
use std::any::TypeId;
use std::collections::HashSet;




#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Debug)]
struct Health {
    value: i32,
}


fn main() {
    // println!("Yo");
    
    // let mut meta = SystemMeta {
    //     reads: HashSet::new(),
    //     writes: HashSet::new(),
    //     resource_reads: HashSet::new(),
    // resource_writes: HashSet::new(),
    // };

    // <(&Position, &Velocity, &mut Health)>::meta(&mut meta);

    // println!("Reads: {:?}", meta.reads);
    // println!("Writes: {:?}", meta.writes);

    // assert!(meta.reads.contains(&TypeId::of::<Position>()));
    // assert!(meta.reads.contains(&TypeId::of::<Velocity>()));
    // assert!(meta.writes.contains(&TypeId::of::<Health>()));
    let mut world = World::new();

    let e1 = world.spawn();
    let e2 = world.spawn();

    // e1 has both components
    world.add_component(e1, Position { x: 1.0, y: 2.0 });
    world.add_component(e1, Velocity { dx: 0.1, dy: 0.2 });

    // e2 only has Position
    world.add_component(e2, Position { x: 5.0, y: 6.0 });

//    for (entity, pos, vel) in world
  //      .query::<(&Position, &Velocity)>()
   //     .iter()
   // {
   //     println!("Entity {}: {:?} {:?}", entity, pos, vel);
   // }
    
  
    let mut engine = Engine::new();

    let texture = engine.load_png("troll.png");
    let entity = engine.spawn();
    engine.add_component(entity, MockSprite {
        name: "Bob".to_string(),
        texture_handle: texture,
        layer: 1,
    });
    engine.add_component(entity, Transform {
        x: 0.0,
        y: 0.0,
    });
    engine.add_component(entity, Player);
    engine.add_component(entity, CameraTarget);

    add_grid_background(&mut engine, 20);


    engine.register_system(RenderSystem);
    engine.run();
}
