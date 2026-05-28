pub mod input_state;
pub mod action_state;
pub mod input_bindings;
use winit::event::MouseButton;
use winit::event::ElementState;
use winit::event::KeyEvent;
use crate::input::input_state::InputState;
use crate::input::action_state::ActionState;
use crate::input::input_bindings::InputBindings;

pub struct Input {
    pub state: InputState,
    pub binds: InputBindings,
    pub actions: ActionState,
}

impl Input {
    pub fn new() -> Self {
        Self {
            state: InputState::new(),
            binds: InputBindings::new(),
            actions: ActionState::new(),
        }
    }
    //Handles all input events and updates the state, updates the active actions based on binds
    pub fn handle_keys(&mut self, event: KeyEvent) {
        self.state.handle_keys(event);
    }
    pub fn handle_mouse_movement(&mut self, x: f64, y: f64) {
        self.state.handle_mouse_movement(x, y);
    }
    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        self.state.handle_mouse_button(button, state);
    }

    //Called once per tick/frame to update actions, (will be changed for our new intended system plan)
    pub fn input_polling(&mut self) {
        
        self.actions.active.clear();
        self.actions.just_pressed.clear();
        self.actions.just_released.clear();

        for (key, action) in &self.binds.key_map {
            if self.state.keys_down.contains(key) {
                self.actions.active.insert(*action);
            }
            if self.state.keys_pressed.contains(key) {
                self.actions.just_pressed.insert(*action);
            }
            if self.state.keys_released.contains(key) {
                self.actions.just_released.insert(*action);
            }
        }

        for (button, action) in &self.binds.mouse_map {
            if self.state.mouse_buttons_down.contains(button) {
                self.actions.active.insert(*action);
            }
            if self.state.mouse_buttons_pressed.contains(button) {
                self.actions.just_pressed.insert(*action);
            }
        }

        self.state.clear_inputs();
    }
}