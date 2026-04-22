#![allow(unused)]
use crate::resource::renderer::Renderer;
use crate::resource::asset_manager::AssetManager;
use crate::resource::asset_manager::{TextureHandle};
use std::sync::Arc;
use winit::window::WindowBuilder;
use winit::event_loop::EventLoop;
use std::any::Any;
use std::collections::HashMap;
use std::any::TypeId;

pub struct Resources{
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        let mut res = Resources{
            data: HashMap::new()
        };
        res.init_am();
        res
        
    }
    pub fn insert<T: 'static>(&mut self, resource: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(resource));
    }
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }
    
    pub fn load_texture(&mut self, png: &str)->TextureHandle {
    if let Some(am) = self.get_mut::<AssetManager>() {
        let handle = am.load_texture(png);
        handle
    } else {
        panic!("AssetManager not initialized");
    }
}
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
    self.data
        .remove(&TypeId::of::<T>())
        .and_then(|boxed| boxed.downcast::<T>().ok())
        .map(|boxed| *boxed)
}

    pub fn init_am(&mut self){
        self.insert(AssetManager::new());
    }

}

