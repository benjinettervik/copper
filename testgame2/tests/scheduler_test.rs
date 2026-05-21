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
    // #[derive(Component)]
    // pub struct SomeRandomComponent {
    //     test: bool,
    // }
    // #[derive(Component)]
    // pub struct SomeRandomComponent {
    //     test: bool,
    // }
    // #[derive(Component)]
    // pub struct SomeRandomComponent {
    //     test: bool,
    // }
    // 
    // ###############
    // ############### Systems for testing
    // ###############
    //
    pub struct S1; 
    impl System for S1{
        components_read!();
        components_write!();
        resources_write!();
        resources_read!(Input);
        components_with!();
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
        components_with!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {

        }
    }
    pub struct S3; 
    impl System for S3{
        components_read!();
        components_write!(C2);
        resources_write!();
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
        components_with!();
        components_without!();

        fn run(&mut self, world: &mut World, resources: &mut Resources) {

        }
    }
    // impl System for TestSystem{
    //     components_read!();
    //     components_write!();
    //     resources_write!();
    //     resources_read!();
    //     components_with!();
    //     components_without!();

    //     fn run(&mut self, world: &mut World, resources: &mut Resources) {

    //     }
    // }
    // impl System for TestSystem{
    //     components_read!();
    //     components_write!();
    //     resources_write!();
    //     resources_read!();
    //     components_with!();
    //     components_without!();

    //     fn run(&mut self, world: &mut World, resources: &mut Resources) {

    //     }
    // }

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
        scheduler.order_system();
        // there are some basic bitch systems now 
        // we are simply going to register some shit for it now

    }
}