// pub mod test_components_renderer;
use std::collections::HashMap;
use crate::Component;
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

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct TextureHandle(pub i32);
    pub struct TextureAsset {
        pub textures: HashMap<TextureHandle, Texture>,
    }

    pub struct MockSprite{
        pub texture: TextureHandle,
    }

    pub struct Transform{
        pub x: f32,
        pub y: f32,
    }

        pub struct Texture{
        pub width: u32,
        pub height: u32,
        pub pixel_data: Vec<u8>, //rgba 0-255 
    }
    pub struct RenderSystem2D{
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

// later import 

//     use copper::engine::world::World;
//     use std::collections::HashMap;
//     use std::sync::Arc;

//     use winit::{
//     event::{Event, WindowEvent},
//     event_loop::EventLoop,
//     window::WindowBuilder,
// };
//     use pixels::{Pixels, SurfaceTexture};
//     use std::ops::ControlFlow;