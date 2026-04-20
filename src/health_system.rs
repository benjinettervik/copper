use crate::engine::world::*;
use crate::engine::system::*;

pub struct HealthSystem;
impl System for HealthSystem {
    fn run(&mut self, world: &mut World) {
        println!("Herro!");
    }
}
