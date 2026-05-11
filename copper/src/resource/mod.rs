// use copper::renderer::test_components_renderer::*;
pub mod camera;
use crate::renderer::test_components_renderer::*;
use crate::resource::camera::*;
use crate::renderer::render_sys::*;
use crate::input::Input;
use std::collections::HashMap;
use std::any::Any;
use std::any::TypeId;
use crate::input::InputState;



// pub struct Resources {
//     pub texture_hash: TextureAsset,
//     pub render_queue: RenderQueue,
//     pub Camera2D: Camera2D,
//     pub input: Input,
// }

pub struct Resources{
    // key becomes the type of the resource that has been saved , and the box contains on 
    // heap the accessible data associated.
    // Box<T> is a smart pointer -- stores variable on heap instead of directly on stack 
    // dyn Any , unknown size at compile time 
    resources: HashMap<TypeId, Box<dyn Any>>,

}

impl Resources{
    pub fn new() -> Self {

        let mut resources = Self {
            resources: HashMap::new(),
        };

        // basic tools for resources, that can be ignored if user want to
        // can also become a system
        resources.init_basic_kit();
        resources
    }

    pub fn input_state(&mut self) -> &InputState{
        let input = self.get::<Input>().unwrap(); //FIXME remove unwrap
        &input.state
    }

    pub fn insert<T:Any>(&mut self, value: T){
        // typeid
        // type fingerprint
        self.resources.insert(TypeId::of::<T>(),Box::new(value));
    }

    pub fn get<T: Any>(&self) -> Option<&T>
    {
        self.resources
            .get(&TypeId::of::<T>()) //Hashmap get with key -> typeid of T -- downcast ref to check if the value inside the dyn Any is actually type T
            .and_then(|v| v.downcast_ref::<T>())
    }

    // same but mutable
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T>
    {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }

    pub fn init_basic_kit(&mut self){
        self.insert(RenderQueue{commands: Vec::new(),});
        self.insert(TextureAsset{textures: HashMap::new(),});
        self.insert(Camera2D::new());
        self.insert(Input::new());
        // self.resources.insert(Grid::new(32,32,16.0));
    }
}















#[derive(Debug)]
pub struct RenderCommand {
    pub texture: TextureHandle,
    pub x: f32,
    pub y: f32,
}

pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
}



pub fn convert_texture(path: &str) -> Result<Texture,String> {
        

        let img = image::open(path)
            .map_err(|e| format!("Failed to load image: {}", e))?
            .to_rgba8();
        
        let (width, height) = img.dimensions();
        let pixel_data = img.into_raw();
        Ok(Texture {
            width,
            height,
            pixel_data,
        })
    }
// camera sys

