use std::any::TypeId;
use crate::{ecs::system::*, query_for_components};

struct Health {
    current: i32,
    max: i32,
}

struct Random {
    rand: i32,
}

pub struct HealthSystem;
impl System for HealthSystem {
    query_for_components!(Health, Random);
    
    fn _on_ready(&self) {
        
    }

    fn _process(&self) {

    }

    fn _delta_process(&self) {

    }
}

