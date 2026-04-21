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