use crate::Component;
use crate::assets::texture_map::TM_Handle;
use crate::renderer::texture::TextureHandle;
#[derive(Clone, Debug)]

pub struct MockSprite {
    pub texture: TextureHandle,
    pub map_handle: TM_Handle,
}

#[derive(Copy, Clone, Debug)]

pub struct Transform {
    pub x: f32,
    pub y: f32,
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

