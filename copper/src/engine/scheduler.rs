use super::System;
use super::World;
// use crate::engine::{Startup, SystemRoutine, Update};
use std::any::Any;
use std::any::TypeId;
use crate::engine::{Startup, Update, SystemRoutine};
use crate::resource::Resources;

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

    pub fn add_system<T1, T2>(&mut self, system_routine: T1, system: T2)
    where
        T1: SystemRoutine + 'static,
        T2: System + 'static,
    {
        if system_routine.type_id() == TypeId::of::<Startup>() {
            self.add_startup_system(system);
            return;
        } else if system_routine.type_id() == TypeId::of::<Update>() {
            self.add_update_system(system);
            return;
        }

        panic!("Custom system routines not yet implemented!");
    }

    pub fn add_startup_system<T: System + 'static>(&mut self, system: T) {
        self.startup.push(Box::new(system));
    }

    pub fn add_update_system<T: System + 'static>(&mut self, system: T) {
        self.update.push(Box::new(system));
    }

    pub fn run_startup(&mut self, world: &mut World, resources: &mut Resources) {
        for system in &mut self.startup {
            system.run(world,resources);
        }
    }

    pub fn run_update(&mut self, world: &mut World,resources: &mut Resources) {
        for system in &mut self.update {
            system.run(world,resources);
        }
    }
}

