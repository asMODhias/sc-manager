use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameEventType {
    SessionStart,
    SessionEnd,
    Kill,
    Death,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: String,
    pub event_type: GameEventType,
    pub timestamp: i64,
    /// Optional raw details or parsed metadata from the source (e.g. key=value pairs)
    pub details: Option<String>,
}

impl GameEvent {
    pub fn new(id: impl Into<String>, event_type: GameEventType, timestamp: i64) -> Self {
        Self {
            id: id.into(),
            event_type,
            timestamp,
            details: None,
        }
    }

    pub fn new_with_details(
        id: impl Into<String>,
        event_type: GameEventType,
        timestamp: i64,
        details: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            event_type,
            timestamp,
            details: Some(details.into()),
        }
    }
}
