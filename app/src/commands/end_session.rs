pub struct EndSessionCommand {
    pub id: String,
    pub end_ts: i64,
}

impl EndSessionCommand {
    pub fn new(id: impl Into<String>, end_ts: i64) -> Self {
        Self {
            id: id.into(),
            end_ts,
        }
    }
}
