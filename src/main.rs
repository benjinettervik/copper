mod health_system;
use health_system::*;
use copper::engine::*;

fn main() {
    println!("--- Main Run ---");
    let mut engine = Engine::new();
    
    engine.add_system(Startup, HealthSystem);
    engine.add_system(Update, HealthSystem);

    engine.run_cycles(1);
}
