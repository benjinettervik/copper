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

use std::any::TypeId;

use crate::ecs::component::Component;

use super::entity::*;

pub struct World {
    next_entity_id: u32,
    entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            entities: vec![],
        }
    }

    pub fn spawn(&mut self) -> &Entity {
        let result = Entity::new(&self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.push(result);
        self.entities.last().unwrap() // may need to fix, error check for 'none' value
    }

    pub fn add_component(&mut self, entity: &mut Entity, component: Component) {
        entity.add_component(component);
    }

    pub fn query(&mut self, components: &Vec<TypeId>) {
        //Vec<&Entity> {
        let result: Vec<&Entity> = vec![];

        // Improve following code pelase
        for entity in &self.entities {
            let components = entity.get_components();
        }
    }
}

