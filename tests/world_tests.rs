#[cfg(test)]
mod world_tests {
    use copper::engine::world::*;
    // use copper::health_system::HealthSystem;
    // use copper::ecs::entity::*;
    use std::any::Any;
    use std::any::TypeId;
    
    pub struct Player;


    #[derive(Debug, PartialEq,Copy,Clone)]
    struct Position {
        x: f32,
        y: f32,
    }



    #[derive(Debug, PartialEq,Copy,Clone)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }
    #[derive(Debug, PartialEq,Copy,Clone)]
    struct Health {
        value: f32,
    }

    
    #[test]
    fn spawn_returns_incrementing_ids() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();

        assert_eq!(e1, 0);
        assert_eq!(e2, 1);
        assert_eq!(e3, 2);
    }
    #[test]
    fn can_add_and_get_component() {
        let mut world = World::new();

        let e = world.spawn();
        world.add_component(e, Position { x: 10.0, y: 20.0 });

        let pos = world.get_component::<Position>(e).unwrap();
        assert_eq!(pos, &Position { x: 10.0, y: 20.0 });
    }
    #[test]
    fn get_component_returns_none_if_missing() {
        let mut world = World::new();

        let e = world.spawn();

        let pos = world.get_component::<Position>(e);
        assert!(pos.is_none());
    }
    #[test]
    fn can_mutate_component() {
        let mut world = World::new();

        let e = world.spawn();
        world.add_component(e, Position { x: 1.0, y: 2.0 });

        let pos = world.get_component_mut::<Position>(e).unwrap();
        pos.x = 99.0;

        let pos_after = world.get_component::<Position>(e).unwrap();
        assert_eq!(pos_after.x, 99.0);
    }


    #[test]
    fn query_test(){
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();

        // e1 → all components
        world.add_component(e1, Position { x: 1.0, y: 2.0 });
        world.add_component(e1, Velocity { dx: 0.1, dy: 0.2 });
        world.add_component(e1, Health { value: 100.0 });

        // e2 → only Position + Velocity
        world.add_component(e2, Position { x: 5.0, y: 6.0 });
        world.add_component(e2, Velocity { dx: 0.5, dy: 0.6 });

        // e3 → only Position
        world.add_component(e3, Position { x: 9.0, y: 10.0 });


        let mut res_comp1: Vec<Position> = Vec::new();
        let mut res_comp2: Vec<(Position, Velocity)> = Vec::new();
        let mut res_comp3: Vec<(Position, Velocity, Health)> = Vec::new();

        for (entity, pos) in world.query::<(&Position,)>().iter() {
            res_comp1.push(*pos);
        }

        for (entity, (pos, vel)) in world
            .query::<(&Position, &Velocity)>()
            .iter()
        {

            res_comp2.push((*pos, *vel));
        }

        for (entity, (pos, vel, health)) in world
            .query::<(&Position, &Velocity, &Health)>()
            .iter()
        {

            res_comp3.push((*pos, *vel, *health));
        }
        assert_eq!(res_comp1.len(), 3);
        assert_eq!(res_comp2.len(), 2);
        assert_eq!(res_comp3.len(), 1);
    }

    #[test]
    fn query_mut_single_component() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();

        world.add_component(e1, Position { x: 1.0, y: 2.0 });
        world.add_component(e2, Position { x: 5.0, y: 6.0 });

        // mutate via QueryParam<&mut T>
        for (entity, pos) in world.query::<(&mut Position,)>().iter() {
            pos.x += 10.0;
            pos.y += 20.0;
        }

        // verify mutation persisted
        let p1 = world.get_component::<Position>(e1).unwrap();
        let p2 = world.get_component::<Position>(e2).unwrap();

        assert_eq!(p1.x, 11.0);
        assert_eq!(p1.y, 22.0);

        assert_eq!(p2.x, 15.0);
        assert_eq!(p2.y, 26.0);
    }
    #[test]
    fn query_mixed_read_write() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();

        world.add_component(e1, Player);
        world.add_component(e1, Position { x: 0.0, y: 0.0 });

        world.add_component(e2, Position { x: 5.0, y: 5.0 }); 

        for (entity, (_player, pos)) in world
            .query::<(&Player, &mut Position)>()
            .iter()
        {
            pos.x += 1.0;
        }

        let p1 = world.get_component::<Position>(e1).unwrap();
        let p2 = world.get_component::<Position>(e2).unwrap();

        assert_eq!(p1.x, 1.0); // mutated
        assert_eq!(p2.x, 5.0); // unchanged
    }
    #[test]
    fn query_double_mut_same_type_should_not_exist() {
        let mut world = World::new();
        let e = world.spawn();

        world.add_component(e, Position { x: 0.0, y: 0.0 });


        assert!(true);
    }
   
    




}