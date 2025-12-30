use sc_manager_core::domain::Event;
use sc_manager_core::repositories::{EventRepository, RepositoryError};
use std::collections::VecDeque;

pub struct InMemoryEventRepo {
    store: VecDeque<Event>,
}

impl InMemoryEventRepo {
    pub fn new() -> Self {
        Self {
            store: VecDeque::new(),
        }
    }
}

impl Default for InMemoryEventRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl EventRepository for InMemoryEventRepo {
    fn append(&mut self, event: Event) -> Result<(), RepositoryError> {
        self.store.push_back(event);
        Ok(())
    }

    fn list_all(&self) -> Result<Vec<Event>, RepositoryError> {
        Ok(self.store.iter().cloned().collect())
    }

    fn list_by_org(&self, org_id: &str) -> Result<Vec<Event>, RepositoryError> {
        let mut out = vec![];
        for e in self.store.iter() {
            if e.title.contains(org_id) {
                out.push(e.clone());
            }
        }
        Ok(out)
    }
}
