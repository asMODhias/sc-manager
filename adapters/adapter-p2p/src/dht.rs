use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Minimal DHT registry trait — announcement and lookup
pub trait DhtRegistry: Send + Sync {
    fn announce(&self, id: &str, public_key_b64: &str);
    fn lookup(&self, id: &str) -> Option<String>;
}

/// In-memory DHT used for tests and local discovery
#[derive(Clone)]
pub struct InMemoryDht {
    map: Arc<Mutex<HashMap<String, String>>>,
}

impl InMemoryDht {
    pub fn new() -> Self {
        Self { map: Arc::new(Mutex::new(HashMap::new())) }
    }
}

impl DhtRegistry for InMemoryDht {
    fn announce(&self, id: &str, public_key_b64: &str) {
        if let Ok(mut guard) = self.map.lock() {
            guard.insert(id.to_string(), public_key_b64.to_string());
        } else {
            tracing::error!("InMemoryDht::announce: mutex poisoned for id={}", id);
        }
    }

    fn lookup(&self, id: &str) -> Option<String> {
        self.map.lock().ok().and_then(|g| g.get(id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn announce_and_lookup() {
        let dht = InMemoryDht::new();
        dht.announce("node-a", "pubkey-a");
        assert_eq!(dht.lookup("node-a"), Some("pubkey-a".to_string()));
        assert_eq!(dht.lookup("node-b"), None);
    }
}
