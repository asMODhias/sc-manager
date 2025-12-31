//! sc_manager_master_server: Minimal scaffold for the Master Server authority

use thiserror::Error;
use tracing::info;

pub mod keys;
pub mod config;

pub mod domain;
pub mod storage;

pub mod audit;
pub mod marketplace;

pub mod api;
pub mod publish;

use keys::KeyStore;
use crate::storage::AppendOnlyLedger;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Error)]
pub enum MasterError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Minimal master server representation
pub struct MasterServer {
    pub id: String,
    pub ks: KeyStore,
    pub ledger: Arc<AppendOnlyLedger>,
    pub marketplace: Arc<tokio::sync::RwLock<crate::marketplace::Marketplace>>,
    /// Optional admin token required for admin endpoints
    pub admin_token: Option<String>,
}

impl Default for MasterServer {
    fn default() -> Self {
        let ledger = Arc::new(AppendOnlyLedger::new(std::env::temp_dir().join("master_ledger.ndjson")));
        Self { id: "master-0".to_string(), ks: KeyStore::generate_testpair(), ledger, marketplace: Arc::new(tokio::sync::RwLock::new(crate::marketplace::Marketplace::new())), admin_token: None }
    }
}

impl MasterServer {
    pub fn new(id: impl Into<String>, ks: KeyStore, ledger: Arc<AppendOnlyLedger>) -> Self {
        Self { id: id.into(), ks, ledger, marketplace: Arc::new(tokio::sync::RwLock::new(crate::marketplace::Marketplace::new())), admin_token: None }
    }

    /// Helper to build from defaults (useful for tests)
    pub fn new_with_defaults(id: impl Into<String>) -> Self {
        Self { id: id.into(), ks: KeyStore::generate_testpair(), ledger: Arc::new(AppendOnlyLedger::new(std::env::temp_dir().join("master_ledger.ndjson"))), marketplace: Arc::new(tokio::sync::RwLock::new(crate::marketplace::Marketplace::new())), admin_token: None }
    }

    /// Run the server with provided address (non-blocking)
    pub async fn run(self: Arc<Self>, addr: std::net::SocketAddr) -> Result<(), MasterError> {
        info!("starting master server: {}", self.id);
        crate::api::run_server(self.clone(), addr).await?;
        Ok(())
    }

    /// Minimal start used in tests when not running HTTP
    pub fn new_with_ledger(id: impl Into<String>, ledger_path: PathBuf) -> Self {
        let ks = KeyStore::generate_testpair();
        let ledger = Arc::new(AppendOnlyLedger::new(ledger_path));
        Self { id: id.into(), ks, ledger, marketplace: Arc::new(tokio::sync::RwLock::new(crate::marketplace::Marketplace::new())), admin_token: None }
    }
}

#[cfg(test)]
mod server_tests {
    use super::*;

    #[tokio::test]
    async fn new_with_generated_key() {
        let ks = KeyStore::generate_testpair();
        let ledger = std::env::temp_dir().join("ms_test_ledger.ndjson");
        let m = MasterServer::new("srv-1", ks, std::sync::Arc::new(crate::storage::AppendOnlyLedger::new(ledger)));
        assert_eq!(m.id, "srv-1");
    }

    #[tokio::test]
    async fn new_with_defaults_ok() {
        let m = MasterServer::new_with_defaults("srv-default");
        assert_eq!(m.id, "srv-default");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_and_defaults() {
        let m = MasterServer::new_with_defaults("test-master");
        assert_eq!(m.id, "test-master");
    }

    #[test]
    fn default_has_id() {
        let d = MasterServer::default();
        assert!(!d.id.is_empty());
    }
}
