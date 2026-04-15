use std::any::TypeId;

#[macro_export]
macro_rules! query_for_components {
    ($( $t:ty ), *) => {
        fn get_component_types() -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}


pub trait System {
    fn get_component_types() -> Vec<TypeId>;
    
    fn _on_ready();

    fn _process();

    fn _delta_process();
}
