use sc_manager_core::domain::Fleet;
use sc_manager_core::repositories::{FleetRepository, RepositoryError};
use std::collections::HashMap;

pub struct InMemoryFleetRepo {
    store: HashMap<String, Fleet>,
}

impl InMemoryFleetRepo {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Default for InMemoryFleetRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetRepository for InMemoryFleetRepo {
    fn create(&mut self, fleet: Fleet) -> Result<(), RepositoryError> {
        if self.store.contains_key(&fleet.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.store.insert(fleet.id.clone(), fleet);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Fleet, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn update(&mut self, fleet: Fleet) -> Result<(), RepositoryError> {
        if !self.store.contains_key(&fleet.id) {
            return Err(RepositoryError::NotFound);
        }
        self.store.insert(fleet.id.clone(), fleet);
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
