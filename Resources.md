# Resources 

## What are resources in an ECS engine?
Resources in an ECS engine are defined as external data that is not associated to an entity. Because of this it is not stored in World, who concerns itself with runtime data. However, systems might require data outside of the boundary of just world.
Similarily to world the engine does not pre-suppose what type of Resources, Components or Systems the user will want to use, and needs to be able to apply a deal of genericity.

```
pub struct Resources {
resources: HashMap<TypeId, Box<dyn Any>>,
}
```

A basic kit is for this base model added for the user of the engine to mess around with.
```
    pub fn init_basic_kit(&mut self){
        self.insert(RenderQueue{commands: Vec::new(),});
        self.insert(TextureAsset{textures: HashMap::new(),});
        self.insert(Camera2D::new());
        self.insert(Input::new());
        // self.resources.insert(Grid::new(32,32,16.0));
    }
```

Technically, however, the user has freedom to add whatever type of resource they deem necessary for their engine.

``` 
pub struct MySupportResources;
...
...
fn main(){
	let mut engine = Engine::new();
	engine.resources.insert(MySupportResources{});
}

```