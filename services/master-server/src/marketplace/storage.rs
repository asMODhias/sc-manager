use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use serde_json::Deserializer;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarketplaceStorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarketplaceEvent {
    Create { id: String, owner: String, price: u64, metadata: String },
    Remove { id: String },
}

#[derive(Debug)]
pub struct MarketplaceLedger {
    pub path: PathBuf,
}

impl MarketplaceLedger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        MarketplaceLedger { path: path.into() }
    }

    pub fn append(&self, event: &MarketplaceEvent) -> Result<(), MarketplaceStorageError> {
        let file = OpenOptions::new().create(true).append(true).open(&self.path)?;
        let mut w = BufWriter::new(file);
        let s = serde_json::to_string(event)?;
        w.write_all(s.as_bytes())?;
        w.write_all(b"\n")?;
        w.flush()?;
        Ok(())
    }

    pub fn load_all(&self) -> Result<Vec<MarketplaceEvent>, MarketplaceStorageError> {
        let file = OpenOptions::new().read(true).open(&self.path)?;
        let reader = BufReader::new(file);
        let stream = Deserializer::from_reader(reader).into_iter::<MarketplaceEvent>();
        let mut res = Vec::new();
        for item in stream {
            res.push(item?);
        }
        Ok(res)
    }

    /// Path to the snapshot file for this ledger
    pub fn snapshot_path(&self) -> PathBuf {
        let mut p = self.path.clone();
        p.set_extension("snapshot.json");
        p
    }

    /// Write an atomic snapshot of the provided state (map of id -> Item).
    pub fn write_snapshot_atomic(&self, state: &std::collections::HashMap<String, crate::marketplace::Item>) -> Result<(), MarketplaceStorageError> {
        let snap = self.snapshot_path();
        let tmp = snap.with_extension("snapshot.json.tmp");
        let mut w = BufWriter::new(OpenOptions::new().create(true).write(true).truncate(true).open(&tmp)?);
        let s = serde_json::to_string(state)?;
        w.write_all(s.as_bytes())?;
        w.flush()?;
        std::fs::rename(&tmp, &snap)?;
        Ok(())
    }

    /// Compact the ledger by rotating the existing ledger file and leaving a fresh one.
    pub fn compact(&self) -> Result<(), MarketplaceStorageError> {
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let bak = self.path.with_extension(format!("ledger.bak.{}", ts));
        // If ledger exists, rename to a backup
        if self.path.exists() {
            std::fs::rename(&self.path, &bak)?;
        }
        // create a new empty ledger file
        let _ = OpenOptions::new().create(true).write(true).truncate(true).open(&self.path)?;
        Ok(())
    }
}
