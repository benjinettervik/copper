/* INTEGRATION TESTS OF ECS */
// Enter 'cargo test' to run tests

use copper::*;

#[cfg(test)]
mod world_tests {
    use copper::ecs::world::*;
    use copper::ecs::entity::*;
    use std::any::Any;
    use std::any::TypeId;

    #[test]
    fn test_1_component() {
        let mut world = World::new();

        struct S1;
        struct S2;
        struct S3;
        
        let components = vec![
            TypeId::of::<S1>(), 
            TypeId::of::<S2>(),];

        let entity_id1: usize = world.spawn();

        world.add_component(entity_id1, S1);

        if let Some::<&S1>(comp_ref1) = world.get_component::<S1>(entity_id1) {
            assert_eq!(comp_ref1.type_id(), TypeId::of::<S1>());
        }
        else {
            assert!(false);
        }

        // CONTINUE!!!!!!
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