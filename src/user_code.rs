#![allow(unused)]
use crate::engine::system::*;
use crate::engine::world::*;
use crate::engine::camera::Transform;
use crate::resource::asset_manager::{TextureHandle};
use crate::resource::resources::Resources;
use winit::keyboard::KeyCode;
use crate::input::Input;


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

pub struct DamageSystem;

impl System for DamageSystem{

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for (_entity, hp) in world.query_mut::<MockHp>() {
            hp.value -= 10.0;
        }
    }
}




pub struct PlayerMovementSystem;

impl System for PlayerMovementSystem {
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let input = resources.get::<Input>().unwrap();
        let speed = 0.02;

        world.query2_mut::<Player, Transform, _>(|_, _, transform| {

            if input.is_pressed(KeyCode::KeyW) {
                transform.y += speed;
            }
            if input.is_pressed(KeyCode::KeyS) {
                transform.y -= speed;
            }
            if input.is_pressed(KeyCode::KeyA) {
                transform.x -= speed;
            }
            if input.is_pressed(KeyCode::KeyD) {
                transform.x += speed;
            }

        });
    }
}