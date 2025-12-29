use sc_manager_core::domain::Organization;
use sc_manager_core::repositories::{OrganizationRepository, RepositoryError};
use std::collections::HashMap;

pub struct InMemoryOrganizationRepo {
    store: HashMap<String, Organization>,
}

impl InMemoryOrganizationRepo {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Default for InMemoryOrganizationRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationRepository for InMemoryOrganizationRepo {
    fn create(&mut self, org: Organization) -> Result<(), RepositoryError> {
        if self.store.contains_key(&org.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.store.insert(org.id.clone(), org);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Organization, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn update(&mut self, org: Organization) -> Result<(), RepositoryError> {
        if !self.store.contains_key(&org.id) {
            return Err(RepositoryError::NotFound);
        }
        self.store.insert(org.id.clone(), org);
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), RepositoryError> {
        if self.store.remove(id).is_some() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}

// Note: In-memory member repo lives in `in_memory_member_repo.rs` to keep responsibilities separated.
