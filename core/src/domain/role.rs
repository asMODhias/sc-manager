use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
}

impl Role {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            permissions: vec![],
        }
    }

    pub fn add_permission(&mut self, permission_id: impl Into<String>) {
        let pid = permission_id.into();
        if !self.permissions.iter().any(|p| p == &pid) {
            self.permissions.push(pid);
        }
    }

    pub fn remove_permission(&mut self, permission_id: &str) {
        self.permissions.retain(|p| p != permission_id);
    }

    pub fn list_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }
}
