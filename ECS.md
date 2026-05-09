# What is Entity Component System?
Entity Component System, also known as ECS, is an architectural pattern used to build games in a data driven way. ECS is, as it sounds, split into three parts:

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
Systems contain all the logic for the game and they operate on entities based on what components they have. For example, a health system might fetch all entities that have a HealthComponent, and then do some logic, for example writing to the HealthComponents. Systems run at specified points in time, for example at startup or at each frame. To enable performance improvements, an important part of systems is that they need to pre-define what components they want to read and write to. With this, a scheduler can automatically schedule systems to run concurrently if it finds there are is no shared data between them.

## Tying everything together
To visualize how the architecture might look for a simple "game", let's create an abstract one where ECS is used. The game should create (spawn) two entities at startup. These entities should both have some health component. For each frame, we want to reduce the health of both entities by one. To achieve this we would:

### 1. Define the HealthComponent
Define a health component that stores data for the current health.

### 2. Define the "SpawnEntitiesSystem"
Define a system that will spawn two entities at start and assign a HealthComponent to each of them. This system will not fetch any entities, so it does not have to define any components it wants to read or write to.

### 3. Define the "ReduceHealthSystem"
Define a system to reduce the health of these entities. This system will want to operate on all entities that have a HealthComponent associated with them, and it will want to write to those components. Furthermore it will have logic that reduces the current health by one on every component, each frame.

### 4. Initialize the engine
Create/initialize the engine.

### 5. Register the systems
The systems are defined but need to be registered within the engine so that their logic is ran, so we will do that now.

### 6. Run the "game"
Running the game should now produce our wanted logic (note however that it would go on until the current health overflows negatively).
