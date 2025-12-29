use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    pub id: String,
    pub model: String,
    pub owner_org: Option<String>,
}

impl Ship {
    pub fn new(id: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            model: model.into(),
            owner_org: None,
        }
    }
}
