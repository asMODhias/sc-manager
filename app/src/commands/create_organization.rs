pub struct CreateOrganizationCommand {
    pub id: String,
    pub name: String,
}

impl CreateOrganizationCommand {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}
