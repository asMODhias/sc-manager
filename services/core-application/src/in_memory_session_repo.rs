use sc_manager_core::domain::Session;
use sc_manager_core::repositories::{SessionRepository, RepositoryError};
use std::collections::HashMap;

pub struct InMemorySessionRepo { store: HashMap<String, Session> }
impl InMemorySessionRepo { pub fn new() -> Self { Self { store: HashMap::new() } } }
impl Default for InMemorySessionRepo { fn default() -> Self { Self::new() } }

impl SessionRepository for InMemorySessionRepo {
    fn create(&mut self, sess: Session) -> Result<(), RepositoryError> { self.store.insert(sess.id.clone(), sess); Ok(()) }
    fn get(&self, id: &str) -> Result<Session, RepositoryError> { self.store.get(id).cloned().ok_or(RepositoryError::NotFound) }
    fn update(&mut self, sess: Session) -> Result<(), RepositoryError> { if !self.store.contains_key(&sess.id) { return Err(RepositoryError::NotFound) } self.store.insert(sess.id.clone(), sess); Ok(()) }
    fn list_all(&self) -> Result<Vec<Session>, RepositoryError> { Ok(self.store.values().cloned().collect()) }
    fn list_by_org(&self, org_id: &str) -> Result<Vec<Session>, RepositoryError> { Ok(self.store.values().filter(|s| s.org_id.as_deref() == Some(org_id)).cloned().collect()) }
}
