use super::System;
use super::World;
// use crate::engine::{Startup, SystemRoutine, Update};
use std::any::Any;
use std::any::TypeId;
use crate::engine::{Startup, Update, SystemRoutine};
// use crate::engine::{Startup, Update, SystemRoutine};
use crate::resource::Resources;
use component_macro_derive::*;
use crate::Component;
use std::collections::HashMap;

pub struct AccessMappings{
    pub read_world: HashMap<TypeId, Vec<TypeId>>,
    pub write_world: HashMap<TypeId, Vec<TypeId>>,
    pub read_resources: HashMap<TypeId, Vec<TypeId>>,
    pub write_resources: HashMap<TypeId, Vec<TypeId>>,
    // 
    pub sys_reg: Vec<TypeId>, //registers what systems are active
    pub comp_reg: Vec<TypeId>, //registers what components are active
    pub res_reg: Vec<TypeId>, //what resources are required 

}
impl AccessMappings{
    pub fn new () ->Self{
        Self{
            read_world:HashMap::new(),
            write_world:HashMap::new(),
            read_resources:HashMap::new(),
            write_resources:HashMap::new(),
            sys_reg:Vec::new(),
            comp_reg:Vec::new(),
            res_reg:Vec::new(),
        }
    }
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
            println!("{:?}",system.type_id());            
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
                for component in c_read{
                    self.access_map.comp_reg.push(component);
                    self.access_map
                        .read_world
                        .entry(component)
                        .or_insert(Vec::new())
                        .push(system.type_id());
                }
            }
            
            if !c_write.is_empty() {
                for component in c_write{
                    self.access_map.comp_reg.push(component);
                    self.access_map
                        .write_world
                        .entry(component)
                        .or_insert(Vec::new())
                        .push(system.type_id());
                }
            }
            if !r_read.is_empty() {
                for resource in r_read{
                    self.access_map.res_reg.push(resource);
                    self.access_map
                        .read_resources
                        .entry(resource)
                        .or_insert(Vec::new())
                        .push(system.type_id());
                }
            }
            if !r_write.is_empty() {
                for resource in r_write{
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

    pub fn order_system(&mut self)
    {

        // debg
        println!("The amount of systems in scheduler are: {:?}",self.access_map.sys_reg.len());

        // so for order system i would simply say
        for component in self.access_map.comp_reg.clone(){
            println!("In order_system loop");
            // conflicts to detect
            let mut read_write_conflict = false;
            let mut write_write_conflict= false;
            // readers and writers
            // returned as options
            let readers = self.access_map.read_world.get(&component);
            let writers = self.access_map.write_world.get(&component);
            

            if let Some(wr) = writers{
                if wr.len()>=2 
                {
                    write_write_conflict =true;
                }
                if readers.is_some() && !wr.is_empty()
                {
                    read_write_conflict=true;
                }
            }

            println!("Read/Write conflict is: {}\nWrite/write conflict is: {}",read_write_conflict,write_write_conflict);
            // if !writers.is_empty()
            // {
            //     if !writers.
            // }
            
            // if let Some(readers) = self.access_map.read_world.get(&component)
            // {
            //     if !readers.is_empty(){
            //         println!("Readers exist");
                    
            //     }
            
            // if let Some(writers) = self.access_map.write_world.get(&component)
            // {
            //     if !writers.is_empty(){
                    
            //         if writers != none&& writers.len()>=2
            //             {
            //                 write_write_conflict = true;
            //                 if readers != None 
            //                 {
            //                     read_write_conflict = true;
            //                 }
            //             }
            //     }
            // }
            
            // }

            

              

            // // now we know that this component has a read/write conflict and can't be ran in parallel
            // println!("The following systems depend upon write: ");
            // for reader in readers {
            //     println!("{:?}",reader);

            // }
            // println!("Writers are:");
            // for writer in writers {
            //     println!("{:?}",writer);
            // }

            // // sorting
            // // dependency graph
            // // later it is
        }
    }

            
            // (!self.access_map.read_world.get(&component).is_empty())
            // {
            //     println!("Component is found in read_world");
            //     // there are systems that read the component
            //     if (!self.access_map.write_world.get(&component).is_empty())
            //     {
            //         // we now know that there are systems that write to a component, and the systems that read this rely on the write_systems.
            //         println!("Found even more!");
            //     }

    //  

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

