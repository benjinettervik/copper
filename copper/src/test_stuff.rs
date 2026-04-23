use copper::engine::system::*;
use copper::engine::world::*;
use copper::*;
use std::any::TypeId;
use component_macro_derive::Component;


#[allow(unused)]
#[derive(Component)]
pub struct HealthComponent {
    curr_hp: i32,
    max_hp: usize,
}


#[derive(Component)]
pub struct DeathComponent;

pub struct SpawnEntitiesSystem;
pub struct HealthSystem;

impl System for SpawnEntitiesSystem {
    get_component_types!();

    fn run(&mut self, world: &mut World) {
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
        world.add_component(entity2, health_component2);
        world.add_component(entity2, DeathComponent {});

        println!("Spawned two entities with Health component");
    }
}

impl System for HealthSystem {
    get_component_types!(HealthComponent, DeathComponent);

    fn run(&mut self, world: &mut World) {
        let entities = world.query(self.get_component_types());

        for entity in entities {
            let mut health_component = world.get_component_mut::<HealthComponent>(entity).unwrap();

            println!(
                "Current HP for Entity {} is {}",
                entity, health_component.curr_hp
            );

            if health_component.curr_hp > 0 {
                health_component.curr_hp -= 1;
            }

            if health_component.curr_hp <= 0 {
                drop(health_component); // reference to World, hence the drop
                world.add_component(entity, DeathComponent {}); //will add multiple DeathComponent.
                //We probably need some kind of
                //filter to query where component
                //does NOT exist on entity
                println!("HP is 0! Adding death component to Entity {}", entity)
            }
        }
    }
}
