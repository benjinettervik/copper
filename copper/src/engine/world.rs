use std::any::Any;
use std::any::TypeId;
use std::cell::*;
use std::collections::HashMap;
use std::collections::HashSet;

type EntityId = usize;
type ComponentId = TypeId;

use crate::Component;

// #[macro_export]
// macro_rules! query {
//     ($($arg:expr), *) => {
//         $(
//             println!("{}", $arg);
//         )*
//     };
// }

pub struct World {
    next_entity_id: usize,
    component_storages: HashMap<ComponentId, RefCell<HashMap<EntityId, Box<dyn Any>>>>,
}

impl World {
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            component_storages: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> EntityId {
        let return_id = self.next_entity_id;
        self.next_entity_id += 1;

        // TODO: Might want to save which entities are currently alive if we want to be able to delete entities later

        return_id
    }

    /// Adds a component onto an entity. Adds any new components into the world's storage if it does not already exist.
    pub fn add_component<T: Component>(&mut self, entity_id: EntityId, component: T) {
        // Generates a unique ID based on a 'static component struct type.
        // Example: every unique component of type 'struct Player' will generate the same component_id!
        let component_id = TypeId::of::<T>();

        // Gets the entry with the right component id.
        // If an entry does not exist, create one for component_id.
        let component_storage_box = self
            .component_storages
            .entry(component_id)
            .or_insert_with(|| RefCell::new(HashMap::new()));

        // 'component_storage' in this case is a hashmap that stores which entities who has the
        // 'component_storage' specific component assigned to it and:
        // - KEY is the unique ID of an entity.
        // - VALUE is the unique component data storage for that entity

        let mut borrow = component_storage_box.borrow_mut();

        borrow.insert(entity_id, Box::new(component));
    }

    /// Gets a reference to a component that is assigned to entity_id
    pub fn get_component<'a, T: Component>(&'a self, entity_id: EntityId) -> Option<Ref<'a, T>> {
        let component_type_id = TypeId::of::<T>();

        let storage_cell = self.component_storages.get(&component_type_id)?;

        let result = Ref::filter_map(storage_cell.borrow(), |entities_map| {
            let component_box = entities_map.get(&entity_id)?;
            component_box.downcast_ref::<T>()
        }).ok();

        result


        // let borrowed = self.component_storages.get(&component_type_id)?.borrow();
        
        // if !borrowed.contains_key(&entity_id) {
        //     return None;
        // }

        // let component = Ref::map(borrowed, |map| {
        //     map.get(&entity_id).unwrap().downcast_ref::<T>().unwrap()
        // });

        // Some(component)
    }

    /// Gets a mutable reference to a component that is assigned to entity_id
    pub fn get_component_mut<T: Component>(&mut self, entity_id: EntityId) -> Option<RefMut<'_, T>> {
        let component_type_id = TypeId::of::<T>();

        let storage_cell = self.component_storages.get(&component_type_id)?;

        let result = RefMut::filter_map(storage_cell.borrow_mut(), |entities_map| {
            let component_box = entities_map.get_mut(&entity_id)?;
            component_box.downcast_mut::<T>()
        }).ok();

        result
        
        
        // let borrowed = self
        //     .component_storages
        //     .get(&component_type_id)?
        //     .borrow_mut();

        // if !borrowed.contains_key(&entity_id) {
        //     return None;
        // }

        // let component = RefMut::map(borrowed, |map| {
        //     map.get_mut(&entity_id)
        //         .unwrap()
        //         .downcast_mut::<T>()
        //         .unwrap()
        // });

        // Some(component)
    }

    pub fn query(&self, type_ids: Vec<TypeId>) -> Vec<EntityId> {
        let mut sets: Vec<HashSet<EntityId>> = Vec::new();

        for type_id in type_ids {
            let Some(storage) = self.component_storages.get(&type_id) else {
                return Vec::new();
            };

            let borrowed = storage.borrow();
            let keys = borrowed.keys().copied().collect::<HashSet<EntityId>>();

            sets.push(keys);
        }

        sets.into_iter()
            .reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap_or_default()
            .into_iter()
            .collect()
    }


    pub fn query2<T: Any + 'static>() {
        
    }
}

