//! Persistence (Postgres scaffold) with an in-memory repo for tests

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type Id = String;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub id: Id,
    pub data: String,
}

pub trait Repository: Send + Sync {
    fn insert(&self, rec: Record);
    fn get(&self, id: &str) -> Option<Record>;
    fn begin_tx(&self);
    fn commit_tx(&self);
    fn rollback_tx(&self);
}

/// Simple in-memory repo that supports transactions via snapshotting
pub struct InMemoryRepo {
    data: Arc<Mutex<HashMap<Id, Record>>>,
    tx_snapshot: Arc<Mutex<Option<HashMap<Id, Record>>>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self { data: Arc::new(Mutex::new(HashMap::new())), tx_snapshot: Arc::new(Mutex::new(None)) }
    }
}

impl Repository for InMemoryRepo {
    fn insert(&self, rec: Record) {
        let mut map = self.data.lock().unwrap();
        map.insert(rec.id.clone(), rec);
    }

    fn get(&self, id: &str) -> Option<Record> {
        self.data.lock().unwrap().get(id).cloned()
    }

    fn begin_tx(&self) {
        let snap = self.data.lock().unwrap().clone();
        *self.tx_snapshot.lock().unwrap() = Some(snap);
    }

    fn commit_tx(&self) {
        *self.tx_snapshot.lock().unwrap() = None;
    }

    fn rollback_tx(&self) {
        if let Some(snap) = self.tx_snapshot.lock().unwrap().take() {
            *self.data.lock().unwrap() = snap;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tx_rollback_restores_state() {
        let repo = InMemoryRepo::new();
        repo.insert(Record { id: "1".into(), data: "a".into() });
        repo.begin_tx();
        repo.insert(Record { id: "2".into(), data: "b".into() });
        assert!(repo.get("2").is_some());
        repo.rollback_tx();
        assert!(repo.get("2").is_none());
        assert!(repo.get("1").is_some());
    }

    #[test]
    fn tx_commit_keeps_state() {
        let repo = InMemoryRepo::new();
        repo.begin_tx();
        repo.insert(Record { id: "x".into(), data: "z".into() });
        repo.commit_tx();
        assert!(repo.get("x").is_some());
    }
}
