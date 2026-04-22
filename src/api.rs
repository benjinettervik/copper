#![allow(unused)]
use crate::engine::*;
use crate::engine::camera::Camera2D;
use crate::engine::system::*;
use crate::resource::asset_manager::{TextureHandle};

impl Engine {
    pub fn spawn(&mut self) -> Entity {
        self.world.spawn()
    }
    pub fn add_component<T: 'static>(&mut self, entity: Entity, comp: T) {
        self.world.add_component(entity, comp);
    }
    pub fn get_component<T: 'static>(&mut self, entity_id:usize) -> Option<&T>
    {
        self.world.get_component(entity_id)
    
    }
    pub fn has_system<T: System + 'static>(&self) -> bool {
        self.scheduler.has_system::<T>()
    }
    pub fn load_png(&mut self,png:&str) ->TextureHandle
    {
        self.resources.load_texture(png)
    }

    pub fn set_camera_position(&mut self, x: f32, y: f32) {
        self.camera_mut().set_position(x, y);
    }

    pub fn move_camera(&mut self, x: f32, y: f32) {
        self.camera_mut().move_by(x, y);
    }

    pub fn set_camera_zoom(&mut self, zoom: f32) {
        self.camera_mut().set_zoom(zoom);
    }

    fn camera_mut(&mut self) -> &mut Camera2D {
        self.resources
            .get_mut::<Camera2D>()
            .expect("Camera2D missing")
    }

}

