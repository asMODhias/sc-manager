pub struct RegisterEquipmentCommand {
    pub id: String,
    pub name: String,
    pub read_only: bool,
}

impl RegisterEquipmentCommand {
    pub fn new(id: impl Into<String>, name: impl Into<String>, read_only: bool) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            read_only,
        }
    }
}
