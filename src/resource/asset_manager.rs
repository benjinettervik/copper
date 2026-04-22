#![allow(unused)]
use image;
use std::collections::HashMap;
use std::default::Default;

// pub const ROOT_PATH: &str = "./src/asset/";
pub const ROOT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/asset/");

#[derive(Clone,Debug)]
pub struct AssetManager{

    pub textures: HashMap<TextureHandle,Texture>,
    pub next_tex_id: u32,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            next_tex_id: 0,
        }
    }

        pub fn get(&self, handle: TextureHandle) -> &Texture {
            self.textures
                .get(&handle)
                .expect("Invalid TextureHandle")
        }


    pub fn create_handle(&mut self) -> TextureHandle {
        let handle = TextureHandle(self.next_tex_id);
        self.next_tex_id += 1;
        handle
    }

    pub fn load_texture(&mut self, png: &str) ->TextureHandle {
        // println!("Loading texture from: {}", ROOT_PATH);
        let path = format!("{}{}", ROOT_PATH, png);
        let handle = self.create_handle();
        match Texture::convert_texture(&path) {
            Ok(tex) => {
                self.textures.insert(handle, tex);
                // self.next_tex_id +=1;
            }
            Err(err) => {
                println!("Error loading texture: {}", err);
            }
        }
        handle
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureHandle(u32);

impl TextureHandle{
    // pub fn increment{}
}


//Component
#[derive(Clone,Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA
}

impl Texture{

    pub fn convert_texture(path: &str) -> Result<Texture,String> {
        
        // println!("Loading from path: {}", path);
        let img = image::open(path)
            .map_err(|e| format!("Failed to load image: {}", e))?
            .to_rgba8();
        
        let (width, height) = img.dimensions();
        let data = img.into_raw();
        Ok(Texture {
            width,
            height,
            data,
        })
    }
    // pub fn 
}