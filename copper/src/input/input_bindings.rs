use crate::input::action_state::Action;
use winit::event::MouseButton;
use std::collections::HashMap;
use winit::keyboard::KeyCode;
pub struct InputBindings {
    pub key_map: HashMap<KeyCode, Action>,
    pub mouse_map: HashMap<MouseButton, Action>,
}

impl InputBindings {
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
            mouse_map: HashMap::new(),
        }
    }
    //Binds key to action
    //engine.resources.input.binds.bind_key(KeyCode::KeyW, Action::Up);
    pub fn bind_key(&mut self, key: KeyCode, action: Action) -> &mut Self {
        self.key_map.insert(key, action);
        self
    }
    //Binds mouse button to action
    //engine.resources.input.binds.bind_mouse(MouseButton::Left, Action::Select);
    pub fn bind_mouse(&mut self, button: MouseButton, action: Action) -> &mut Self {
        self.mouse_map.insert(button, action);
        self
    }
}