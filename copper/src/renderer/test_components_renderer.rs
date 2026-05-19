// pub mod test_components_renderer;
use std::collections::HashMap;
use crate::Component;
use crate::resource::TM_Handle;

// just a macro to quickly provide rgba pixel data 
#[macro_export]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr, $h:expr, $w:expr) => {{
        let mut data = Vec::new();

        for _ in 0..$h {
            for _ in 0..$w {
                data.extend_from_slice(&[
                    $r as u8,
                    $g as u8,
                    $b as u8,
                    $a as u8,
                ]);
            }
        }
        data
    }};
}


#[derive(Copy, Clone,Debug,Eq,Hash,PartialEq)]
pub struct TextureHandle(pub i32);

#[derive(Clone)]
pub struct TextureAsset {
    pub textures: HashMap<TextureHandle, Texture>,
}

#[derive(Clone,Debug)]
    
pub struct MockSprite{
    pub texture: TextureHandle,
    pub map_handle: TM_Handle
}
    
#[derive(Copy, Clone,Debug)]

pub struct Transform{
    pub x: f32,
    pub y: f32,
}
#[derive(Debug,Hash,Clone)]
pub struct Texture{
    pub width: u32,
    pub height: u32,
    pub pixel_data: Vec<u8>, //rgba 0-255 
}

impl Component for Texture {
    fn name(&self) -> &str {
            "Texture"
    }
}
impl Component for TextureHandle {
    fn name(&self) -> &str {
        "TextureHandle"
    }
}

impl Component for MockSprite {
    fn name(&self) -> &str {
        "MockSprite"
    }
}

impl Component for Transform {
    fn name(&self) -> &str {
        "Transform"
    }
}

