use sc_manager_core::domain::Event;
use sc_manager_core::repositories::{EventRepository, RepositoryError};

pub struct InMemoryEventRepo {
    store: Vec<Event>,
}

impl InMemoryEventRepo {
    pub fn new() -> Self { Self { store: vec![] } }
}

impl Default for InMemoryEventRepo { fn default() -> Self { Self::new() } }

impl EventRepository for InMemoryEventRepo {
    fn append(&mut self, event: Event) -> Result<(), RepositoryError> { self.store.push(event); Ok(()) }
    fn list_all(&self) -> Result<Vec<Event>, RepositoryError> { Ok(self.store.clone()) }
    fn list_by_org(&self, _org_id: &str) -> Result<Vec<Event>, RepositoryError> { Ok(vec![]) }
}
