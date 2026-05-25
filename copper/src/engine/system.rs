//! Contains the definition of a system. 

use crate::{ComponentId};
use crate::engine::world::*;
use crate::resource::Resources;
use std::any::TypeId;
// use std::fmt::Debug;
// This can probably be done without boilerplate : )

/// Defines which components a given system will read. This is used by the system scheduler.
#[macro_export]
macro_rules! components_read {
    ($( $t:ty ), *) => {
        fn components_read(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

/// Defines which components a given system will manipulate. This is used by the system scheduler.
#[macro_export]
macro_rules! components_write {
    ($( $t:ty ), *) => {
        fn components_write(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

/// Defines which components an entity will need have for a system to query for it. This is used by the system scheduler.
#[macro_export]
macro_rules! components_with {
    ($( $t:ty ), *) => {
        fn components_with(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

/// Defines which components an entity will need NOT have for a system to query for it. This is used by the system scheduler.
#[macro_export]
macro_rules! components_without {
    ($( $t:ty ), *) => {
        fn components_without(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}

#[macro_export]
macro_rules! resources_read {
    ($( $t:ty ), *) => {
        fn resources_read(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}
#[macro_export]
macro_rules! resources_write {
    ($( $t:ty ), *) => {
        fn resources_write(&self) -> Vec<TypeId> {
            vec![
                $(
                    TypeId::of::<$t>(),
                )*
            ]
        }
    };
}
#[macro_export]
macro_rules! system_id {
    () => {
        fn sys_id(&self) -> TypeId {
            TypeId::of::<Self>()
        }
    };
}

/// Defines the structure of a system.

pub trait System {
    // with this implementation we will have to trust the user doesn't fetch other components
    // than what's specified
    
    /// Defines which components a given system will read. This is used by the system scheduler.
    fn components_read(&self) -> Vec<ComponentId>;
    
    /// Defines which components a given system will manipulate. This is used by the system scheduler.  
    fn components_write(&self) -> Vec<ComponentId>;

    fn resources_read(&self) -> Vec<TypeId>;
    fn resources_write(&self) -> Vec<TypeId>;
    fn sys_id(&self) -> TypeId;
    /// Defines which components an entity will need have for a system to query for it. This is used by the system scheduler.  
    fn components_with(&self) -> Vec<ComponentId>;

    /// Defines which components an entity will need NOT have for a system to query for it. This is used by the system scheduler.
    fn components_without(&self) -> Vec<ComponentId>;

    /// The function that will be run by the engine on start. 
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}
