use component_macro_derive::*;
use copper::core::engine::*;
use copper::ecs::system::*;
use copper::ecs::world::*;
use copper::resource::*;
use copper::*;
use std::any::TypeId;
use std::env;
use winit::keyboard::KeyLocation::Standard;

#[derive(Component)]
struct Component1 {
    value: i32,
}

#[derive(Component)]
struct Component2 {
    value: i32,
}

#[derive(Component)]
struct Component3 {
    value: i32,
}

#[derive(Component)]
struct Component4 {
    value: i32,
}

#[derive(Component)]
struct Component5 {
    value: i32,
}

#[derive(Component)]
struct Component6 {
    value: i32,
}

#[derive(Component)]
struct Component7 {
    value: i32,
}

#[derive(Component)]
struct Component8 {
    value: i32,
}

struct Spawn1000EntitiesSystem;
impl System for Spawn1000EntitiesSystem {
    components_read!();
    components_write!();
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for _ in 0..1000 {
            let entitiy = world.spawn();
            world.add_component(entitiy, Component1 { value: 0 });
            world.add_component(entitiy, Component2 { value: 0 });
            world.add_component(entitiy, Component3 { value: 0 });
            world.add_component(entitiy, Component4 { value: 0 });
            world.add_component(entitiy, Component5 { value: 0 });
            world.add_component(entitiy, Component6 { value: 0 });
            world.add_component(entitiy, Component7 { value: 0 });
            world.add_component(entitiy, Component8 { value: 0 });
        }
    }
}

struct Spawn10000EntitiesSystem;
impl System for Spawn10000EntitiesSystem {
    components_read!();
    components_write!();
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for _ in 0..10000 {
            let entitiy = world.spawn();
            world.add_component(entitiy, Component1 { value: 0 });
            world.add_component(entitiy, Component2 { value: 0 });
            world.add_component(entitiy, Component3 { value: 0 });
            world.add_component(entitiy, Component4 { value: 0 });
            world.add_component(entitiy, Component5 { value: 0 });
            world.add_component(entitiy, Component6 { value: 0 });
            world.add_component(entitiy, Component7 { value: 0 });
            world.add_component(entitiy, Component8 { value: 0 });
        }
    }
}

struct Spawn100000EntitiesSystem;
impl System for Spawn100000EntitiesSystem {
    components_read!();
    components_write!();
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for _ in 0..100000 {
            let entitiy = world.spawn();
            world.add_component(entitiy, Component1 { value: 0 });
            world.add_component(entitiy, Component2 { value: 0 });
            world.add_component(entitiy, Component3 { value: 0 });
            world.add_component(entitiy, Component4 { value: 0 });
            world.add_component(entitiy, Component5 { value: 0 });
            world.add_component(entitiy, Component6 { value: 0 });
            world.add_component(entitiy, Component7 { value: 0 });
            world.add_component(entitiy, Component8 { value: 0 });
        }
    }
}

struct IncreaseComp1System;
impl System for IncreaseComp1System {
    components_read!();
    components_write!(Component1);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component1>(entity).unwrap();
            component.value += 1;
        }
    }
}

struct IncreaseComp2System;
impl System for IncreaseComp2System {
    components_read!();
    components_write!(Component2);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component2>(entity).unwrap();
            component.value += 1;
        }
    }
}

struct IncreaseComp3System;
impl System for IncreaseComp3System {
    components_read!();
    components_write!(Component3);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component3>(entity).unwrap();
            component.value += 1;
        }
    }
}

struct IncreaseComp4System;
impl System for IncreaseComp4System {
    components_read!();
    components_write!(Component4);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component4>(entity).unwrap();
            component.value += 1;
        }
    }
}

struct IncreaseComp5System;
impl System for IncreaseComp5System {
    components_read!();
    components_write!(Component5);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component5>(entity).unwrap();
            component.value += 1;
        }
    }
}

struct IncreaseComp6System;
impl System for IncreaseComp6System {
    components_read!();
    components_write!(Component6);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component6>(entity).unwrap();
            component.value += 1;
        }
    }
}

