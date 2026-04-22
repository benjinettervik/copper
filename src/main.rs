mod test_stuff;
use copper::engine::*;
use test_stuff::*;

fn main() {
    println!("--- Main Run ---");
    let mut engine = Engine::new();

    engine.add_system(Startup, SpawnEntitiesSystem);
    engine.add_system(Update, HealthSystem);

    engine.run_cycles(5);
}
