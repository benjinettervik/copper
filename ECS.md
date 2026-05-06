# What is Entity Component System?
Entity Component System, also known as ECS, is an architectural pattern used to build games in a data driven way. ECS is, as the name implies, split into three parts:

## Entity
An entity is just that, an entity. In Copper, Entities are represented as a usize. Entities are in turn associated with components, for example a "health" component or an "attack" component. 
Whenever it's referred to that an entity "has" a component, it means it has a component associated with it. Entities don't technically contain components, they are associated by mapping. This enables storing components together in the memory, achieving better performance.

## Component
Components are simply containers of data. In Copper, components are represented as structs. An example of a "HealthComponent":
```
struct HealthComponent {
  current: i32,
  max: i32
}
```

## System
Systems contain all the logic for the game and they operate on entities based on what components they have. For example, a health system might fetch all entities that have a HealthComponent, and then do some logic, for example writing updated health to the components. Systems run at specified points in time, for example at startup or at each frame. To enable performance improvements, an important part of systems is that they need to pre-define what components they want to read and write to. With this, a scheduler can automatically schedule systems to run concurrently if it finds there are is no shared data between them.

## Tying everything together
Check the README.md for an example game!
