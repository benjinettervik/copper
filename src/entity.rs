pub struct Entity {
    id: i32,
    components: Vec<Component>,
}

impl Entity {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
}
