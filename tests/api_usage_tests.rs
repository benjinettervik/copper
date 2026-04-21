use copper::*;
// use crate::user_code;



// pub struct tests { 
//     test_batch: Vec<fn(mut Engine)->Engine>,
// }
#[cfg(test)]
mod api_usage_tests {
    use copper::engine::world::*;
    use copper::engine::*;
    use crate::resource::asset_manager::AssetManager;
    // use copper::health_system::HealthSystem;
    // use copper::ecs::entity::*;
    use std::any::Any;
    use std::any::TypeId;
    use copper::api::*;
    // 
    // concurrency
    use std::thread;
    use std::sync::{mpsc, Arc, Mutex};
    use rayon::prelude::*;
    
    // 
    // 
    // 
    // 
    use copper::user_code::*;
    pub struct TestScenario {pub test_batch: Vec<fn(Engine) -> Engine>,
    }
    
    // mod user_code;
    #[test]
    fn test_basics()
    {
        let mut engine = Engine::new();
        let batch = TestScenario {
        test_batch: vec![
            test_no_api_add_comp_to_world,
            test_api_add_comp_to_world,
            test_comp_still_in_engine,
            test_api_user_add_system_to_scheduler,
            test_engine_run_basic_scheduler,
        ],
    };
    for test in batch.test_batch {
        engine = test(engine);
    }
    }

    fn test_no_api_add_comp_to_world(mut engine:Engine) -> Engine{

        let mut world = World::new();
        let entity_id1: usize = world.spawn();
        println!("entity_id1");
        world.add_component(entity_id1, MockComponent1{x:1.0});
        let comp_ref: Option<&MockComponent1> = world.get_component(entity_id1);
        assert_eq!(comp_ref.unwrap().x,1.0);
        engine
    
    }

    fn test_api_add_comp_to_world(mut engine:Engine)->Engine{

        let entity = engine.spawn();
        assert_eq!(entity,0);
        engine.add_component(entity, MockComponent1::new());
        let ref_1 = engine.get_component::<MockComponent1>(entity);
        assert_eq!(ref_1.unwrap().x,1.0);
        engine
    }

    fn test_comp_still_in_engine(mut engine:Engine)->Engine{

        let entity = engine.spawn();
        // next id would be one
        assert_eq!(entity,1);
        engine
    }
    
    fn test_api_user_add_system_to_scheduler(mut engine:Engine) -> Engine {
        engine.register_system(DamageSystem);
        assert_eq!(engine.has_system::<DamageSystem>(),true);
        engine
    }
    fn test_engine_run_basic_scheduler(mut engine:Engine) -> Engine {
        
        let entity=engine.spawn();
        engine.add_component(entity,MockHp{value:30.0});
        engine.no_ev_loop_run(1);
        let comp_ref:Option<&MockHp> = engine.get_component(entity);
        // println!("{}",comp_ref.unwrap().value);
        assert_eq!(comp_ref.unwrap().value,20.0);
        engine
    }


    #[test]
    fn test_user_loads_png(){
        
            let mut engine = Engine::new();
            // load into texture - get a texture handle
            let handle = engine.load_png("troll.png");
            // println!("{:?}",handle);
            if let Some(am) = engine.resources.get_mut::<AssetManager>()
            {
                let tex = am.textures.get(&handle);
                // println!("{:?}",tex);
                assert_eq!(tex.unwrap().width,32);
                assert_eq!(tex.unwrap().height,32);
                let rgba_index = 4;
                let size = 32*32;
                assert_eq!(tex.unwrap().data.len(),(size)*rgba_index);
            }
        
    }
    #[test]
    fn concurrency_data_types(){
        let handle = thread::spawn(|| {
            // println!("Hello from another thread!");
        });

        handle.join().unwrap(); // wait for it to finish
        // Rayon for data parallel systems

    }

    #[test]
        fn test_user_attach_texture_to_sprite_entity(){
            let mut engine = Engine::new();
            // load into texture - get a texture handle
            let handle = engine.load_png("troll.png");
            let entity = engine.spawn();
            let sprite = MockSprite{name:"Bob".to_string(),texture_handle:handle,layer: 1};
            engine.add_component(entity, sprite);
            let compref = engine.get_component::<MockSprite>(entity);
            // println!("{:?}",compref);
        }


        
    #[test]
        fn test_eventloop_and_rendersys(){


               
        }
        
        

}


    // #[test]
    //     fn test_template(){
                
    //         }