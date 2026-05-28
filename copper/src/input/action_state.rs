use std::collections::HashSet;
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
//more actions can be added here, unclear future for this guy
pub enum Action {
    Select,
    Up,
    Down,
    Left,
    Right,
}
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

