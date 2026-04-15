use std::any::TypeId;
use crate::{ecs::system::*, query_for_components};

struct Health {
    current: i32,
    max: i32,
}

struct Random {
    rand: i32,
}

struct Health_system;
impl System for Health_system {
    query_for_components!(Health, Random);
    
    fn _on_ready() {
        
    }

    fn _process() {

    }

    fn _delta_process() {

    }
}

