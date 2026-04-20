/* INTEGRATION TESTS OF ECS */
// Enter 'cargo test' to run tests

use copper::*;

#[cfg(test)]
mod world_tests {
    use copper::engine::world::*;
    use copper::health_system::HealthSystem;
    // use copper::ecs::entity::*;
    use std::any::Any;
    use std::any::TypeId;

    #[test]
    fn test_1_component() {
        struct HealthComponent {
            _health: i32,
        }

        let mut world = World::new();

        let entity_id1: usize = world.spawn();

        world.add_component(entity_id1, HealthComponent { _health: 100 });

        if let Some::<&HealthComponent>(comp_ref1) =
            world.get_component::<HealthComponent>(entity_id1)
        {
            assert_eq!(comp_ref1.type_id(), TypeId::of::<HealthComponent>());
            assert_eq!(comp_ref1._health, 100);
        } else {
            assert!(false);
        }

        // Returns mutable reference to entity, that's tied to world
        let new_entity = world.spawn();

        // Can not use world because the entity 
        // reference is still in use
        world.add_component(new_entity, HealthComponent { _health: 100 });

        // CONTINUE!!!!!!
    }
}

#[cfg(test)]
mod engine_tests {
    use copper::engine::world::*;
    use copper::engine::*;
    use copper::health_system::HealthSystem;
    // use copper::ecs::entity::*;
    use std::any::Any;
    use std::any::TypeId;

    #[test]
    fn test_1_create_engine() {
        let mut engine = Engine::new();
        
    }

    
}
