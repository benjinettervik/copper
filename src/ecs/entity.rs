// Private file. 

use super::component::Component;

#[derive(Debug)]
pub struct Entity {
    
    id: u32,
    components: Vec<Component>,
}

impl Entity {
    pub fn new(id: &u32) -> Self {
        Self {
            id: id.clone(),
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
}
