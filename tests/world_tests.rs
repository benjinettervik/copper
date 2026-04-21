#[cfg(test)]
mod world_tests {
    use copper::engine::world::*;
    // use copper::health_system::HealthSystem;
    // use copper::ecs::entity::*;
    use std::any::Any;
    use std::any::TypeId;


    #[derive(Debug, PartialEq)]
    struct Position {
        x: i32,
        y: i32,
    }

    #[derive(Debug, PartialEq)]
    struct Velocity {
        dx: i32,
        dy: i32,
    }
    #[test]
    fn test_world_starts_empty() {
        let world = World::new();
        let entities = world.query::<Position>();        
        assert!(entities.is_empty());
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
        world.add_component(e, Position { x: 10, y: 20 });

        let pos = world.get_component::<Position>(e).unwrap();
        assert_eq!(pos, &Position { x: 10, y: 20 });
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
        world.add_component(e, Position { x: 1, y: 2 });

        let pos = world.get_component_mut::<Position>(e).unwrap();
        pos.x = 99;

        let pos_after = world.get_component::<Position>(e).unwrap();
        assert_eq!(pos_after.x, 99);
    }
    #[test]
    fn query_returns_all_entities_with_component() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();

        world.add_component(e1, Position { x: 1, y: 1 });
        world.add_component(e2, Position { x: 2, y: 2 });

        let result = world.query::<Position>();

        assert_eq!(result.len(), 2);
    }
    #[test]
    fn query_excludes_entities_without_component() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();

        world.add_component(e1, Position { x: 1, y: 1 });
        // e2 has no Position

        let result = world.query::<Position>();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, e1);
    }
    #[test]
    fn query2_mut_updates_components_correctly() {
        let mut world = World::new();

        let e = world.spawn();

        world.add_component(e, Position { x: 0, y: 0 });
        world.add_component(e, Velocity { dx: 5, dy: 7 });

        world.query2_mut::<Velocity, Position, _>(|_, vel, pos| {
            pos.x += vel.dx;
            pos.y += vel.dy;
        });

        let pos = world.get_component::<Position>(e).unwrap();
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 7);
    }
    #[test]
    fn query2_mut_skips_entities_missing_components() {
        let mut world = World::new();

        let e1 = world.spawn();
        let e2 = world.spawn();

        world.add_component(e1, Position { x: 0, y: 0 });
        world.add_component(e1, Velocity { dx: 1, dy: 1 });

        world.add_component(e2, Position { x: 10, y: 10 });

        world.query2_mut::<Velocity, Position, _>(|_, vel, pos| {
            pos.x += vel.dx;
        });

        let pos1 = world.get_component::<Position>(e1).unwrap();
        let pos2 = world.get_component::<Position>(e2).unwrap();

        assert_eq!(pos1.x, 1);
        assert_eq!(pos2.x, 10); 
    }




}