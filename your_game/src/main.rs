use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::renderer::render_sys::RenderSys;
use copper::resource::Resources;
use copper::*;
use winit::event::KeyEvent;
use winit::window;
use std::any::TypeId;
use component_macro_derive::*;

use copper::input::input::*;
use winit::keyboard::*;
use winit::keyboard::KeyCode::*;



struct SpawnSnakeSystem;
impl System for SpawnSnakeSystem {
    components_read!();
    components_write!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        
    }
}


fn main() {
    let mut engine = Engine::new();

    println!("LOL");

    engine.run_cycles(5);
}

// cargo run --bin your_game