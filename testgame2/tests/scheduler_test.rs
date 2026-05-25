#[cfg(test)]
mod scheduler_test{
    // import test components
    use copper::engine::system::*;
    use copper::engine::world::*;
    use copper::engine::scheduler::*;
    use copper::engine::*;
    use copper::input::*;
    use copper::grid::*;
    use copper::resource::{Resources,RenderLayer,TM_Handle,TextureMap};
    use copper::renderer::render_sys::{NewRenderSys,RenderSys,TileMapStorage,GridStorage,TileMap,GridRenderSys,TMapHandle,GridHandle,GridRenderMeta};
    use copper::resource::{convert_texture,extract_tileset,extract_layer_data2,json_to_rendermap};
    use copper::resource::camera::*;
    // use copper::input::Input;
    use std::collections::HashMap;
    use copper::renderer::test_components_renderer::*;
    use copper::*;
    use std::any::TypeId;
    use component_macro_derive::*;
    use std::sync::RwLock;
    use std::any::Any;

    // 
    // ###############
    // ############### Components for testing
    // ###############
    // 
    #[derive(Component)]
    pub struct C1 {
        test: bool,
    }
    #[derive(Component)]
    pub struct C2 {
        test: bool,
    }
    pub struct S1; 
    impl System for S1{
        components_read!();
        components_write!();
        resources_write!();
        resources_read!(Input);
        components_with!();
        system_id!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {

        }
    }
    pub struct S2; 
    impl System for S2{
        components_read!(C1);
        components_write!();
        resources_write!();
        resources_read!();
        system_id!();
        components_with!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {

        }
    }
    pub struct S3; 
    impl System for S3{
        components_read!(C2);
        components_write!(C1);
        resources_write!();
        system_id!();
        resources_read!();
        components_with!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {

        }
    }
    pub struct S4; 
    impl System for S4{
        components_read!(C1);
        components_write!();
        resources_write!(Input);
        resources_read!();
        system_id!();
        components_with!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {

        }
    }
    #[test]
    pub fn scheduler_test_basic(){
    // there is technically now a data access struct that saves all the read_world, write_world data
    // also one that has all the comp_reg
    
        let mut scheduler = Scheduler::new();
        let mut world = World::new();
        let mut resources = Resources::new();
        scheduler.add_system(Update,S1);
        scheduler.add_system(Update,S2);
        scheduler.add_system(Update,S3);
        scheduler.add_system(Update,S4);
        scheduler.run_update(&mut world, &mut resources);
        let dep_graph = scheduler.make_dep_graph();
        println!("{:?}",dep_graph);
        let sorted_sys = scheduler.sort_systems(dep_graph);
        println!("{:?}",sorted_sys);
        // scheduler.set_order(sorted_sys);
        let mut world = World::new();
        let mut resources = Resources::new();
        scheduler.run_prio_update(&mut world,&mut resources,&sorted_sys);
        // there are some basic bitch systems now 
        // we are simply going to register some shit for it now

    }
    pub struct WorldRefact {
        next_entity_id: usize,
        component_storages: HashMap<ComponentId, RwLock<HashMap<EntityId, Box<dyn Any+ Send+ Sync>>>>,
    }

    impl WorldRefact
    {
        pub fn new() -> Self {
            WorldRefact {
                next_entity_id: 0,
                component_storages: HashMap::new(),
            }
    }
        pub fn spawn(&mut self) -> EntityId {
            let return_id = self.next_entity_id;
            self.next_entity_id += 1;
            return_id
        }

        pub fn add_component(){}
        pub fn get_component(){}
        pub fn get_component_mut(){}
        pub fn query(){}
        // 1. add component
        // 2. get component
        // 3. get component mut
        // 4. implement new query
    }
 
    #[test]
    pub fn world_refactor_tests()
    {
        let mut world_refact = WorldRefact::new();
    }

}