mod health_system;
use health_system::*;
use copper::engine::*;

fn main() {
    println!("---------------- LOL ---------------");
    let mut engine = Engine::new();

    //engine.scheduler.add_startup_system(HealthSystem);
    //engine.scheduler.add_update_system(HealthSystem);
    
    engine.add_system(Startup, HealthSystem);
    engine.add_system(Update, HealthSystem);

    engine.run_cycles(2);
}
