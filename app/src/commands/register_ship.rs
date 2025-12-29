pub struct RegisterShipCommand {
    pub id: String,
    pub model: String,
    pub owner_org: Option<String>,
}

impl RegisterShipCommand {
    pub fn new(id: impl Into<String>, model: impl Into<String>, owner_org: Option<String>) -> Self {
        Self {
            id: id.into(),
            model: model.into(),
            owner_org,
        }
    }
}
