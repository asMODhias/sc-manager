pub struct StartSessionCommand {
    pub id: String,
    pub start_ts: i64,
    pub org_id: Option<String>,
}

impl StartSessionCommand {
    pub fn new(id: impl Into<String>, start_ts: i64, org_id: Option<String>) -> Self {
        Self {
            id: id.into(),
            start_ts,
            org_id,
        }
    }
}
