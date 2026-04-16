pub mod system;
pub mod world;

use system::*;
use world::*;

pub struct Engine {
    world: World,
    systems: Vec<Box<dyn System>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            systems: vec![], // perhaps add default systems here, rendering, etc
        }
    }

    pub fn register_system<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&self, cycles: usize) {
        for system in &self.systems {
            system._on_ready(self.world, //);
        }

        for _ in 0..cycles {
            for system in &self.systems {
                system._process();
            }
        }
    }
}
