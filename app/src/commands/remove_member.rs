pub struct RemoveMemberCommand {
    pub id: String,
}

impl RemoveMemberCommand {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
