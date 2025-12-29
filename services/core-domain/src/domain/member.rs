use serde::{Deserialize, Serialize};

/// Representation of a Member in the Core Domain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub role_id: String,
    pub resource_id: Option<String>,
}

impl RoleAssignment {
    pub fn new(role_id: impl Into<String>, resource_id: Option<String>) -> Self {
        Self {
            role_id: role_id.into(),
            resource_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub rsi_handle: Option<String>,
    pub online: bool,
    pub org_id: Option<String>, // optional association to Organization
    pub last_seen: Option<i64>,
    pub last_session_id: Option<String>,
    pub roles: Vec<RoleAssignment>,
}

impl Member {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            rsi_handle: None,
            online: false,
            org_id: None,
            last_seen: None,
            last_session_id: None,
            roles: vec![],
        }
    }

    pub fn assign_role(&mut self, role_id: impl Into<String>, resource_id: Option<String>) {
        let r = RoleAssignment::new(role_id, resource_id);
        if !self
            .roles
            .iter()
            .any(|ra| ra.role_id == r.role_id && ra.resource_id == r.resource_id)
        {
            self.roles.push(r);
        }
    }

    pub fn revoke_role(&mut self, role_id: &str, resource_id: Option<&str>) {
        self.roles
            .retain(|ra| !(ra.role_id == role_id && ra.resource_id.as_deref() == resource_id));
    }

    pub fn assign_to_org(&mut self, org_id: impl Into<String>) {
        self.org_id = Some(org_id.into());
    }

    pub fn unassign_org(&mut self) {
        self.org_id = None;
    }

    // TODO: Add verification logic for RSI handle
}
