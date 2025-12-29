use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
}

impl Organization {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    pub fn rename(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }
}
