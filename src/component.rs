
#[derive(Debug)]
pub enum PropertyData {
    Integer(i32),
    Text(String),
    Flag(bool),
}

#[derive(Debug)]
pub struct Component {
    pub data: Vec<(String, PropertyData)>, // maybe change to private?
}

impl Default for Component {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl Component {
    pub fn add_property(&mut self, prop: (String, PropertyData)) {
        self.data.push(prop);
    }
}
