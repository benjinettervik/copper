use std::any::TypeId;

mod ecs;
use ecs::system::*;

struct Health {
    current: i32,
    max: i32,
}

struct Random {
    rand: i32,
}

struct Health_system;
impl System for Health_system {
     fn get_component_types() -> Vec<TypeId> {
        vec![
            TypeId::of::<Health>(), 
            TypeId::of::<Random>()]
     }
    
    fn _on_ready() {
        
    }
}