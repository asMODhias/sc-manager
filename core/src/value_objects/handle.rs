#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle(pub String);

impl Handle {
    pub fn is_valid(&self) -> bool {
        // Placeholder: real validation to be implemented later
        !self.0.is_empty()
    }
}
