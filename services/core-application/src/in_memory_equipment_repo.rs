use sc_manager_core::domain::Equipment;
use sc_manager_core::repositories::{EquipmentRepository, RepositoryError};
use std::collections::HashMap;

pub struct InMemoryEquipmentRepo {
    store: HashMap<String, Equipment>,
}

impl InMemoryEquipmentRepo {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Default for InMemoryEquipmentRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl EquipmentRepository for InMemoryEquipmentRepo {
    fn register(&mut self, equipment: Equipment) -> Result<(), RepositoryError> {
        if self.store.contains_key(&equipment.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.store.insert(equipment.id.clone(), equipment);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Equipment, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn list_all(&self) -> Result<Vec<Equipment>, RepositoryError> {
        Ok(self.store.values().cloned().collect())
    }
}
