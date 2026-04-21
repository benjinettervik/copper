use crate::engine::system::System;
use crate::engine::world::World;
use crate::resource::resources::Resources;

#[derive(Debug)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Camera2D {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 2.0,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}

pub struct CameraTarget;

pub struct CameraFollowSystem;

// impl System for CameraFollowSystem {
//     fn run(&mut self, world: &mut World, resources: &mut Resources) {
//         let mut target_position = None;

//         for (entity, _target) in world.query::<CameraTarget>() {
//             if let Some(transform) = world.get_component::<Transform>(entity) {
//                 target_position = Some((transform.x, transform.y));
//                 break;
//             }
//         }

//         if let Some((x, y)) = target_position {
//             if let Some(camera) = resources.get_mut::<Camera2D>() {
//                 camera.set_position(x, y);
//             }
//         }
//     }
// }
