use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::event::ElementState;
use std::collections::HashSet;
use winit::keyboard::PhysicalKey;
use winit::event::KeyEvent;
pub struct InputState {
    pub keys_down: HashSet<KeyCode>,
    pub keys_pressed: HashSet<KeyCode>,
    pub keys_released: HashSet<KeyCode>, //may remove

    pub mouse_pos: (f64 , f64),
    pub mouse_buttons_down: HashSet<MouseButton>,
    pub mouse_buttons_pressed: HashSet<MouseButton>,
    pub mouse_buttons_released: HashSet<MouseButton>, //may remove

}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(), //may remove
            mouse_pos: (0.0, 0.0),
            mouse_buttons_down: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_released: HashSet::new(), //may remove
        }
    }
    
    /// Handle input events, currently pressed keys will be added to keys_down and keys_pressed, when the key gets 
    /// released it will be removed from keys_down and added to keys_released
    /// When a key gets removed from pressed it gets inserted into released but we dont have a proper plan for now
    /// self.resources.input.handle_keys(event);
    pub fn handle_keys(&mut self, event: KeyEvent) {
        if let PhysicalKey::Code(keys) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    if !self.keys_down.contains(&keys) {
                        self.keys_pressed.insert(keys);
                    }
                    self.keys_down.insert(keys);
                }
                ElementState::Released => {
                    self.keys_down.remove(&keys);
                    self.keys_released.insert(keys);
                }
            }
        }
    }

    /// Handle mouse movement and save mouse coords
    /// self.resources.input.handle_mouse_movement(position.x, position.y);
    pub fn handle_mouse_movement(&mut self, x: f64, y: f64) {
        self.mouse_pos = (x, y);
    }

    /// Handle mouse button events, similar behavior to handle_keys
    /// self.resources.input.handle_mouse_button(button, state);
    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if !self.mouse_buttons_down.contains(&button) {
                    self.mouse_buttons_pressed.insert(button);
                }
                self.mouse_buttons_down.insert(button);
            }
            ElementState::Released => {
                self.mouse_buttons_down.remove(&button);
                self.mouse_buttons_released.insert(button);
            }
        }
    }

    /// To clear all HashSets for next tick/frame
    pub fn clear_inputs(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
    }

    //Helper functions to check if a key or mouse button is down, pressed or released
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }
}