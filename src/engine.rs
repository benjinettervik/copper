pub mod system;
pub mod world;
pub mod scheduler;

use system::*;
use world::*;
use scheduler::*;

pub struct Engine {
    world: World,
    pub scheduler: Scheduler,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn run_cycles(&mut self, cycles: usize) {
        self.scheduler.run_startup(&mut self.world);

        for _ in 0..cycles {
            self.scheduler.run_update(&mut self.world);
        }
    }
}
