use crate::engine::world::World;
use crate::engine::system::System;
use crate::resource::resources::Resources;
use std::any::{Any, TypeId};

pub struct Scheduler {
    startup: Vec<Box<dyn System>>,
    update: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            startup: Vec::new(),
            update: Vec::new(),
        }
    }

    pub fn add_startup_system<T: System + 'static>(&mut self, system: T) {
        self.startup.push(Box::new(system));
    }

    pub fn add_update_system<T: System + 'static>(&mut self, system: T) {
        self.update.push(Box::new(system));
    }

    pub fn run_startup(&mut self, world: &mut World, _resources: &mut Resources) {
        
        for system in &mut self.startup {
            system.run(world,_resources);
            // system.run(world);
            println!("For testing: Running startup systems");
        }
    }

    pub fn run_update(&mut self, world: &mut World, _resources: &mut Resources) {
        for system in &mut self.update {
            system.run(world,_resources);
        }
    }

    pub fn has_system<T: System + 'static>(&self) -> bool {
        let target = TypeId::of::<T>();

        self.startup.iter().chain(self.update.iter()).any(|sys| {
            sys.as_ref().type_id() == target
        })
    }
}