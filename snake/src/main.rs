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
use winit::event::MouseButton::*;



#[derive(Component)]
enum Orientation {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Component)]
struct TransformComponent {
    pos_x: i32,
    pos_y: i32,
}


#[derive(Component)]
struct SnakeComponent {
    length: usize,
    orientation: Orientation,
}

struct SpawnSnakeSystem;
impl System for SpawnSnakeSystem {
    components_read!();
    components_write!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let snake = world.spawn();
        world.add_component(snake, TransformComponent { pos_x: 0, pos_y: 0 })
        .add_component(
            snake,
            SnakeComponent {
                length: 1,
                orientation: Orientation::Up,
            },
        )
        .add_component(snake, InputState::new());


        println!("Snake initialized.");
    }
}

struct MoveSnakeSystem;
impl System for MoveSnakeSystem {
    components_read!(SnakeComponent);
    components_write!(TransformComponent, InputState);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let snake = query!(self, world)
            .first()
            .unwrap()
            .clone();

        if resources.input_state().is_key_pressed(KeyW) {
            println!("W");
        }
        if resources.input_state().is_key_pressed(KeyA) {
            println!("A");
        }
        if resources.input_state().is_key_pressed(KeyD) {
            println!("D");
        }
        if resources.input_state().is_key_pressed(KeyS) {
            println!("S");
        }
        if resources.input_state().is_key_pressed(KeyE) {
            println!("E");
        }
        if resources.input_state().is_mouse_pressed(Left) {
            println!("Left");
        }
        if resources.input_state().is_mouse_pressed(Right) {
            println!("Right");
        }
        if resources.input_state().is_mouse_pressed(Left) {
            print!("{:?}", resources.input_state().mouse_pos)
            
        }
    }
}


fn main() {
    let mut engine = Engine::new();

    // Startup
    engine.add_system(Startup, SpawnSnakeSystem);

    // Update
    engine.add_system(Update, MoveSnakeSystem)
    .add_system(Update, RenderSys);

    engine.resources.get_mut::<Input>().unwrap().binds
    .bind_key(KeyCode::KeyW, Action::Up)
    .bind_key(KeyCode::KeyS, Action::Down)
    .bind_key(KeyCode::KeyA, Action::Left)
    .bind_key(KeyCode::KeyD, Action::Right);

    engine.test_run();
}
