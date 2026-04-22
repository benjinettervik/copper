#![allow(unused)]
use crate::engine::system::*;
use crate::engine::world::*;
use crate::engine::camera::Transform;
use crate::resource::asset_manager::{TextureHandle};
use crate::resource::resources::Resources;
use winit::keyboard::KeyCode;
use crate::input::Input;
use crate::engine::meta::SystemMeta;
use std::any::TypeId;


#[derive(Debug)]
pub struct MockComponent1{
    pub x: f32,
}

impl MockComponent1{
    pub fn new() -> Self{
        Self{x: 1.0,}
    }
}

#[derive(Debug)]
pub struct MockSprite{
    pub name: String,
    pub texture_handle: TextureHandle,
    pub layer: i32,
}

pub struct Player;

pub struct MockComponent2{
    pub pp: f32,
    pub y: MockComponent1
}

pub struct MockHp{
    pub value: f32,
}


#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Health {
    pub value: i32,
}


pub struct SpawnSystem;

impl System for SpawnSystem {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        let e1 = world.spawn();

        world.add_component(e1, Position { x: 0.0, y: 0.0 });
        world.add_component(e1, Velocity { dx: 1.0, dy: 0.5 });
        world.add_component(e1, Health { value: 100 });
        world.add_component(e1, Player);

        let e2 = world.spawn();

        world.add_component(e2, Position { x: 5.0, y: 5.0 });
        world.add_component(e2, Velocity { dx: -0.5, dy: 0.0 });
    }

    fn meta(&self, meta: &mut SystemMeta) {
        // writing components during setup
        meta.writes.insert(TypeId::of::<Position>());
        meta.writes.insert(TypeId::of::<Velocity>());
        meta.writes.insert(TypeId::of::<Health>());
        meta.writes.insert(TypeId::of::<Player>());
    }
}

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for (_entity, (pos, vel)) in world
            .query::<(&mut Position, &Velocity)>()
            .iter()
        {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    }

    fn meta(&self, meta: &mut SystemMeta) {
        <(&mut Position, &Velocity)>::meta(meta);
    }
}

pub struct DamageSystem;

impl System for DamageSystem {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for (_entity, health) in world
            .query::<(&mut Health,)>()
            .iter()
        {
            health.value -= 1;
        }
    }

    fn meta(&self, meta: &mut SystemMeta) {
        <(&mut Health,)>::meta(meta);
    }
}