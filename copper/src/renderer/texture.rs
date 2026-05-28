use crate::Component;
#[derive(Copy, Clone,Debug,Eq,Hash,PartialEq)]
pub struct TextureHandle(pub i32);
#[derive(Debug,Hash,Clone)]
pub struct Texture{
    pub width: u32,
    pub height: u32,
    pub pixel_data: Vec<u32>, //rgba 0-255 
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