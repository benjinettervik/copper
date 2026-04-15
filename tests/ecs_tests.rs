/* INTEGRATION TESTS OF ECS */
// Enter 'cargo test' to run tests

use copper::*;

#[cfg(test)]
mod world_tests {
    use copper::ecs::world::*;
    use copper::ecs::entity::*;
    use std::any::TypeId;

    #[test]
    fn test_query() {
        let mut world = World::new();

        struct S1 {}
        struct S2 {}
        struct S3 {}
        
        let components = vec![
            TypeId::of::<S1>(), 
            TypeId::of::<S2>(),];

        let entity_ref1= world.spawn();
        //world.add_component(entity_ref1, );

        let entity_ref2: &Entity = world.spawn();

        let result = world.query(&components);

        assert!
    }
}

#[cfg(test)]
mod entity_tests {
    #[test]
    #[ignore = "Shitty test imo"]
    fn testiiing() {
        assert!(true);
    }
}

#[cfg(test)]
mod component_tests {
    
}

#[cfg(test)]
mod system_tests {

}