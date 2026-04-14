
#[allow(unused)]
#[derive(Debug)]
pub enum PropertyData {
    Integer(i32),
    Text(String),
    Flag(bool),
}

#[derive(Debug)]
pub struct Component {
    identifier: u32,
    data: Vec<(String, PropertyData)>,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            identifier: 1, // FIXME: add UNIQUE id!!!! 
            data: Vec::new() 
        }
    }
}

impl Component {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_property(&mut self, prop: (String, PropertyData)) {
        self.data.push(prop);
    }

    pub fn get_id(&self) -> u32 {
        self.identifier
    }
}
