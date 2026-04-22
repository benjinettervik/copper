mod test_stuff;
use copper::engine::*;
use test_stuff::*;

fn main() {
    println!("--- Main Run ---");
    let mut engine = Engine::new();

    engine.add_system(Startup, SpawnEntitiesSystem)
        .add_system(Update, HealthSystem)
        .run_cycles(5);
}
