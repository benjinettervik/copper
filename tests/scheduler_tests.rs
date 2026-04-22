#[cfg(test)]

mod scheduler_tests{

    use copper::engine::world::*;
    // use copper::health_system::HealthSystem;
    // use copper::ecs::entity::*;
    use std::any::Any;
    use std::any::TypeId;
    use copper::engine::Engine;
    use copper::user_code::*;
    // use copper::engine::Scheduler;
    use copper::engine::scheduler::Scheduler;
    

    #[test]
    fn scheduler_has_systems() {
        // test block
        println!("\n\n\n~~~~~~~~~~\nBreakpoint testing scheduler: \n~~~~~~~~~~\n\n\n");
        println!("~~~~~~~~~~\n\n\n");
        
        let mut scheduler = Scheduler::new();
        scheduler.add_startup_system(SpawnSystem);
        scheduler.add_update_system(MovementSystem);
        scheduler.add_update_system(DamageSystem);
        
        
        assert!(scheduler.has_system::<DamageSystem>());
        assert!(scheduler.has_system::<MovementSystem>());
        assert!(scheduler.has_system::<SpawnSystem>());
    }
    
    #[test]
    fn scheduler_can_access_metadata() {
        
        let mut scheduler = Scheduler::new();
        scheduler.add_startup_system(SpawnSystem);
        scheduler.add_update_system(MovementSystem);
        scheduler.add_update_system(DamageSystem);
        
        assert!(!scheduler.collect_startup_meta().is_empty());
        assert!(!scheduler.collect_update_meta().is_empty());
        
        let check = scheduler.collect_startup_meta();
        let check2 = scheduler.collect_update_meta();

        for meta in check{
            meta.print_it();
        }
        for meta in check2{
            meta.print_it();
        }

    }


    #[test]
    fn scheduler_can_thread_depending(){

        // this will test the scheduler logic
        // actually making some sort of call depending on the system data that is provided. 
    }
    
}
