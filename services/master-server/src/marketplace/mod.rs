use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

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
}

#[derive(Debug, Default)]
pub struct Marketplace {
    // simple in-memory store for now; persistent storage can be added later
    inner: RwLock<HashMap<String, Item>>,
}

impl Marketplace {
    pub fn new() -> Self {
        Marketplace { inner: RwLock::new(HashMap::new()) }
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
        m.insert(item.id.clone(), item);
        Ok(())
    }
}

pub type SharedMarketplace = Arc<Marketplace>;

#[cfg(test)]
mod tests {
    use super::*;

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
}
