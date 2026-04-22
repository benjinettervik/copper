#![allow(unused)]
use std::collections::HashSet;
use winit::keyboard::KeyCode;
use winit::event::{KeyEvent, ElementState};
use winit::keyboard::PhysicalKey;

#[derive(Debug)]
pub struct Input {
    pub pressed: HashSet<KeyCode>,
    pub just_pressed: HashSet<KeyCode>,
    pub just_released: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn handle_input(&mut self, event: KeyEvent) {
        if let PhysicalKey::Code(code) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    // only trigger once
                    if !self.pressed.contains(&code) {
                        self.just_pressed.insert(code);
                    }
                    self.pressed.insert(code);
                }

                ElementState::Released => {
                    self.pressed.remove(&code);
                    self.just_released.insert(code);
                }
            }
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }

    pub fn clear_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}
