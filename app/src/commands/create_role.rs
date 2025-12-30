pub struct CreateRoleCommand {
    pub id: String,
    pub name: String,
}

impl CreateRoleCommand {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}
