//! sc_manager_master_server: Minimal scaffold for the Master Server authority

use thiserror::Error;
use tracing::info;

pub mod keys;
pub mod config;

use keys::KeyStore;

#[derive(Debug, Error)]
pub enum MasterError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Minimal master server representation
pub struct MasterServer {
    pub id: String,
    pub ks: KeyStore,
}

impl Default for MasterServer {
    fn default() -> Self {
        Self { id: "master-0".to_string(), ks: KeyStore::generate_testpair() }
    }
}

impl MasterServer {
    pub fn new(id: impl Into<String>, ks: KeyStore) -> Self {
        Self { id: id.into(), ks }
    }

    /// Helper to build from defaults (useful for tests)
    pub fn new_with_defaults(id: impl Into<String>) -> Self {
        Self { id: id.into(), ks: KeyStore::generate_testpair() }
    }

    /// Start the master server (async placeholder)
    pub async fn start(&self) -> Result<(), MasterError> {
        info!("starting master server: {}", self.id);
        // placeholder: initialize storage, keys, network
        Ok(())
    }
}

#[cfg(test)]
mod server_tests {
    use super::*;

    #[tokio::test]
    async fn start_with_generated_key() {
        let ks = KeyStore::generate_testpair();
        let m = MasterServer::new("srv-1", ks);
        m.start().await.expect("start");
    }

    #[tokio::test]
    async fn start_with_defaults() {
        let m = MasterServer::new_with_defaults("srv-default");
        m.start().await.expect("start");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_and_start() {
        let m = MasterServer::new_with_defaults("test-master");
        assert_eq!(m.id, "test-master");
        m.start().await.expect("start");
    }

    #[test]
    fn default_has_id() {
        let d = MasterServer::default();
        assert!(!d.id.is_empty());
    }
}
