use component_macro_derive::Component;
use copper::engine::system::*;
use copper::engine::world::*;
use copper::*;
use std::any::TypeId;
use component_macro_derive::Component;
use copper::resource::Resources;


#[allow(unused)]
#[derive(Component)]
pub struct HealthComponent {
    curr_hp: i32,
    max_hp: usize,
}

#[allow(unused)]
#[derive(Component)]
pub struct SomeRandomComponent {
    test: bool,
}

#[derive(Component)]
pub struct DeathComponent;

pub struct SpawnEntitiesSystem;
pub struct HealthSystem;

impl System for SpawnEntitiesSystem {
    components_write!(HealthComponent);
    components_read!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entity1 = world.spawn();
        let entity2 = world.spawn();

        let health_component1 = HealthComponent {
            curr_hp: 4,
            max_hp: 100,
        };

        println!("Derive macro test: {}", health_component1.name());

        let health_component2 = HealthComponent {
            curr_hp: 20,
            max_hp: 100,
        };

        world.add_component(entity1, health_component1);
        world.add_component(entity1, SomeRandomComponent { test: true });
        world.add_component(entity2, health_component2);
        world.add_component(entity2, DeathComponent {});

        println!("Spawned two entities with Health component");
    }
}

impl System for HealthSystem {
    components_write!(HealthComponent);
    components_read!(SomeRandomComponent);
    components_with!();
    components_without!(DeathComponent);

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        for entity in entities {
            let curr_hp = {
                let mut health_component =
                    world.get_component_mut::<HealthComponent>(entity).unwrap();

                println!("Reducing HP by 1...");

                if health_component.curr_hp > 0 {
                    health_component.curr_hp -= 1;
                }

                health_component.curr_hp
            };

            if curr_hp <= 0 {
                world.add_component(entity, DeathComponent {}); // This is a bit of a question
                // mark. How do you deal with
                // creating new components? We
                // clearly cannot specify them in
                // the the "write" array since the
                // entity wont be returned
                println!("HP is 0! Adding death component to Entity {}", entity)
            }

            let some_random_component = world.get_component::<SomeRandomComponent>(entity).unwrap();

            println!(
                "HP is: {} and value of SomeRandomComponent: {}",
                curr_hp, some_random_component.test
            );
        }
    }
}
