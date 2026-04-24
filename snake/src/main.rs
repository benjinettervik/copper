use copper::engine::system::*;
use copper::engine::world::*;
use copper::engine::*;
use copper::resource::Resources;
use copper::*;
use std::any::TypeId;

enum Orientation {
    Left,
    Up,
    Right,
    Down,
}

struct TransformComponent {
    pos_x: i32,
    pos_y: i32,
}

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
        world.add_component(snake, TransformComponent { pos_x: 0, pos_y: 0 });
        world.add_component(
            snake,
            SnakeComponent {
                length: 1,
                orientation: Orientation::Up,
            },
        );
    }
}

struct MoveSnakeSystem;
impl System for MoveSnakeSystem {
    components_read!(SnakeComponent);
    components_write!(TransformComponent);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let snake = world
            .query(
                &self.components_read(),
                &self.components_write(),
                &self.components_with(),
                &self.components_without(),
            )
            .first()
            .unwrap()
            .clone();

        let mut transform = world.get_component_mut::<TransformComponent>(snake);

        // osv osv
    }
}

fn main() {
    println!("Hello, world!");

    let mut engine = Engine::new();

    engine.add_system(Startup, SpawnSnakeSystem);
    engine.add_system(Update, MoveSnakeSystem);
}
