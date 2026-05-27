use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::grid::*;
use copper::renderer::render_sys::{
    GridHandle, GridRenderMeta, GridRenderSys, GridStorage, NewRenderSys, RenderSys, TMapHandle,
    TileMap, TileMapStorage,
};
use copper::resource::camera::*;
use copper::resource::{RenderLayer, Resources, TM_Handle, TextureMap};
use copper::resource::{convert_texture, extract_layer_data2, extract_tileset, json_to_rendermap};
// use copper::input::Input;
use component_macro_derive::*;
use copper::renderer::test_components_renderer::*;
use copper::*;
use core::time;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::mpsc::RecvTimeoutError::Timeout;
use std::time::SystemTime;

use winit::keyboard::KeyCode;
// use copper::input::Action;
use copper::input::Input;
use copper::input::input::*;
use winit::keyboard::KeyCode::*;
use winit::keyboard::*;

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

struct SpawnEntitiesSystem;
impl System for SpawnEntitiesSystem {
    components_read!();
    components_write!();
    components_with!();
    components_without!();
    resources_read!();
    resources_write!();
    system_id!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for _ in 0..120000 {
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
    {
        let mut engine = Engine::new();

        // Startup
        engine
            .add_system(Startup, SpawnEntitiesSystem)
            .add_system(Update, IncreaseComp1System)
            .add_system(Update, IncreaseComp2System)
            .add_system(Update, IncreaseComp3System)
            .add_system(Update, IncreaseComp4System)
            .add_system(Update, IncreaseComp5System)
            .add_system(Update, IncreaseComp6System)
            .add_system(Update, IncreaseComp7System)
            .add_system(Update, IncreaseComp8System);

        let start_time_concurrent = SystemTime::now();
        engine.test_run(20);

        println!(
            "Amount of time elapsed for concurrent run: {}",
            start_time_concurrent.elapsed().unwrap().as_millis()
        );
    }

    {
        let mut engine = Engine::new();

        // Startup
        engine
            .add_system(Startup, SpawnEntitiesSystem)
            .add_system(Update, IncreaseComp1System)
            .add_system(Update, IncreaseComp2System)
            .add_system(Update, IncreaseComp3System)
            .add_system(Update, IncreaseComp4System)
            .add_system(Update, IncreaseComp5System)
            .add_system(Update, IncreaseComp6System)
            .add_system(Update, IncreaseComp7System)
            .add_system(Update, IncreaseComp8System)
            .add_system(Update, NewRenderSys);

        let start_time_non_concurrent = SystemTime::now();
        //engine.test_run(10);

        println!(
            "Amount of time elapsed for non-concurrent run: {}",
            start_time_non_concurrent.elapsed().unwrap().as_millis()
        );
    }
}
