use std::any::TypeId;
use std::time::{Duration, Instant};

use copper::engine::system::System;
use copper::engine::world::World;
use copper::engine::{Engine, Startup, Update};
use copper::grid::{Grid, GridPosition};
use copper::input::input::Input;
use copper::renderer::render_sys::RenderSys;
use copper::renderer::test_components_renderer::{
    MockSprite, Texture, TextureAsset, TextureHandle, Transform,
};
use copper::resource::Resources;
use copper::{
    EntityId, components_read, components_with, components_without, components_write, rgba,
};
use rand::RngExt;
use winit::keyboard::KeyCode;

const W: usize = 24;
const H: usize = 18;
const CELL: f32 = 30.0;
const STEP: Duration = Duration::from_millis(100);
const SNAKE: TextureHandle = TextureHandle(1);
const FOOD: TextureHandle = TextureHandle(2);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

struct SnakeGame {
    snake: EntityId,
    food: EntityId,
    pos: GridPosition,
    food_pos: GridPosition,
    dir: Direction,
    last: Instant,
    score: usize,
}

struct SetupSnakeSystem;
impl System for SetupSnakeSystem {
    components_read!();
    components_write!(GridPosition, Transform, MockSprite);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        resources
            .get_mut::<TextureAsset>()
            .unwrap()
            .textures
            .extend([
                (SNAKE, texture([90, 220, 105, 255])),
                (FOOD, texture([220, 50, 48, 255])),
            ]);
        resources.insert(Grid::new(W, H, CELL));

        let start = GridPosition::new(W / 2, H / 2);
        let food_pos = random_position(resources);
        let game = SnakeGame {
            snake: spawn(world, SNAKE),
            food: spawn(world, FOOD),
            pos: start,
            food_pos,
            dir: Direction::Right,
            last: Instant::now(),
            score: 0,
        };

        place(world, resources, game.snake, game.pos);
        place(world, resources, game.food, game.food_pos);
        resources.insert(game);
        println!("Snake started. Use WASD. Score: 0");
    }
}

struct MoveSnakeSystem;
impl System for MoveSnakeSystem {
    components_read!();
    components_write!(GridPosition, Transform);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        if let Some(dir) = pressed_direction(resources.get::<Input>().unwrap()) {
            resources.get_mut::<SnakeGame>().unwrap().dir = dir;
        }

        let (snake, food, pos, food_pos, dir) = {
            let game = resources.get::<SnakeGame>().unwrap();
            if game.last.elapsed() < STEP {
                return;
            }
            (game.snake, game.food, game.pos, game.food_pos, game.dir)
        };

        let Some(pos) = moved(pos, dir) else {
            return reset(world, resources);
        };
        if !resources.get::<Grid>().unwrap().contains_position(pos) {
            return reset(world, resources);
        }

        let ate = pos == food_pos;
        let food_pos = if ate {
            random_position(resources)
        } else {
            food_pos
        };

        {
            let game = resources.get_mut::<SnakeGame>().unwrap();
            game.pos = pos;
            game.food_pos = food_pos;
            game.last = Instant::now();
            if ate {
                game.score += 1;
                println!("Score: {}", game.score);
            }
        }

        place(world, resources, snake, pos);
        place(world, resources, food, food_pos);
    }
}

fn spawn(world: &mut World, texture: TextureHandle) -> EntityId {
    let entity = world.spawn();
    world
        .add_component(entity, GridPosition::new(0, 0))
        .add_component(entity, Transform { x: 0.0, y: 0.0 })
        .add_component(entity, MockSprite { texture });
    entity
}

fn pressed_direction(input: &Input) -> Option<Direction> {
    if input.state.is_key_down(KeyCode::KeyW) {
        Some(Direction::Up)
    } else if input.state.is_key_down(KeyCode::KeyS) {
        Some(Direction::Down)
    } else if input.state.is_key_down(KeyCode::KeyA) {
        Some(Direction::Left)
    } else if input.state.is_key_down(KeyCode::KeyD) {
        Some(Direction::Right)
    } else {
        None
    }
}

fn reset(world: &mut World, resources: &mut Resources) {
    println!("Game over.");
    let start = GridPosition::new(W / 2, H / 2);
    let food_pos = random_position(resources);
    let (snake, food) = {
        let game = resources.get_mut::<SnakeGame>().unwrap();
        game.pos = start;
        game.food_pos = food_pos;
        game.dir = Direction::Right;
        game.last = Instant::now();
        game.score = 0;
        (game.snake, game.food)
    };
    place(world, resources, snake, start);
    place(world, resources, food, food_pos);
    println!("Score: 0");
}

fn moved(pos: GridPosition, dir: Direction) -> Option<GridPosition> {
    let (dx, dy) = dir.delta();
    let x = pos.x as i32 + dx;
    let y = pos.y as i32 + dy;

    if x < 0 || y < 0 {
        None
    } else {
        Some(GridPosition::new(x as usize, y as usize))
    }
}

fn random_position(resources: &Resources) -> GridPosition {
    let grid = resources.get::<Grid>().unwrap();
    let mut rng = rand::rng();
    GridPosition::new(
        rng.random_range(0..grid.width()),
        rng.random_range(0..grid.height()),
    )
}

fn place(world: &mut World, resources: &Resources, entity: EntityId, pos: GridPosition) {
    let grid = resources.get::<Grid>().unwrap();
    let (x, y) = grid.grid_to_world(pos.x, pos.y).unwrap();
    *world.get_component_mut::<GridPosition>(entity).unwrap() = pos;
    *world.get_component_mut::<Transform>(entity).unwrap() = Transform { x, y };
}

fn texture(color: [u8; 4]) -> Texture {
    Texture {
        width: 22,
        height: 22,
        pixel_data: rgba!(color[0], color[1], color[2], color[3], 22, 22),
    }
}

fn main() {
    let mut engine = Engine::new();
    engine
        .add_system(Startup, SetupSnakeSystem)
        .add_system(Update, MoveSnakeSystem)
        .add_system(Update, RenderSys);
    engine.test_run();
}
