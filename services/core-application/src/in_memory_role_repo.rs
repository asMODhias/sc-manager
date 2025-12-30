use sc_manager_core::domain::Role;
use sc_manager_core::repositories::{RepositoryError, RoleRepository};
use std::collections::HashMap;

pub struct InMemoryRoleRepo {
    store: HashMap<String, Role>,
}

impl InMemoryRoleRepo {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Default for InMemoryRoleRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl RoleRepository for InMemoryRoleRepo {
    fn create(&mut self, role: Role) -> Result<(), RepositoryError> {
        if self.store.contains_key(&role.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.store.insert(role.id.clone(), role);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Role, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn update(&mut self, role: Role) -> Result<(), RepositoryError> {
        if !self.store.contains_key(&role.id) {
            return Err(RepositoryError::NotFound);
        }
        self.store.insert(role.id.clone(), role);
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
