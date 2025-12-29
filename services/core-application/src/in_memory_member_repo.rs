use sc_manager_core::domain::Member;
use sc_manager_core::repositories::{MemberRepository, RepositoryError};
use std::collections::HashMap;

pub struct InMemoryMemberRepo {
    store: HashMap<String, Member>,
}

impl InMemoryMemberRepo {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }
}

impl Default for InMemoryMemberRepo {
    fn default() -> Self { Self::new() }
}

impl MemberRepository for InMemoryMemberRepo {
    fn add(&mut self, member: Member) -> Result<(), RepositoryError> {
        if self.store.contains_key(&member.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.store.insert(member.id.clone(), member);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Member, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn update(&mut self, member: Member) -> Result<(), RepositoryError> {
        if !self.store.contains_key(&member.id) {
            return Err(RepositoryError::NotFound);
        }
        self.store.insert(member.id.clone(), member);
        Ok(())
    }

    fn remove(&mut self, id: &str) -> Result<(), RepositoryError> {
        if self.store.remove(id).is_some() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    fn list_by_org(&self, org_id: &str) -> Result<Vec<Member>, RepositoryError> {
        let res = self.store.values().filter(|m| m.org_id.as_deref() == Some(org_id)).cloned().collect();
        Ok(res)
    }
}
