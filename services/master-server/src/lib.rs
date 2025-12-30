//! sc_manager_master_server: Minimal scaffold for the Master Server authority

use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum MasterError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Minimal master server representation
pub struct MasterServer {
    pub id: String,
}

impl Default for MasterServer {
    fn default() -> Self {
        Self { id: "master-0".to_string() }
    }
}

impl MasterServer {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    /// Start the master server (async placeholder)
    pub async fn start(&self) -> Result<(), MasterError> {
        info!("starting master server: {}", self.id);
        // placeholder: initialize storage, keys, network
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_and_start() {
        let m = MasterServer::new("test-master");
        assert_eq!(m.id, "test-master");
        m.start().await.expect("start");
    }

    #[test]
    fn default_has_id() {
        let d = MasterServer::default();
        assert!(!d.id.is_empty());
    }
}
