/* INTEGRATION TESTS OF ECS */
// Enter 'cargo test' to run tests

#[cfg(test)]
mod world_tests {
    // mod health_system;
    // use health_system::HealthSystem;

    // use::copper::*;
    use::copper::*;
    // use::engine::world::World;
    // use::engine::world::World;

    // use crate::engine_tests::world::World;

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
    use::copper::*;
    use crate::engine_tests::world::World;
    use std::any::TypeId;
    // use::world::*;
        
    
    

    
    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Health {
        value: f32,
    }
    
    impl Component for Health {
        fn name(&self) -> &str {
            "Health"
        }
    }
    impl Component for Position {
        fn name(&self) -> &str {
            "Position"
        }
    }

    impl Component for Velocity {
        fn name(&self) -> &str {
            "Velocity"
        }
    }






    use copper::engine::*;

    #[test]
    fn test_1_create_engine() {
        let mut _engine = Engine::new();
        
    }

    #[test]
    fn query_test(){

        // // take 15 
        // let mut world = World::new();
        // let entity1 = world.spawn();
        // let entity2 = world.spawn();
        // let entity3 = world.spawn();
        // // ent1
        // world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        // world.add_component(entity1, Velocity { dx: 0.1, dy: 0.2 });
        // world.add_component(entity1, Health { value: 100.0 });
        // // ent2
        // world.add_component(entity2, Position { x: 5.0, y: 6.0 });
        // world.add_component(entity2, Velocity { dx: 0.5, dy: 0.6 });
        // // ent3
        // world.add_component(entity3, Position { x: 9.0, y: 10.0 });

        // let mut result1: Vec<Position> = Vec::new();
        // let mut result2: Vec<(Position, Velocity)> = Vec::new();
        // let mut result3: Vec<(Position,Velocity,Health)> = Vec::new();

        // // make a 3 component argument
        // let mut three_components = Vec::new();
        // three_components.push(TypeId::of::<Position>());
        // three_components.push(TypeId::of::<Velocity>());
        // three_components.push(TypeId::of::<Health>());
        // // make a 2 component arg
        // let mut two_components = Vec::new();
        // two_components.push(TypeId::of::<Velocity>());
        // two_components.push(TypeId::of::<Position>());
        // // 1 comp arg
        // let mut one_component = Vec::new();
        // one_component.push(TypeId::of::<Position>());
        
        // let check1 = world.query(
        //     &self.components_read(three_components),
        //     &self.components_write(),
        //     &self.components_with(),
        //     &self.components_without(),
        // );
        // let check2 = world.query(
        //     &self.components_read(two_components),
        //     &self.components_write(),
        //     &self.components_with(),
        //     &self.components_without(),
        // );
        // let check3 = world.query(&self.components_read(one_component),
        //     &self.components_write(),
        //     &self.components_with(),
        //     &self.components_without(),
        // );
        
        // assert_eq!(check1.len(),3);
        // assert_eq!(check2.len(),2);
        // assert_eq!(check3.len(),1);

        // if passed it will return the correct len of the components in storage
    }
    
    #[test]
    fn query_mut_single_component() {
        // let mut world = World::new();

        // let entity1 = world.spawn();
        // let entity2 = world.spawn();

        // world.add_component(entity1, Position { x: 1.0, y: 2.0 });
        // world.add_component(entity2, Position { x: 5.0, y: 6.0 });

        // // Mock system

        // // user argument is the vec of specified component 
        // let mut one_component = Vec::new();
        // one_component.push(TypeId::of::<Position>());


        // // it gets the ids
        // let ent_ids = world.query(one_component);
        // assert_eq!(ent_ids.len(),2);
        // // 
        // // change them
        // for entities in &ent_ids
        // {
        //     let mut mut_comp = world.get_component_mut::<Position>(*entities).unwrap();
            
        //     mut_comp.x = mut_comp.x +10.0;
        //     mut_comp.y = mut_comp.y +10.0;
        // }
        // // check them
        // for ent in &ent_ids {
        //     let pos = world.get_component::<Position>(*ent).unwrap();
        //     assert!(pos.x >= 11.0);
        //     assert!(pos.y >= 12.0);
        // }

    }

    #[test]
    fn two_mutable_borrows() {
        
        // det fungerar för query implementerar t.ex. inte "iterator" som genast applicerar systemets angivelser såsom "öka pos värde på x";
        // den ger bara entities - så får systemen: hämta components och definiera överlag.
        // man kan separara detta genom scopes 
        // {} i systemen 
        // typ mutate velocity i ett scope, mutate position i ett annat
        // det kommer göra att systemen ser bökigare ut men det funkar

        // vad resulterar nuvarande implementationen i: 
        // Systemen kommer behöva ha implementationskrav såsom att systemen måste själva hantera borrow-livstider
        // explicit typ "get → mutate → drop → nästa"

        
        // Att använda sig av refcel skulle minimera kravet på scopes

    }



    
}
