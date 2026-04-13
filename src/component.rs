enum PropertyData {
    Integer(i32),
    Text(String),
    Flag(bool),
}

pub struct Component {
    data: Vec<(String, PropertyData)>,
}

impl Default for Component {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl Component {
    fn add_property(&mut self, prop: (String, PropertyData)) {
        self.data.push(prop);
    }
}
