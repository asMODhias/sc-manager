pub struct UpdateMemberCommand {
    pub id: String,
    pub rsi_handle: Option<String>,
    pub online: Option<bool>,
    pub org_id: Option<Option<String>>,
}

impl UpdateMemberCommand {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            rsi_handle: None,
            online: None,
            org_id: None,
        }
    }
}
