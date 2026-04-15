use std::any::TypeId;

#[macro_export]
macro_rules! query_for_components {
    ($( $t:ty ), *) => {
        fn get_component_types(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}


pub trait System {
    fn get_component_types(&self) -> Vec<TypeId>;
    
    fn _on_ready(&self);

    fn _process(&self);

    fn _delta_process(&self);
}
