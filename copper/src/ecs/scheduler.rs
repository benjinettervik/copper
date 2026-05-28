use crate::ecs::system::System;
use crate::ecs::world::World;
use rayon;
// use crate::engine::{Startup, SystemRoutine, Update};
use crate::core::engine::{Startup, SystemRoutine, Update};
use std::any::Any;
use std::any::TypeId;
// use crate::engine::{Startup, Update, SystemRoutine};
// use crate::Component;
use crate::resource::Resources;
// use component_macro_derive::*;
use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread;

pub struct AccessMappings {
    pub read_world: HashMap<TypeId, Vec<TypeId>>,
    pub write_world: HashMap<TypeId, Vec<TypeId>>,
    pub read_resources: HashMap<TypeId, Vec<TypeId>>,
    pub write_resources: HashMap<TypeId, Vec<TypeId>>,
    //
    pub sys_reg: Vec<TypeId>,  //registers what systems are active
    pub comp_reg: Vec<TypeId>, //registers what components are active
    pub res_reg: Vec<TypeId>,  //what resources are required
}
impl AccessMappings {
    pub fn new() -> Self {
        Self {
            read_world: HashMap::new(),
            write_world: HashMap::new(),
            read_resources: HashMap::new(),
            write_resources: HashMap::new(),
            sys_reg: Vec::new(),
            comp_reg: Vec::new(),
            res_reg: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SchedulerDepGraph {
    dep: Vec<SchedulerDepNode>,
    len: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SchedulerDepNode {
    id: TypeId,
    dependencies: Vec<TypeId>,
}

pub struct Scheduler {
    startup: Vec<Box<dyn System>>,
    update: Vec<Box<dyn System>>,
    access_map: AccessMappings,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            startup: Vec::new(),
            update: Vec::new(),
            access_map: AccessMappings::new(),
            // sorted ?
            // run_update runs sorted?
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
            // from this we have access to the system type and its metadata
            //println!("{:?}", system.type_id());
            let c_read = system.components_read();
            let c_write = system.components_write();
            let r_read = system.resources_read();
            let r_write = system.resources_write();

            // register mappings:
            if !self.access_map.sys_reg.contains(&system.type_id()) {
                self.access_map.sys_reg.push(system.type_id());
            }
            // read write shit
            if !c_read.is_empty() {
                for component in c_read {
                    self.access_map.comp_reg.push(component);
                    self.access_map
                        .read_world
                        .entry(component)
                        .or_insert(Vec::new())
                        .push(system.type_id());
                }
            }

            if !c_write.is_empty() {
                for component in c_write {
                    self.access_map.comp_reg.push(component);
                    self.access_map
                        .write_world
                        .entry(component)
                        .or_insert(Vec::new())
                        .push(system.type_id());
                }
            }
            if !r_read.is_empty() {
                for resource in r_read {
                    self.access_map.res_reg.push(resource);
                    self.access_map
                        .read_resources
                        .entry(resource)
                        .or_insert(Vec::new())
                        .push(system.type_id());
                }
            }
            if !r_write.is_empty() {
                for resource in r_write {
                    self.access_map.res_reg.push(resource);
                    self.access_map
                        .write_resources
                        .entry(resource)
                        .or_insert(Vec::new())
                        .push(system.type_id());
                }
            }
            self.add_update_system(system);
            return;
        }

        panic!("Custom system routines not yet implemented!");
    }

    pub fn make_dep_graph(&mut self) -> SchedulerDepGraph {
        // debg
        // println!(
        //     "The amount of systems in scheduler are: {:?}",
        //     self.access_map.sys_reg.len()
        // );

        // so for order system i would simply say
        // let mut dep_graph = SchedulerDepGraph{dep:Vec::new(),len:0};
        let mut depgraph = SchedulerDepGraph {
            dep: Vec::new(),
            len: 0,
        };
        for component in self.access_map.comp_reg.clone() {
            // println!("In order_system loop");
            // conflicts to detect
            let mut read_write_conflict = false;
            let mut write_write_conflict = false;
            // readers and writers
            // returned as options
            let readers = self.access_map.read_world.get(&component);
            let writers = self.access_map.write_world.get(&component);

            if let Some(wr) = writers {
                if readers.is_some() && !wr.is_empty() {
                    read_write_conflict = true;
                    let reader_vec = readers.unwrap();
                    let writer_vec = writers.unwrap();
                    for dep_reader in reader_vec {
                        // let depnode_exists = depgraph.dep.iter().any(|node| node.id == *dep_reader);

                        if let Some(depnode) =
                            depgraph.dep.iter_mut().find(|node| node.id == *dep_reader)
                        {
                            for each in writer_vec {
                                if depnode.dependencies.contains(each) {
                                    continue;
                                }
                                depnode.dependencies.push(*each);
                            }
                        } else {
                            // println!("Depnode does not exist!");
                            let mut read_depend = Vec::new();
                            for each in writer_vec {
                                read_depend.push(*each);
                            }
                            depgraph.dep.push(SchedulerDepNode {
                                id: *dep_reader,
                                dependencies: read_depend,
                            });
                            depgraph.len += 1;
                        }
                    }
                }
            }

            // println!("Read/Write conflict is: {}\nWrite/write conflict is: {}",read_write_conflict,write_write_conflict);
        }

        let system_vec = self.access_map.sys_reg.clone();
        for sys in system_vec {
            if !depgraph.dep.iter().any(|node| node.id == sys) {
                depgraph.dep.push(SchedulerDepNode {
                    id: sys,
                    dependencies: Vec::new(),
                });
                depgraph.len += 1;
            }
        }
        return depgraph;
    }

    pub fn sort_systems(&mut self, dep_graph: SchedulerDepGraph) -> Vec<Vec<TypeId>> {
        // println!("Depgraph we are working with: {:?}\n", dep_graph);
        let mut result = Vec::new();
        let mut dgraph = dep_graph.clone();
        // let mut node_vec = dep_graph.dep.clone();
        while !dgraph.dep.is_empty() {
            let mut stage = Vec::new();
            // no dependencies --> lets push and remove

            for dep in dgraph.dep.iter() {
                if dep.dependencies.is_empty() {
                    let mut compatible = true;
                    for existing in &stage {
                        if self.conflicting_systems(dep.id, *existing) {
                            compatible = false;
                            break;
                        }
                    }
                    if compatible {
                        stage.push(dep.id);
                    }
                }
            }
            if stage.is_empty() {
                panic!("Cyclic dependency detected");
            }
            dgraph.dep.retain(|node| !stage.contains(&node.id));

            // println!("sorting systems loop");
            for node in dgraph.dep.iter_mut() {
                // now remove it from dependencies
                node.dependencies.retain(|dep| !stage.contains(dep));
            }
            result.push(stage);
        }
        // println!("Systems was sorted: \n{:?}\n", result);
        result
    }
    pub fn conflicting_systems(&self, a: TypeId, b: TypeId) -> bool {
        for key in self.access_map.comp_reg.iter() {
            let readers = self.access_map.read_world.get(key);
            let writers = self.access_map.write_world.get(key);

            let a_reads = readers.is_some_and(|v| v.contains(&a));
            let a_writes = writers.is_some_and(|v| v.contains(&a));

            let b_reads = readers.is_some_and(|v| v.contains(&b));
            let b_writes = writers.is_some_and(|v| v.contains(&b));

            if (a_writes && b_reads) || (a_reads && b_writes) || (a_writes && b_writes) {
                return true;
            }
        }

        false
    }

    pub fn set_order(&mut self, stages: Vec<Vec<TypeId>>) {
        // println!("setting the order given now");
    }

    pub fn add_startup_system<T: System + 'static>(&mut self, system: T) {
        self.startup.push(Box::new(system));
    }

    pub fn add_update_system<T: System + 'static>(&mut self, system: T) {
        self.update.push(Box::new(system));
    }

