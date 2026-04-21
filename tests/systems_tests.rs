mod systems_tests {
    use copper::engine::world::*;
    // use copper::health_system::HealthSystem;
    // use copper::ecs::entity::*;
    use copper::engine::meta::SystemMeta;
    use std::any::Any;
    use std::collections::HashSet;
    use std::any::TypeId;
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
    fn system_meta_collects_correctly() {
        let mut meta = SystemMeta {
            reads: HashSet::new(),
            writes: HashSet::new(),
            resource_reads: HashSet::new(),
            resource_writes: HashSet::new(),
        };

        <(&Position, &mut Velocity)>::meta(&mut meta);
        assert!(meta.reads.contains(&TypeId::of::<Position>()));
        assert!(meta.writes.contains(&TypeId::of::<Velocity>()));
    }
}