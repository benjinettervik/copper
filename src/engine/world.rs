/*

struct world:
* id (incrementing)
* Vec<Entities>


impl:
* new
* spawn
* despawn
* add_component
* query
*/
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
// use super::entity::*;
use crate::engine::system::Entity;

pub struct World {
    next_entity_id: usize,
    component_storages: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            component_storages: HashMap::<TypeId, Box<dyn Any>>::new(),
        }
    }

    pub fn spawn(&mut self) -> usize {
        let return_id = self.next_entity_id;
        self.next_entity_id += 1;

        // TODO: Might want to save which entities are currently alive if we want to be able to delete entities later 

        return_id
    }

    /// Adds a component onto an entity. Adds any new components into the world's storage. 
    pub fn add_component<T: 'static>(&mut self, entity_id: usize, component: T) {
        // Generates a unique ID based on a 'static component struct type. 
        // Example: every unique component of type 'struct Player' will generate the same component_id!
        let component_id = TypeId::of::<T>();

        // Gets the entry with the right component id.
        // If an entry does not exist, create one for component_id.
        let component_storage_box = self.component_storages
                .entry(component_id)
                .or_insert_with(|| Box::new(HashMap::<usize, T>::new()));
        
        // 'component_storage' in this case is a hashmap that stores which entities who has the 
        // 'component_storage' specific component assigned to it and:
        // - KEY is the unique ID of an entity.
        // - VALUE is the unique component data storage for that entity

        // Downcast component_storage_box
        let component_storage = component_storage_box.downcast_mut::<HashMap<usize, T>>()
            .expect("FATAL ERROR: This should not happen... ");

        component_storage.insert(entity_id, component);
    }


    /// Gets a reference to a component that is assigned to entity_id
    pub fn get_component<T: 'static>(&self, entity_id: usize) -> Option<&T>{
        self.component_storages.get(&TypeId::of::<T>())?
        .downcast_ref::<HashMap<usize, T>>()?
        .get(&entity_id)
    }

    /// Gets a mutable reference to a component that is assigned to entity_id
    pub fn get_component_mut<T: 'static>(&mut self, entity_id: usize) -> Option<&mut T> {
        self.component_storages.get_mut(&TypeId::of::<T>())?
        .downcast_mut::<HashMap<usize, T>>()?
        .get_mut(&entity_id)
    }

    

    // pub fn query <T: 'static>(&self) -> Vec<(Entity,&T)> 
    // {        
    // }

    pub fn query_mut<T: 'static>(&mut self) -> Vec<(Entity, &mut T)> {
        let mut result = Vec::new();

        if let Some(storage_box) = self.component_storages.get_mut(&TypeId::of::<T>()) {
            let storage = storage_box
                .downcast_mut::<HashMap<usize, T>>()
                .expect("Storage type mismatch");

            for (entity, comp) in storage.iter_mut() {
                result.push((*entity, comp));
            }
        }

        result
    }
    pub fn query<T: 'static>(&self) -> Vec<(Entity, &T)> {
    let mut result = Vec::new();

    if let Some(storage_box) = self.component_storages.get(&TypeId::of::<T>()) {
        let storage = storage_box
            .downcast_ref::<HashMap<usize, T>>()
            .expect("Storage type mismatch");

        for (entity, comp) in storage.iter() {
            result.push((*entity, comp));
        }
    }
        result
    }
    

    
    pub fn query2_mut<A: 'static, B: 'static, F>(&mut self, mut f: F)
        where
            F: FnMut(Entity, &A, &mut B),
        {
            let type_a = TypeId::of::<A>();
            let type_b = TypeId::of::<B>();



            let storage_a_ptr = self.component_storages
                .get(&type_a)
                .and_then(|s| s.downcast_ref::<HashMap<Entity, A>>())
                .map(|s| s as *const HashMap<Entity, A>);

            let storage_b_ptr = self.component_storages
                .get_mut(&type_b)
                .and_then(|s| s.downcast_mut::<HashMap<Entity, B>>())
                .map(|s| s as *mut HashMap<Entity, B>);

            if let (Some(a_ptr), Some(b_ptr)) = (storage_a_ptr, storage_b_ptr) {
                let a = unsafe { &*a_ptr };
                let b = unsafe { &mut *b_ptr };

                for (entity, comp_a) in a.iter() {
                    if let Some(comp_b) = b.get_mut(entity) {
                        f(*entity, comp_a, comp_b);
                    }
                }
            }
        }
}
// pub struct Query<A,B> {}

