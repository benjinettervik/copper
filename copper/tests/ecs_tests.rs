/* INTEGRATION TESTS OF ECS */
// Enter 'cargo test' to run tests

#[cfg(test)]
mod world_tests {
    // mod health_system;
    // use health_system::HealthSystem;

    use::copper::*;
    use component_macro_derive::Component;
    use copper::engine::world::*;

    #[test]
    fn test_1_component() {
        #[derive(Component)]
        struct HealthComponent {
            health: i32,
        }

        let mut world = World::new();

        let entity_id1 = world.spawn();

        world.add_component(entity_id1, HealthComponent { health: 100 });

        
        { // Necessary scope to kill 'health_comp' early
            let health_comp = world.get_component::<HealthComponent>(entity_id1).unwrap();
            assert_eq!(entity_id1, 0);
            assert_eq!(health_comp.health, 100);
        }

        {
            let mut health_comp1_mut= world.get_component_mut::<HealthComponent>(entity_id1).unwrap();
            health_comp1_mut.health = 69;
            assert_eq!(health_comp1_mut.health, 69);

        }

        let entity_id2 = world.spawn();

        world.add_component(entity_id2, HealthComponent { health: 50 });

        { // Necessary scope to kill 'health_comp' early
            let health_comp2 = world.get_component::<HealthComponent>(entity_id2).unwrap();
            assert_eq!(entity_id2, 1);
            assert_eq!(health_comp2.health, 50);
        }

        {
            let health_comp1= world.get_component::<HealthComponent>(entity_id1).unwrap();
            assert_eq!(health_comp1.health, 69);
        }

    }
}

#[cfg(test)]
mod engine_tests {
    use copper::engine::*;

    #[test]
    fn test_1_create_engine() {
        let mut _engine = Engine::new();
        
    }

    
}
