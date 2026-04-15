// // depicated lol

// #[allow(unused)]
// #[derive(Debug)]
// pub enum PropertyData {
//     Integer(i32),
//     Text(String),
//     Flag(bool),
// }

// #[derive(Debug)]
// pub struct Component {
//     name: String,
//     identifier: u32,
//     data: Vec<(String, PropertyData)>,
// }

// impl Component {
//     pub fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string(),
//             identifier: 0,
//             data: Vec::new(),
//         }
//     }

//     pub fn add_property(&mut self, prop: (String, PropertyData)) {
//         self.data.push(prop);
//     }

//     pub fn get_id(&self) -> u32 {
//         self.identifier
//     }
// }
