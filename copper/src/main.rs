mod test_stuff;
use copper::engine::*;
use test_stuff::*;

// rend
use copper::*;
use copper::renderer::test_components_renderer::*;
use copper::renderer::render_sys::*;
use copper::resource::camera::*;
use copper::resource::{convert_texture};
use copper::engine::world::World;
use copper::renderer::render_sys::*;
use copper::engine::system::System;
use copper::resource::{Resources,RenderLayer};
use copper::renderer::test_components_renderer::*;
use copper::rgba;
// use copper::engine::system::System;
use std::collections::HashMap;
use copper::components_with;
use copper::components_without;
use copper::components_write;
use copper::components_read;

use std::any::TypeId;
use copper::query;
use winit::keyboard::KeyCode;
use copper::input::Action;
use copper::input::Input;
use copper::input::input::*;
// use winit::keyboard::*;
// use winit::keyboard::KeyCode::*;
pub struct MySupportResources;

fn main() {
    println!("In copper-main");
    // let mut engine = Engine::new();
    // engine.test_run(16);
}

