use std::collections::HashMap;

use crate::vbox;
pub enum Value {
    VBox(vbox::VBox),
    Color(i64, i64, i64),
}
pub struct CMap {
    vboxes: Vec<HashMap<String, Value>>,
}

impl CMap {
    pub fn new(vboxes: Vec<HashMap<String, Value>>) -> Self {
        // Our Vboxes data structure looks like this
        // [
        //      { "vboxes": <Vbox_class>, "color": (123,123,123) }
        // ]
        return Self { vboxes: vboxes };
    }

    pub fn palette(&mut self) -> Vec<(i64, i64, i64)> {
        let return_vector: Vec<(i64, i64, i64)> = Vec::new();
        for item in &self.vboxes {
            match item.get("color") {
                Some(&value) => {}
                None => todo!(),
            }
        }
        return return_vector;
    }
}
