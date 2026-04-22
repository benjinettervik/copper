use crate::engine::world::World;
use crate::engine::system::System;
use crate::resource::resources::Resources;
use crate::engine::meta::SystemMeta;
use std::any::{Any, TypeId};
use std::thread;

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

    // pub fn test_sys()
    pub fn has_system<T: System + 'static>(&self) -> bool {
        let target = TypeId::of::<T>();

        if self.startup.iter().chain(self.update.iter()).any(|sys| {
            sys.as_ref().type_id() == target
        })
        {
            return true
        }
        else if self.update.iter().chain(self.update.iter()).any(|sys| {
            sys.as_ref().type_id() == target
        })
        {
            return true
        }
        false
    }

    // example of threading
    // pub fn run_thread() -> i32 {
    //     let t1 = thread::spawn(|| {
    //         println!("Hello from thread 1!");
    //         1
    //     });

    //     let t2 = thread::spawn(|| {
    //         println!("Hello from thread 2!");
    //         1
    //     });

    //     let r1 = t1.join().unwrap();
    //     let r2 = t2.join().unwrap();

    //     println!("Main thread done!");

    //     r1 + r2
    // }

    pub fn collect_update_meta(&self) -> Vec<SystemMeta> {
        let mut metas = Vec::new();

            for system in &self.update {
                let mut meta = SystemMeta {
                    reads: Default::default(),
                    writes: Default::default(),
                    resource_reads: Default::default(),
                    resource_writes: Default::default(),
                };
                
                system.meta(&mut meta);
                metas.push(meta);
            }

            metas
    }
    pub fn collect_startup_meta(&self) -> Vec<SystemMeta> {
        let mut metas = Vec::new();

            for system in &self.startup {
                let mut meta = SystemMeta {
                    reads: Default::default(),
                    writes: Default::default(),
                    resource_reads: Default::default(),
                    resource_writes: Default::default(),
                };

                system.meta(&mut meta);
                metas.push(meta);
            }

            metas
        }
}