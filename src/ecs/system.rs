use std::any::TypeId;

pub trait System {
    fn _on_ready(&self);

    fn _process();

    fn _delta_process();

    fn get_component_types() -> Vec<TypeId>;
}
