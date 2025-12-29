use sc_manager_core::domain::Permission;
use sc_manager_core::repositories::{PermissionRepository, RepositoryError};
use std::collections::HashMap;

pub struct InMemoryPermissionRepo {
    store: HashMap<String, Permission>,
}

impl InMemoryPermissionRepo {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Default for InMemoryPermissionRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl PermissionRepository for InMemoryPermissionRepo {
    fn create(&mut self, permission: Permission) -> Result<(), RepositoryError> {
        if self.store.contains_key(&permission.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.store.insert(permission.id.clone(), permission);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Permission, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn list_all(&self) -> Result<Vec<Permission>, RepositoryError> {
        Ok(self.store.values().cloned().collect())
    }
}
