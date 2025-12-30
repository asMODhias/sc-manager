use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub timestamp: i64,
}

impl Event {
    pub fn new(id: impl Into<String>, title: impl Into<String>, timestamp: i64) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            timestamp,
        }
    }
}
