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

#[derive(Debug)]
pub struct SystemDepGraph
{
	dep: Vec<SystemDepNode>,
	len: u32,
}

#[derive(PartialEq,Debug)]
pub struct SystemDepNode
{
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

    pub fn make_dep_graph(&mut self) -> SystemDepGraph
    {

        // debg
        println!("The amount of systems in scheduler are: {:?}",self.access_map.sys_reg.len());

        // so for order system i would simply say
        // let mut dep_graph = SystemDepGraph{dep:Vec::new(),len:0};
        let mut depgraph = SystemDepGraph {dep:Vec::new(),len:0};
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
                    let writer_vec = writers.unwrap();

                    for dep_writer in writer_vec{
                        if let Some(depnode) = depgraph.dep.iter_mut().find(|node| node.id == *dep_writer) {
                            for each in writer_vec{
                                if each == dep_writer{
                                    continue;
                                }
                                else{
                                    if depnode.dependencies.contains(each)
                                    {
                                        continue;
                                    }
                                    depnode.dependencies.push(*each);
                                }
                            }
                        }
                        else
                        {
                            println!("Depnode does not exist!");
                            let mut write_depend = Vec::new();
                            for each in writer_vec{
                                if each == dep_writer{
                                    continue;
                                }
                                write_depend.push(*each);
                            }
                            depgraph.dep.push(SystemDepNode{id:*dep_writer,dependencies:write_depend}); 
                            depgraph.len+=1;
                        }
                    }
                }
                if readers.is_some() && !wr.is_empty()
                {
                    read_write_conflict=true;
                    let reader_vec = readers.unwrap();
                    let writer_vec = writers.unwrap();
                    for dep_reader in reader_vec{
                        
                        // let depnode_exists = depgraph.dep.iter().any(|node| node.id == *dep_reader);         
                        
                        if let Some(depnode) = depgraph.dep.iter_mut().find(|node| node.id == *dep_reader) {
                            
                            for each in writer_vec{
                                if depnode.dependencies.contains(each)
                                {
                                    continue;
                                }
                                depnode.dependencies.push(*each);
                            }
                        }
                        else
                        {
                            println!("Depnode does not exist!");
                            let mut read_depend = Vec::new();
                            for each in writer_vec{
                                read_depend.push(*each);
                            }
                            depgraph.dep.push(SystemDepNode{id:*dep_reader,dependencies:read_depend}); 
                            depgraph.len+=1;
                        }
                    }
                }
            }

            // println!("Read/Write conflict is: {}\nWrite/write conflict is: {}",read_write_conflict,write_write_conflict);
            
        }
        let system_vec = self.access_map.sys_reg.clone();

        for sys in system_vec 
        {
            if !depgraph.dep.iter().any(|node| node.id == sys)
            {
                depgraph.dep.push(SystemDepNode{id:sys, dependencies:Vec::new()});
                depgraph.len+=1;
            }
        }
        return depgraph;            
    }

    pub fn sort_systems(&mut self, dep_graph:SystemDepGraph)
    {
        // top sort of the depgrahp provided?
        println!("We are now going to sort the dependency graph");
        // so still a top sort , kahns algo basically.
        // collect all w/o dependencies, since they can be run in parallell
        // remove dependencies, do the same for the next stage, until no systems remain in the queue
        // this does not entirely help cyclic behaviour i think.
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

