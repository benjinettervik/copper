# Copper
Copper is a game engine based on ECS (see ECS.md) written in Rust.

## How to use Copper
Copper does not have a GUI, instead it's used programatically in Rust. You can start using Copper by importing the Crate:
```use copper::engine::*;```

Below we will create a simple game to show how Copper is used. The game will, at startup, spawn 10 entities, each with a health component that keeps track of their current health. Then, at every update, each entities' health will be reduced by 1. When an entity reaches 0 health, a death component will be added to it.

## Initializing the engine
The engine is used by initializing the base struct "Engine". This is done by running ```Engine::new();```. In our game's main function, lets do:  
 ```let mut engine = Engine::new();```

## Defining our components
Now we have an instance of the engine, but we cannot really do anything since we don't have any components or systems. Let's define our components. Components are simply structs with the ```#[derive(Component)]``` tag:
```
#[derive(Component)]
struct HealthComponent {
    current: usize
}

#[derive(Component)]
struct DeathComponent {}
```

## Defining our systems
Now that we have our components we should define our systems. For our logic we need two systems, one that spawns the entities and creates their components, and one that reduces the health on each update. Systems are structs that implement the "system" trait. If you are familiar with ECS, you know that systems query for entities. This query is statically defined for each system. In Copper, the system specified which components it wants to read and write to with the following macros:
- ```components_read!();```
- ```components_write!();```

In addition to this, a system might also want to query for entities based on what components they simply have or don't have, without wanting to actually read or write to them. This is defined with the following macros:
- ```components_with!();``` 
- ```components_without!();```

So, whenever a system does its query for entities, the components defined in these four will act as the filter. A component that is defined in read/write should not be defined in with/without. NOTE: the query system will hopefully be made more intuitive later on.

Lastly, the systems defines their run logic inside the ```run``` function. The run function has the following signature:
```fn run(&mut self, world: &mut World, resources: &mut Resources)```

Let's create our systems:
```
struct SpawnEntitiesSystem
impl System for SpawnEntitiesSystem {
    // We will not be querying for any entities, hence these are all empty
    components_read!();
    components_write!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for _ in 1..10 {
            // we use world.spawn() to create/spawn new entities.
            let entity = world.spawn();
            world.add_component(entity, ArbitraryComponent {value_x: 2, value_y: 10});
        }
    }
}

struct ReduceHealthSystem
impl System for ReduceHealthSystem {
    // We want to write to entities health components. We also don't want entities with the death component. Let's define this here.
    components_read!();
    components_write!(HealthComponent);
    components_with!();
    components_without!(DeathComponent);

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let entities = query!(self, world);

        for entity in entities {
            let mut health_component =
                world.get_component_mut::<HealthComponent>(entity).unwrap();

            println!("Reducing HP by 1...");

            if health_component.curr_hp > 0 {
                health_component.curr_hp -= 1;
            }
            else {
                world.add_component(entity, DeathComponent{});
            }
        }
    }
}
```

As you see, entities exist within an abstraction called "World". The World is simply the place where runtime data is stored, i.e existing entities and their initalized components.

## Registering our systems
Now, we are ready to try our "game". We have our systems and components, but for the systems to actually run we need to register them in the engine. We do this by running:  
```
engine.add_system(Startup, SpawnEntitiesSystem);
engine.add_system(Update, ReduceHealthSystem);
```

Startup and Update represent two different schedules that are built into the engine.

Let's run our game with ```engine.run()```
