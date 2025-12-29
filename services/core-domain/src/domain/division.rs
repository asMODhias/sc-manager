use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Division {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
}

impl Division {
    pub fn new(id: impl Into<String>, name: impl Into<String>, parent: Option<String>) -> Self {
        Self { id: id.into(), name: name.into(), parent }
    }
}
