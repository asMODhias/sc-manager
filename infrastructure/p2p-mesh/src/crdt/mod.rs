use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Initial, simple CRDTStateManager implementation using in-memory state.
/// This is a functional adapter that provides the same API surface planned
/// for the Automerge-backed implementation; it is intentionally simple and
/// serializable so it can be used in tests and early integration.
#[derive(Clone)]
pub struct CRDTStateManager {
    inner: Arc<RwLock<HashMap<String, OrgState>>>,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrgState {
    pub id: String,
    pub name: String,
    pub member_count: u32,
}

#[derive(Debug, Error)]
pub enum CRDTError {
    #[error("Not found")]
    NotFound,

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl CRDTStateManager {
    pub fn new(actor_id: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            actor_id: actor_id.into(),
        }
    }

    /// Update organization state (creates or updates)
    pub async fn update_org(&self, org_id: &str, field: &str, value: serde_json::Value) -> Result<(), CRDTError> {
        let mut guard: tokio::sync::RwLockWriteGuard<'_, HashMap<String, OrgState>> = self.inner.write().await;
        let entry = guard.entry(org_id.to_string()).or_insert_with(|| OrgState {
            id: org_id.to_string(),
            name: String::new(),
            member_count: 0,
        });

        match (field, value) {
            ("name", serde_json::Value::String(s)) => entry.name = s,
            ("member_count", serde_json::Value::Number(n)) => {
                if let Some(i) = n.as_u64() {
                    entry.member_count = i as u32;
                }
            }
            (_f, _) => {
                // For unknown fields, ignore silently for now
            }
        }

        Ok(())
    }

    /// Get organization state
    pub async fn get_org(&self, org_id: &str) -> Result<OrgState, CRDTError> {
        let guard: tokio::sync::RwLockReadGuard<'_, HashMap<String, OrgState>> = self.inner.read().await;
        match guard.get(org_id) {
            Some(s) => Ok(s.clone()),
            None => Err(CRDTError::NotFound),
        }
    }

    /// Generate a sync message for a peer - here: JSON of the single org state
    pub async fn generate_sync_message(&self, _peer_id: &str) -> Result<Vec<u8>, CRDTError> {
        // For now send full state for simplicity
        let guard = self.inner.read().await;
        let b = serde_json::to_vec(&*guard)?;
        Ok(b)
    }

    /// Apply a sync message from a peer (merge states)
    pub async fn apply_sync_message(&self, _peer_id: &str, message: &[u8]) -> Result<(), CRDTError> {
        let other: HashMap<String, OrgState> = serde_json::from_slice(message)?;
        let mut guard: tokio::sync::RwLockWriteGuard<'_, HashMap<String, OrgState>> = self.inner.write().await;

        for (k, v) in other {
            // naive merge: replace or insert
            let entry = guard.entry(k).or_insert(v.clone());
            // Merge simple fields: prefer higher member_count and non-empty names
            if v.member_count > entry.member_count {
                entry.member_count = v.member_count;
            }
            if !v.name.is_empty() {
                entry.name = v.name;
            }
        }

        Ok(())
    }

    /// Export full state for persistence
    pub async fn export_state(&self) -> Result<Vec<u8>, CRDTError> {
        let guard = self.inner.read().await;
        let b = serde_json::to_vec(&*guard)?;
        Ok(b)
    }

    /// Import state from persistence (overwrites local state)
    pub async fn import_state(&self, data: &[u8]) -> Result<(), CRDTError> {
        let map: HashMap<String, OrgState> = serde_json::from_slice(data)?;
        let mut guard = self.inner.write().await;
        *guard = map;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_and_get_org() {
        let cm = CRDTStateManager::new("actor-1");
        cm.update_org("org-1", "name", serde_json::Value::String("ACME".into())).await.expect("update");
        cm.update_org("org-1", "member_count", serde_json::Value::Number(serde_json::Number::from(42))).await.expect("update");

        let org = cm.get_org("org-1").await.expect("get");
        assert_eq!(org.name, "ACME");
        assert_eq!(org.member_count, 42);
    }

    #[tokio::test]
    async fn test_export_and_import_state() {
        let cm = CRDTStateManager::new("actor-2");
        cm.update_org("org-2", "name", serde_json::Value::String("Beta".into())).await.expect("update");
        let b = cm.export_state().await.expect("export");

        let cm2 = CRDTStateManager::new("actor-3");
        cm2.import_state(&b).await.expect("import");
        let org = cm2.get_org("org-2").await.expect("get");
        assert_eq!(org.name, "Beta");
    }

    #[tokio::test]
    async fn test_generate_and_apply_sync_message() {
        let cm1 = CRDTStateManager::new("a1");
        cm1.update_org("org-sync", "name", serde_json::Value::String("G1".into())).await.expect("update");

        let cm2 = CRDTStateManager::new("a2");
        cm2.update_org("org-sync", "member_count", serde_json::Value::Number(serde_json::Number::from(5))).await.expect("update");

        let msg = cm1.generate_sync_message("peer-a2").await.expect("gen");
        cm2.apply_sync_message("peer-a1", &msg).await.expect("apply");

        let org = cm2.get_org("org-sync").await.expect("get");
        // name should be merged from cm1, member_count preserved from cm2
        assert_eq!(org.name, "G1");
        assert_eq!(org.member_count, 5);
    }
}