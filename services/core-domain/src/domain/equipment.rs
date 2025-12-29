use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub id: String,
    pub name: String,
    pub read_only: bool, // e.g. data from Erkul
}

impl Equipment {
    pub fn new(id: impl Into<String>, name: impl Into<String>, read_only: bool) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            read_only,
        }
    }
}
