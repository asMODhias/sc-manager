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
}