struct IncreaseComp7System;
impl System for IncreaseComp7System {
    components_read!();
    components_write!(Component7);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component7>(entity).unwrap();
            component.value += 1;
        }
    }
}

struct IncreaseComp8System;
impl System for IncreaseComp8System {
    components_read!();
    components_write!(Component8);
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut component = world.get_component_mut::<Component8>(entity).unwrap();
            component.value += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let concurrent_arg = &args[1];
    let interactive_arg = &args[2];
    let entities_arg = &args[3];

    if (concurrent_arg != "true" && concurrent_arg != "false")
        || (interactive_arg != "true" && interactive_arg != "false")
        || (entities_arg != "1000" && entities_arg != "10000" && entities_arg != "100000")
    {
        panic!("You need to supply arguments (true/false) for concurrency and interactivity");
    }

    if interactive_arg == "true" {
        let mut engine = Engine::new();

        if entities_arg == "1000" {
            engine.add_system(Startup, Spawn1000EntitiesSystem);
        }

        if entities_arg == "10000" {
            engine.add_system(Startup, Spawn10000EntitiesSystem);
        }

        if entities_arg == "100000" {
            engine.add_system(Startup, Spawn100000EntitiesSystem);
        }

        engine
            .add_system(Update, IncreaseComp1System)
            .add_system(Update, IncreaseComp2System)
            .add_system(Update, IncreaseComp3System)
            .add_system(Update, IncreaseComp4System)
            .add_system(Update, IncreaseComp5System)
            .add_system(Update, IncreaseComp6System)
            .add_system(Update, IncreaseComp7System)
            .add_system(Update, IncreaseComp8System);

        println!(
            "Will run with settings: \nConcurrent: {}\nInteractive: {}
        ",
            concurrent_arg, interactive_arg
        );

        println!("Please close window manually when you want to see results!");
        if concurrent_arg == "true" {
            engine.run_render(true);
        } else {
            engine.run_render(false);
        }
    } else {
        {
            let mut engine = Engine::new();

            if entities_arg == "1000" {
                engine.add_system(Startup, Spawn1000EntitiesSystem);
            }

            if entities_arg == "10000" {
                engine.add_system(Startup, Spawn10000EntitiesSystem);
            }

            if entities_arg == "100000" {
                engine.add_system(Startup, Spawn100000EntitiesSystem);
            }

            engine
                .add_system(Update, IncreaseComp1System)
                .add_system(Update, IncreaseComp2System)
                .add_system(Update, IncreaseComp3System)
                .add_system(Update, IncreaseComp4System)
                .add_system(Update, IncreaseComp5System)
                .add_system(Update, IncreaseComp6System)
                .add_system(Update, IncreaseComp7System)
                .add_system(Update, IncreaseComp8System);

            println!(
                "Will run with settings: \nConcurrent: {}\nInteractive: {}
        ",
                concurrent_arg, interactive_arg
            );

            println!("Running non-concurrently for 10 seconds.");
            engine.run_without_render(false, 10);
        }
        {
            let mut engine = Engine::new();

            if entities_arg == "1000" {
                engine.add_system(Startup, Spawn1000EntitiesSystem);
            }

            if entities_arg == "10000" {
                engine.add_system(Startup, Spawn10000EntitiesSystem);
            }

            if entities_arg == "100000" {
                engine.add_system(Startup, Spawn100000EntitiesSystem);
            }

            engine
                .add_system(Update, IncreaseComp1System)
                .add_system(Update, IncreaseComp2System)
                .add_system(Update, IncreaseComp3System)
                .add_system(Update, IncreaseComp4System)
                .add_system(Update, IncreaseComp5System)
                .add_system(Update, IncreaseComp6System)
                .add_system(Update, IncreaseComp7System)
                .add_system(Update, IncreaseComp8System);

            println!(
                "Will run with settings: \nConcurrent: {}\nInteractive: {}",
                concurrent_arg, interactive_arg
            );

            println!("Running concurrently for 10 seconds.");
            engine.run_without_render(true, 10);
        }
    }
}
