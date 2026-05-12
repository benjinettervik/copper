# Copper
Copper is a game engine written in Rust based on ECS (see ECS.md).

## How to use Copper
Copper does not have a GUI and is used programatically in Rust. You can start using Copper by importing the Crate:
```use copper::engine::*;```
Below you can see some examples of how to use the engine

## Initialize the engine
```let mut engine = Engine::new();```

## Define a component
Components are structs with ```#[derive(Component)]```
```
#[derive(Component)]
struct MyComponent {
    ...
}
```
## Define a system
Systems are structs that implement the System trait. The system trait needs to define which components it wants to read and/or write to, aswell as any filters for query. This is done with the following macros:
- ```components_read!();```
- ```components_write!();```
- ```components_with!();```
- ```components_without!();```

Lastly, a system needs to define its logic within the run function. The run function has the following signature:
```fn run(&mut self, world: &mut World, resources: &mut Resources)```

Example system:

```
struct MySystem
impl System for MySystem {
    components_read!(MyComponent);
    components_write!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
      /// Query for entities, based on defined components above
      let entities = query!(self, world);

      for entity in entities {
          ... do stuff ...
      }
    }
}
```

## Using components and systems
Now that you have components and systems defined, you can start using them. Copper has an abstraction called the "World". The world contains all runtime data for the game, this more or less means all entities and their initialized components.
Let's create a system that creates 10 entities and assigns a component to each of them at startup:
```
struct CreateEntitiesSystem
impl System for CreateEntitiesSystem {
    components_read!();
    components_write!();
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for
    }
}
```
## Snake demo command
CARGO_TARGET_DIR=/private/tmp/copper-snake-run cargo run -p snake