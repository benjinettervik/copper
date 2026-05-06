use component_macro_derive::Component;

use crate::{components_read, components_with, components_without, components_write};
use crate::resource::Resources;
use crate::engine::system::System;
use crate::engine::world::World;
use std::any::TypeId;
use crate::Component;

#[derive(Component)]
pub struct Selected {
    selected: bool,
}

pub struct ActionSystem;

impl System for ActionSystem {
    components_write!();
    components_read!(Selected);
    components_with!();
    components_without!();

    fn run(&mut self, world: &mut World, resources: &mut Resources) {   
        let input = &resources.input;

        let entities = world.query(
            &self.components_read(),
            &self.components_write(),
            &self.components_with(),
            &self.components_without(),
        );

        //let entities = query!(world, self);

        for entity in &entities {
            
            let selected = world.get_component::<Selected>(*entity).unwrap();

            for (key, function) in &input.binds.key_map{
                if input.state.keys_pressed.contains(key) {
                    if selected.selected ==  true {
                        function(*entity);
                    }
                    
                } 
            }

            for (button, function) in &input.binds.mouse_map {
                if input.state.mouse_buttons_pressed.contains(&button) {
                    if selected.selected ==  true {
                        function(*entity);
                    }
                }
            }
        }
    }
}
