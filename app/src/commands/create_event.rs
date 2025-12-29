pub struct CreateEventCommand {
    pub id: String,
    pub title: String,
    pub timestamp: i64,
    pub org_id: Option<String>,
}

impl CreateEventCommand {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        timestamp: i64,
        org_id: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            timestamp,
            org_id,
        }
    }
}
