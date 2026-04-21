# Copper Fork 


## DONE

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

