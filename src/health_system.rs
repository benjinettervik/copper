use copper::*;
use copper::engine::world::*;
use copper::engine::system::*;

pub struct Hej {}

pub struct HealthSystem;
impl System for HealthSystem {
    get_component_types!();
    
    fn run(&mut self, world: &mut World) {
        println!("Herro!");
    }
}

