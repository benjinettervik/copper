// use crate::renderer::test_components_renderer::*;
pub mod time;
pub mod config;
use crate::assets::texture_asset::TextureAsset;
use crate::assets::texture_map::TextureMap;
use crate::input::Input;
use crate::renderer::camera::Camera2D;
use crate::renderer::render_map::RenderMap;
use crate::renderer::render_queue::RenderQueue;
use crate::resource::time::Time;
use crate::resource::config::CopperConfig;
use std::any::Any;
use std::collections::HashMap;
// use crate::assets::texture_map::TextureMap;

use std::any::TypeId;

pub struct Resources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        let mut resources = Self {
            resources: HashMap::new(),
        };

        // basic tools for resources, that can be ignored if user want to
        // can also become a system
        resources.init_basic_kit();
        resources
    }

    pub fn insert<T: Any>(&mut self, value: T) {
        // typeid
        // type fingerprint
        self.resources.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>()) //Hashmap get with key -> typeid of T -- downcast ref to check if the value inside the dyn Any is actually type T
            .and_then(|v| v.downcast_ref::<T>())
    }

    // same but mutable
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }

    pub fn init_basic_kit(&mut self) {
        self.insert(RenderQueue {
            commands: Vec::new(),
        });
        self.insert(TextureAsset {
            textures: HashMap::new(),
        });
        self.insert(Camera2D::new());
        self.insert(Input::new());
        self.insert(RenderMap::new());
        self.insert(TextureMap::new());
        self.insert(Time::new());
        self.insert(CopperConfig::new());
        // self.resources.insert(Grid::new(32,32,16.0));
    }
}
