
use copper::engine::world::*;
use copper::engine::system::*;

pub struct HealthSystem;
impl System for HealthSystem {
    fn run(&mut self, world: &mut World) {
        println!("Herro!");
    }
}
