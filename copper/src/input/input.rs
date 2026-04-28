use std::collections::HashMap;
use std::collections::HashSet;
use winit::event::ElementState;
use winit::event::KeyEvent;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
//Component for saving all inputs
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
    
    //Handle input events, currently pressed keys will be added to keys_down and keys_pressed, when the key gets 
    //released it will be removed from keys_down and added to keys_released
    //When a key gets removed from pressed it gets inserted into released but we dont have a proper plan for now
    //self.resources.input.handle_keys(event);
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
    //Handle mouse movement and save mouse coords
    //self.resources.input.handle_mouse_movement(position.x, position.y);
    pub fn handle_mouse_movement(&mut self, x: f64, y: f64) {
        self.mouse_pos = (x, y);
    }
    //Handle mouse button events, similar behavior to handle_keys
    //self.resources.input.handle_mouse_button(button, state);
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
    //To clear all HashSets for next tick/frame
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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
//more actions can be added here, unclear future for this guy
pub enum Action {
Select,
Up,
Down,
Left,
Right,
}

//Component for saving all input binds
pub struct InputBindings {
    key_map: HashMap<KeyCode, Action>,
    mouse_map: HashMap<MouseButton, Action>,
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

//Component for saving all active actions, likley used for match_action system
pub struct ActionState {
    pub active: HashSet<Action>,
    pub just_pressed: HashSet<Action>,
    pub just_released: HashSet<Action>,
}

impl ActionState {
    pub fn new() -> Self {
        Self {
            active: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(), //likley not needed
        }
    }

    //Helper functions to check if an action is active, just pressed
    pub fn is_active(&self, action: Action) -> bool {
        self.active.contains(&action)
    }

    pub fn is_just_pressed(&self, action: Action) -> bool {
        self.just_pressed.contains(&action)
    }

    pub fn is_just_released(&self, action: Action) -> bool {
        self.just_released.contains(&action)
    }
}

//Main input component that will be added to resources, contains InputState, InputBindings and ActionState
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