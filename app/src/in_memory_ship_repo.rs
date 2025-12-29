use sc_manager_core::domain::Ship;
use sc_manager_core::repositories::{RepositoryError, ShipRepository};
use std::collections::HashMap;

pub struct InMemoryShipRepo {
    store: HashMap<String, Ship>,
}

impl InMemoryShipRepo {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Default for InMemoryShipRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipRepository for InMemoryShipRepo {
    fn register(&mut self, ship: Ship) -> Result<(), RepositoryError> {
        if self.store.contains_key(&ship.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.store.insert(ship.id.clone(), ship);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Ship, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn update(&mut self, ship: Ship) -> Result<(), RepositoryError> {
        if !self.store.contains_key(&ship.id) {
            return Err(RepositoryError::NotFound);
        }
        self.store.insert(ship.id.clone(), ship);
        Ok(())
    }

    fn remove(&mut self, id: &str) -> Result<(), RepositoryError> {
        if self.store.remove(id).is_some() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    fn list_by_owner_org(&self, org_id: &str) -> Result<Vec<Ship>, RepositoryError> {
        let mut out = vec![];
        for s in self.store.values() {
            if let Some(ref o) = s.owner_org {
                if o == org_id {
                    out.push(s.clone());
                }
            }
        }
        Ok(out)
    }
}
