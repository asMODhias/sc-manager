use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::path::PathBuf;

pub mod storage;
use self::storage::{MarketplaceLedger, MarketplaceEvent};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub owner: String,
    pub price: u64,
    pub metadata: String,
}

#[derive(Debug, Error)]
pub enum MarketplaceError {
    #[error("item not found")]
    NotFound,
    #[error("item id already exists")]
    Exists,
    #[error("storage error: {0}")]
    Storage(#[from] crate::marketplace::storage::MarketplaceStorageError),
}

#[derive(Debug)]
pub struct Marketplace {
    // in-memory store reconstructed from events
    inner: RwLock<HashMap<String, Item>>,
    ledger: Option<MarketplaceLedger>,
}

impl Marketplace {
    /// In-memory only
    pub fn new() -> Self {
        Marketplace { inner: RwLock::new(HashMap::new()), ledger: None }
    }


    /// Persistent marketplace backed by NDJSON event file
    pub fn with_ledger(path: impl Into<PathBuf>) -> Result<Self, crate::marketplace::storage::MarketplaceStorageError> {
        let ledger = MarketplaceLedger::new(path);

        // If a snapshot exists, load state directly from snapshot for fast startup
        let snap = ledger.snapshot_path();
        if snap.exists() {
            let s = std::fs::read_to_string(&snap)?;
            let state: HashMap<String, Item> = serde_json::from_str(&s)?;
            return Ok(Marketplace { inner: RwLock::new(state), ledger: Some(ledger) });
        }

        // Otherwise, replay events from the ledger
        let events = ledger.load_all()?;
        let mut map = HashMap::new();
        for ev in events {
            match ev {
                MarketplaceEvent::Create { id, owner, price, metadata } => {
                    map.insert(id.clone(), Item { id, owner, price, metadata });
                }
                MarketplaceEvent::Remove { id } => {
                    map.remove(&id);
                }
            }
        }
        Ok(Marketplace { inner: RwLock::new(map), ledger: Some(ledger) })
    }

    /// Compact persistent storage by writing an atomic snapshot and rotating the ledger.
    pub async fn compact(&self) -> Result<(), MarketplaceError> {
        if let Some(ref ledger) = self.ledger {
            // capture current state
            let state = self.inner.read().await.clone();
            ledger.write_snapshot_atomic(&state).map_err(MarketplaceError::Storage)?;
            ledger.compact().map_err(MarketplaceError::Storage)?;
        }
        Ok(())
    }

    pub async fn list_items(&self) -> Vec<Item> {
        let m = self.inner.read().await;
        m.values().cloned().collect()
    }

    pub async fn list_item(&self, id: &str) -> Result<Item, MarketplaceError> {
        let m = self.inner.read().await;
        match m.get(id) {
            Some(i) => Ok(i.clone()),
            None => Err(MarketplaceError::NotFound),
        }
    }

    pub async fn insert_item(&self, item: Item) -> Result<(), MarketplaceError> {
        let mut m = self.inner.write().await;
        if m.contains_key(&item.id) {
            return Err(MarketplaceError::Exists);
        }
        // persist event if ledger is configured
        if let Some(ref ledger) = self.ledger {
            let ev = MarketplaceEvent::Create { id: item.id.clone(), owner: item.owner.clone(), price: item.price, metadata: item.metadata.clone() };
            ledger.append(&ev)?;
        }
        m.insert(item.id.clone(), item);
        Ok(())
    }
}

impl Default for Marketplace {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedMarketplace = Arc<Marketplace>;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_insert_and_list() {
        let mp = Marketplace::new();
        let s = Arc::new(mp);

        let it = Item { id: "item-1".into(), owner: "alice".into(), price: 100, metadata: "{}".into() };
        s.insert_item(it.clone()).await.expect("insert ok");

        let list = s.list_items().await;
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], it);

        let got = s.list_item("item-1").await.expect("found");
        assert_eq!(got, it);
    }

    #[tokio::test]
    async fn test_insert_duplicate() {
        let mp = Marketplace::new();
        let s = Arc::new(mp);
        let it = Item { id: "item-dup".into(), owner: "bob".into(), price: 50, metadata: "{}".into() };
        s.insert_item(it.clone()).await.expect("insert ok");
        let res = s.insert_item(it.clone()).await;
        assert!(matches!(res, Err(MarketplaceError::Exists)));
    }

    #[tokio::test]
    async fn test_persistent_ledger_replay() {
        let tf = NamedTempFile::new().expect("tmp");
        let p = tf.path().to_path_buf();

        // create marketplace with ledger and persist an item
        let mp = Marketplace::with_ledger(&p).expect("create with ledger");
        let s = Arc::new(mp);
        let it = Item { id: "item-persist".into(), owner: "carol".into(), price: 250, metadata: "{}".into() };
        s.insert_item(it.clone()).await.expect("insert ok");

        // compact storage
        s.compact().await.expect("compact ok");

        // create a new instance from same ledger and ensure item is present
        let mp2 = Marketplace::with_ledger(&p).expect("reload ledger");
        let s2 = Arc::new(mp2);
        let list = s2.list_items().await;
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], it);

        // ensure snapshot file exists
        let snap = crate::marketplace::storage::MarketplaceLedger::new(&p).snapshot_path();
        assert!(snap.exists());
    }
}
