use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    /// Optional divisions/sub-units inside an organization (e.g. wings, squads)
    pub divisions: Vec<crate::domain::Division>,
}

impl Organization {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            divisions: vec![],
        }
    }

    pub fn rename(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }

    pub fn add_division(&mut self, division: crate::domain::Division) {
        if !self.divisions.iter().any(|d| d.id == division.id) {
            self.divisions.push(division);
        }
    }

    pub fn list_divisions(&self) -> Vec<crate::domain::Division> {
        self.divisions.clone()
    }

    pub fn remove_division(&mut self, division_id: &str) {
        self.divisions.retain(|d| d.id != division_id);
    }
}
