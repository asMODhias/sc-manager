use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub start_ts: i64,
    pub end_ts: Option<i64>,
    pub events: Vec<String>,
    pub org_id: Option<String>,
    pub participant: Option<String>,
}

impl Session {
    pub fn new(
        id: impl Into<String>,
        start_ts: i64,
        org_id: Option<String>,
        participant: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            start_ts,
            end_ts: None,
            events: vec![],
            org_id,
            participant,
        }
    }

    pub fn end(&mut self, ts: i64) {
        self.end_ts = Some(ts);
    }

    pub fn add_event(&mut self, event_id: impl Into<String>) {
        self.events.push(event_id.into());
    }

    pub fn is_active(&self) -> bool {
        self.end_ts.is_none()
    }
}
