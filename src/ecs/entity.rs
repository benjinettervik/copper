// Private file.

//use super::component::Component;
use std::any::Any;

#[derive(Debug)]
pub struct Entity {
    id: usize,
    components: Vec<Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self {
            id: id,
            components: Vec::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }

    pub fn get_components(&self) -> &Vec<Box<dyn Any>> {
        &self.components
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