    pub fn run_startup(&mut self, world: &mut World, resources: &mut Resources) {
        for system in &mut self.startup {
            system.run(world, resources);
        }
    }

    pub fn run_update(&mut self, world: &mut World, resources: &mut Resources) {
        for system in &mut self.update {
            system.run(world, resources);
        }
    }

    /*pub fn run_prio_update(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        order: &Vec<Vec<TypeId>>,
    ) {
        let mut fixed_update: Vec<Box<dyn System>> = Vec::new();
        let mut order_batch = order.clone();
        // println!("The order batch \n {:?}",order_batch);
        for stage in order_batch {
            for system in stage {
                if let Some(system) = self.update.iter_mut().find(|sys| sys.sys_id() == system) {
                    // have to wrap in Arc for threading

                    system.run(world, resources);

                    println!("Running ID: {:?}\n", system.sys_id());
                    // now its time to fix the concurrency part
                }
            }
        }
    }*/

    pub fn run_prio_update(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        order: &Vec<Vec<TypeId>>,
    ) {
        let order_batch = order.clone();
        //println!("\n In Benjamin threading update\n");

        let world_ptr = world as *mut World as usize; // convert world to hard pointer
        let res_ptr = resources as *mut Resources as usize; // convert resource to hard pointer

        for stage in order_batch {
            let mut stage_systems: Vec<usize> = Vec::new();

            for system_id in stage {
                if let Some(system) = self.update.iter_mut().find(|sys| sys.sys_id() == system_id) {
                    let sys_ptr = system as *mut Box<dyn System> as usize; // convert system to
                    // hard pointer
                    stage_systems.push(sys_ptr);
                }
            }

            rayon::scope(|s| {
                for sys_ptr in stage_systems {
                    // Spawn the actual thread
                    s.spawn(move |_| {
                        // 3. UNSAFE BLOCK: Reconstruct the mutable references
                        unsafe {
                            // Cast the numbers back to raw pointers, then deference them into &mut
                            let thread_world = &mut *(world_ptr as *mut World);
                            let thread_res = &mut *(res_ptr as *mut Resources);
                            let thread_sys = &mut *(sys_ptr as *mut Box<dyn System>);

                            // Run the system concurrently!
                            thread_sys.run(thread_world, thread_res);

                            //println!("Running ID: {:?}\n", thread_sys.sys_id());
                        }
                    });
                }
            });
        }
    }
}
