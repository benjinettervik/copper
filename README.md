# branchlog 
cargo run //pops window
cargo test // does tests

## Steps

- [x] 0. Initial setup
- [x] a. Structure the folder more clearly
- [x] 1. Create first `engine_api` page  
- [x] 2. Define how user code interacts with the engine  
- [x] 4. Introduce a basic query system  
- [x] 5. Implement a basic scheduler  
- [x] startup asset manager 
- [x] user can load png into asset manager
- [x] expand -> asset manager -> return a texture handle so user can have some sort of ID, ecs run on handles --> user need to be able to both load and use it for an entity
- [x] user can attach texture to a entity/player 
- [x] user can open window 
- [x] change system trait and apply it so that tests run and render_sys runs
- [x] user can see this texture on entity in window 
- [x] startup renderer 
- [x] change render() functionality to render_sys and have user be able to add systems and whatnot
- [x] input add
- [x] user can move with input




## TODO
Through engine api:
- [ ] trait query -- test cases 
- [ ] expand render system
- [ ] better test split for the core aspects of the engine that must hold true despite what we modify: test scheduler, rendering, asset-m, world, query, yada yada. 
- [ ] grid system

# branchlog 20-04-2026 notes


# basic input added 
basic input in the event loop is added, now where do we store this in a user system? 



# camera
introducing a camera than could be done through passing a matrix that dictates, in what span am I looking at the world?
	- this would mean a shader can look at it

ideally then, considering something like a Player entity, 
the camera would be fixed at this entity position, or controlled by the user for a RTS.


# world refact?
use std::any::Any;
use std::any::TypeId; // we'd like to use generic types and Any 
pub type Entity = usize;

pub struct World {
    next_entity_id: usize, // a given, we need a way to handle the entity ids we give as receipts.

}


// hashmapping pros: give a key, and a value -- fast lookup.
// get, containskey, insert, 

// option is 


// A Box<T> is a smart pointer that stores data on the heap instead of the stack. -> box gives a copy of x on the heap 

// enum Option<T> {}



// so what we want to do is to store a component we do not know the type of yet, and store it on the heap. 
// One owner per value
// You can borrow instead of moving --> mutability is tightly controlled. 

// this means methods decide whether they borrow World immutably (&self)
// or mutably (&mut self), enabling read or write access.

// &self = read access, &mut self = write access

impl World
{
    pub fn new() -> Self{
        Self{
            next_entity_id:0, // we start it off as a 0 
        }
    }

    // read
    // write 


    // now when it comes to query, we have to make a distinction
    // when they call for query, it is to say: "retrieve entities with a given set of components, now we do not know the amount of components asked for" 
    pub fn query2_mut(){}


    pub fn query2_mut<A: 'static, B: 'static, F>(&mut self, mut f: F) 
    where F: FnMut(Entity, &mut A, &mut B), 
    {

    }


}

// could make a trait based query not unlike the system trait 

pub type MutableQuery(fn(Entity,A,B));



# branchlog 19-04-2026 notes

## overall 
... bla bla bla 

# branchlog 18-04-2026 notes

## query 
A query = “give me all entities that have a certain set of components”

pub fn query <T: 'static>(&self) -> Vec<(Entity,&T)> 
    {        
    }
	
Understanding the query:
	a. You interact with world query by asking it: 
		"Give me all entities with these set of components".
		-> You provide the components.
		
		-> It returns an iterator over matching entities AND their components
			-> The data you actually want to operate on



## development with user perspective in mind
    fn test_basics()
    {
        let mut engine = Engine::new();
        let batch = TestScenario {
        test_batch: vec![
            test_no_api_add_comp_to_world,
            test_api_add_comp_to_world,
            test_comp_still_in_engine,
            test_api_user_add_system,
        ],
    };
    for test in batch.test_batch {
        engine = test(engine);
    }}
	
made some basic tests to test engine


## engine api
basic api 

use crate::engine::world::*;
use crate::engine::system::*;
use crate::engine::*;

impl Engine {
    pub fn spawn(&mut self) -> Entity {
        self.world.spawn()
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, comp: T) {
        self.world.add_component(entity, comp);
    }

    pub fn get_component<T: 'static>(&mut self, entity_id:usize) -> Option<&T>
    {
        self.world.get_component(entity_id)
    
    }

    ... etc
}

## a scheduler 
Decides when, in what order, and under what conditions systems run.
	* When systems run
	* What order
	* Under what conditions 
	
made a basic one that now runs in engine.run()




## loading a .png file to texture
 This function is also in the engine api. However not a straight forward system but a functionality the engine offers the user.





