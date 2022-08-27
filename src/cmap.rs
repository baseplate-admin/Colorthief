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
        return Self { vboxes: vboxes };
    }

    pub fn palette(&self) -> Vec<(i64, i64, i64)> {
        let data = self.vboxes.iter().map(|item| {
            item.into_iter()
                .filter(|&(k, _)| (k.to_lowercase()) == "color")
                .map(|(_, value)| value)
        });
        return vec![];
    }
}
