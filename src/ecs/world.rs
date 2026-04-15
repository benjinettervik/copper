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
use super::entity::*;


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

    pub fn get_components()

    pub fn query(&mut self, components: &Vec<TypeId>) -> Vec<&Entity> {
        let mut result: Vec<&Entity> = vec![];

        // Improve following code pelase
        for entity in &self.entities {
            let mut count: usize = 0;

            for component in entity.get_components() {
                if components.contains(&component.type_id()) {
                    count += 1;

                    if count == components.len() {
                        result.push(entity);
                        break;
                    }
                }
            }
        }

        result
    }
}

