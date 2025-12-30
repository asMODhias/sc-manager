pub struct AddMemberCommand {
    pub id: String,
    pub rsi_handle: Option<String>,
    pub org_id: Option<String>,
}

impl AddMemberCommand {
    pub fn new(id: impl Into<String>, rsi_handle: Option<String>, org_id: Option<String>) -> Self {
        Self {
            id: id.into(),
            rsi_handle,
            org_id,
        }
    }
}
