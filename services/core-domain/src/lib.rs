//! sc_manager_core - minimal domain placeholders

// High-level modules derived from FINAL_REWORK.md
pub mod domain;
pub mod events;
pub mod repositories;
pub mod value_objects;

// Re-exports for convenience (add more as domain grows)
pub use domain::Member;
pub use domain::Organization;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_org() {
        let org = Organization::new("org1", "Test Org");
        assert_eq!(org.id, "org1");
        assert_eq!(org.name, "Test Org");
    }

    #[test]
    fn rename_org() {
        let mut org = Organization::new("org2", "Old");
        org.rename("New");
        assert_eq!(org.name, "New");
    }
}
