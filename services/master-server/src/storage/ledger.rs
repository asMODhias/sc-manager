use crate::domain::AuditEvent;
use serde_json::Deserializer;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("chain verification failed")]
    ChainVerificationFailed,
}

/// Very small append-only ledger backed by newline-delimited JSON (NDJSON)
pub struct AppendOnlyLedger {
    path: PathBuf,
}

impl AppendOnlyLedger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Append an event - writes a single line JSON to the file
    pub fn append(&self, event: &AuditEvent) -> Result<(), LedgerError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let mut w = BufWriter::new(file);
        let s = serde_json::to_string(event)?;
        w.write_all(s.as_bytes())?;
        w.write_all(b"\n")?;
        w.flush()?;
        Ok(())
    }

    /// Load all events in order
    pub fn load_all(&self) -> Result<Vec<AuditEvent>, LedgerError> {
        let file = OpenOptions::new().read(true).open(&self.path)?;
        let reader = BufReader::new(file);
        let stream = Deserializer::from_reader(reader).into_iter::<AuditEvent>();
        let mut res = Vec::new();
        for item in stream {
            res.push(item?);
        }
        Ok(res)
    }

    /// Verify chain integrity across the ledger
    pub fn verify_chain(&self) -> Result<bool, LedgerError> {
        let events = self.load_all()?;

        if events.is_empty() {
            return Ok(true);
        }

        for pair in events.windows(2) {
            let prev = &pair[0];
            let cur = &pair[1];
            if !cur.verify_chain(prev) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_append_and_verify() {
        let f = NamedTempFile::new().expect("tempfile");
        let p = f.path().to_path_buf();
        let ledger = AppendOnlyLedger::new(&p);

        let e1 = AuditEvent::new(
            crate::domain::AuditEventType::InstanceRegistered,
            "node-a",
            "payload-1",
            "US",
            "8.0.0",
            "",
        );
        ledger.append(&e1).expect("append e1");

        let e2 = AuditEvent::new(
            crate::domain::AuditEventType::InstanceActive,
            "node-a",
            "payload-2",
            "US",
            "8.0.0",
            e1.event_id.clone(),
        );
        ledger.append(&e2).expect("append e2");

        let events = ledger.load_all().expect("load_all");
        assert_eq!(events.len(), 2);
        assert!(ledger.verify_chain().expect("verify"));
    }
}
