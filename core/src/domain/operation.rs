use std::time::{SystemTime, UNIX_EPOCH};

/// Represents an Operation in the domain
#[derive(Debug, Clone)]
pub struct Operation {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: OperationStatus,
    pub phases: Vec<Phase>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationStatus {
    Planning,
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Phase {
    pub id: String,
    pub name: String,
    pub completed_at: Option<i64>,
}

impl Operation {
    pub fn new(id: &str, name: &str, description: &str, ts: i64) -> Self {
        Operation {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status: OperationStatus::Planning,
            phases: Vec::new(),
            created_at: ts,
            updated_at: ts,
        }
    }

    pub fn add_phase(&mut self, id: &str, name: &str) {
        let p = Phase {
            id: id.to_string(),
            name: name.to_string(),
            completed_at: None,
        };
        self.phases.push(p);
        self.updated_at = now_seconds();
    }

    pub fn start(&mut self) {
        if self.status == OperationStatus::Planning {
            self.status = OperationStatus::Active;
            self.updated_at = now_seconds();
        }
    }

    pub fn complete_phase(&mut self, phase_id: &str) {
        for p in &mut self.phases {
            if p.id == phase_id {
                p.completed_at = Some(now_seconds());
            }
        }
        self.updated_at = now_seconds();
        if self.phases.iter().all(|p| p.completed_at.is_some()) && !self.phases.is_empty() {
            self.status = OperationStatus::Completed;
        }
    }

    pub fn cancel(&mut self) {
        self.status = OperationStatus::Cancelled;
        self.updated_at = now_seconds();
    }
}

fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
