use sc_manager_core::domain::Organization;
use sc_manager_core::repositories::RepositoryError;

#[allow(dead_code)]
pub struct DummyPermissionRepo;
impl sc_manager_core::repositories::PermissionRepository for DummyPermissionRepo {
    fn create(
        &mut self,
        _permission: sc_manager_core::domain::Permission,
    ) -> Result<(), RepositoryError> {
        Ok(())
    }
    fn get(&self, _id: &str) -> Result<sc_manager_core::domain::Permission, RepositoryError> {
        Err(RepositoryError::NotFound)
    }
    fn list_all(&self) -> Result<Vec<sc_manager_core::domain::Permission>, RepositoryError> {
        Ok(vec![])
    }
}

// Simple in-memory org repo for tests
#[allow(dead_code)]
pub struct InMemoryOrganizationRepo {
    store: std::collections::HashMap<String, Organization>,
}
#[allow(dead_code)]
impl InMemoryOrganizationRepo {
    pub fn new() -> Self {
        Self {
            store: std::collections::HashMap::new(),
        }
    }
}
impl sc_manager_core::repositories::OrganizationRepository for InMemoryOrganizationRepo {
    fn create(&mut self, org: Organization) -> Result<(), RepositoryError> {
        if self.store.contains_key(&org.id) {
            Err(RepositoryError::AlreadyExists)
        } else {
            self.store.insert(org.id.clone(), org);
            Ok(())
        }
    }
    fn get(&self, id: &str) -> Result<Organization, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }
    fn update(&mut self, org: Organization) -> Result<(), RepositoryError> {
        if self.store.contains_key(&org.id) {
            self.store.insert(org.id.clone(), org);
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
    fn delete(&mut self, id: &str) -> Result<(), RepositoryError> {
        if self.store.remove(id).is_some() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}
