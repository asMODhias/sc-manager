pub struct AssignPermissionToRoleCommand {
    pub role_id: String,
    pub permission_id: String,
}

impl AssignPermissionToRoleCommand {
    pub fn new(role_id: impl Into<String>, permission_id: impl Into<String>) -> Self {
        Self {
            role_id: role_id.into(),
            permission_id: permission_id.into(),
        }
    }
}
