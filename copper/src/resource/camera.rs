use crate::engine::system::System;
use crate::engine::world::World;
use std::any::TypeId;
use crate::{components_read, components_with, components_without, components_write};
use crate::resource::*;
use crate::Component;


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
impl Component for CameraTarget {
    fn name(&self) -> &str {
            "CameraTarget"
    }
}

pub struct CameraTarget;
pub struct CameraFollowSystem;

impl System for CameraFollowSystem {
    components_write!();
    components_read!(CameraTarget,Transform);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        
        let mut target_position: Option<Transform> = None;
        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        for entity in entities
        {   
                let transform = world.get_component::<Transform>(entity).unwrap();
                resources.Camera2D.set_position(transform.x,transform.y);
        }
    }
}