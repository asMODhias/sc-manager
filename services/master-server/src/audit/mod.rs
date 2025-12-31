use crate::domain::AuditEvent;
use crate::storage::AppendOnlyLedger;
use crate::storage::ledger::LedgerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("ledger error: {0}")]
    Ledger(#[from] LedgerError),

    #[error("chain mismatch: expected previous hash {expected} but got {found}")]
    ChainMismatch { expected: String, found: String },
}

/// Append an event to the ledger.
/// If `event.previous_hash` is empty, it will be set to the last event's id.
/// If `event.previous_hash` is provided, it must match the ledger's last event id.
pub fn append_event(ledger: &AppendOnlyLedger, mut event: AuditEvent) -> Result<(), AuditError> {
    let last = ledger.load_all()?;
    if let Some(prev) = last.last() {
        if event.previous_hash.is_empty() {
            event.previous_hash = prev.event_id.clone();
        } else if event.previous_hash != prev.event_id {
            return Err(AuditError::ChainMismatch { expected: prev.event_id.clone(), found: event.previous_hash });
        }
    } else {
        // no previous events; ensure previous_hash is empty (genesis)
        if !event.previous_hash.is_empty() {
            // allow previous hash if explicitly provided for genesis
        }
    }

    ledger.append(&event)?;
    Ok(())
}

pub fn list_events(ledger: &AppendOnlyLedger) -> Result<Vec<AuditEvent>, AuditError> {
    let ev = ledger.load_all()?;
    Ok(ev)
}

pub fn verify_chain(ledger: &AppendOnlyLedger) -> Result<bool, AuditError> {
    Ok(ledger.verify_chain()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use crate::domain::AuditEventType;

    #[test]
    fn test_append_and_verify_chain() {
        let tf = NamedTempFile::new().expect("tmp");
        let p = tf.path().to_path_buf();
        let ledger = AppendOnlyLedger::new(&p);

        // genesis event
        let e1 = AuditEvent::new(
            AuditEventType::InstanceRegistered,
            "node-x",
            "payload-1",
            "DE",
            "8.0.0",
            "",
        );

        append_event(&ledger, e1).expect("append e1");
        assert!(verify_chain(&ledger).unwrap());

        // next event without previous_hash should be linked automatically
        let e2 = AuditEvent::new(
            AuditEventType::InstanceActive,
            "node-x",
            "payload-2",
            "DE",
            "8.0.0",
            "",
        );

        append_event(&ledger, e2).expect("append e2");
        assert!(verify_chain(&ledger).unwrap());

        let events = list_events(&ledger).expect("list");
        assert_eq!(events.len(), 2);
        assert!(events[1].verify_chain(&events[0]));
    }

    #[test]
    fn test_append_chain_mismatch_error() {
        let tf = NamedTempFile::new().expect("tmp");
        let p = tf.path().to_path_buf();
        let ledger = AppendOnlyLedger::new(&p);

        let e1 = AuditEvent::new(
            AuditEventType::InstanceRegistered,
            "node-x",
            "payload-1",
            "DE",
            "8.0.0",
            "",
        );
        append_event(&ledger, e1).expect("append e1");

        // create event with wrong previous hash
        let e2 = AuditEvent::new(
            AuditEventType::InstanceActive,
            "node-x",
            "payload-2",
            "DE",
            "8.0.0",
            "WRONGHASH",
        );

        let res = append_event(&ledger, e2);
        assert!(matches!(res, Err(AuditError::ChainMismatch{..}))); 
    }
}
