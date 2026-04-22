use core::borrow;
use std::any::Any;
use std::any::TypeId;
use std::cell::*;
use std::collections::HashMap;

type EntityId = usize;
type ComponentId = TypeId;

use crate::Component;

#[macro_export]
macro_rules! query {
    ($($arg:expr), *) => {
        $(
            println!("{}", $arg);
        )*
    };
}

pub struct World {
    next_entity_id: usize,
    component_storages: HashMap<ComponentId, RefCell<Box<dyn Any>>>,
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
    pub fn add_component<T: Component + 'static>(&mut self, entity_id: EntityId, component: T) {
        // Generates a unique ID based on a 'static component struct type.
        // Example: every unique component of type 'struct Player' will generate the same component_id!
        let component_id = TypeId::of::<T>();

        // Gets the entry with the right component id.
        // If an entry does not exist, create one for component_id.
        let component_storage_box = self
            .component_storages
            .entry(component_id)
            .or_insert_with(|| RefCell::new(Box::new(HashMap::<EntityId, T>::new())));

        // 'component_storage' in this case is a hashmap that stores which entities who has the
        // 'component_storage' specific component assigned to it and:
        // - KEY is the unique ID of an entity.
        // - VALUE is the unique component data storage for that entity

        let mut borrow = component_storage_box.borrow_mut();

        // Downcast component_storage_box
        let component_storage = borrow
            .downcast_mut::<HashMap<EntityId, T>>()
            .expect("FATAL ERROR: This should not happen... ");

        component_storage.insert(entity_id, component);
    }

    /// Gets a reference to a component that is assigned to entity_id
    pub fn get_component<T: Component + 'static>(&self, entity_id: EntityId) -> Option<Ref<T>> {
        let borrowed = self.component_storages.get(&TypeId::of::<T>())?.borrow();

        let map_ref = Ref::map(borrowed, |any| {
            any.downcast_ref::<HashMap<EntityId, T>>().unwrap()
        });

        if !map_ref.contains_key(&entity_id) {
            return None;
        }

        let component = Ref::map(map_ref, |map| map.get(&entity_id).unwrap());

        return Some(component);
    }

    /// Gets a mutable reference to a component that is assigned to entity_id
    pub fn get_component_mut<T: Component + 'static>(&mut self, entity_id: EntityId) -> Option<RefMut<T>> {
        let borrowed = self
            .component_storages
            .get(&TypeId::of::<T>())?
            .borrow_mut();

        let map = RefMut::map(borrowed, |any| {
            any.downcast_mut::<HashMap<EntityId, T>>().unwrap()
        });

        if !map.contains_key(&entity_id) {
            return None;
        }

        let component = RefMut::map(map, |map| map.get_mut(&entity_id).unwrap());

        return Some(component);
    }

    pub fn query<T: Component + 'static>(&self) -> Vec<EntityId> {
        let mut result = Vec::new();

        let Some(component_storage) = self.component_storages.get(&TypeId::of::<T>()) else {
            return result;
        };

        let borrowed = component_storage.borrow();

        let entities = Ref::map(borrowed, |any| {
            any.downcast_ref::<HashMap<usize, T>>().unwrap()
        });

        for (entity, _) in entities.iter() {
            result.push(*entity);
        }

        return result;
    }


    pub fn query2<T: Any + 'static>() {
        
    }
}
