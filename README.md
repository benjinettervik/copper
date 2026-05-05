# Copper
Copper is a game engine written in Rust based on ECS (see ECS.md).

## How to use Copper

Import Copper
```use copper::engine::*;```

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
