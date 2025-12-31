use crate::domain::{UpdateEntry, AuditEvent, AuditEventType};
use crate::keys::KeyStore;
use crate::storage::AppendOnlyLedger;
use ed25519_dalek::Signature;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("invalid signature")]
    InvalidSignature,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Verify signature on UpdateEntry and publish an AuditEvent
pub fn publish_update(ks: &KeyStore, ledger: &AppendOnlyLedger, entry: &UpdateEntry) -> Result<(), PublishError> {
    // decode signature
    let sig_bytes = B64.decode(entry.signature.as_bytes()).map_err(|_| PublishError::InvalidSignature)?;
    let sig = Signature::from_bytes(&sig_bytes).map_err(|_| PublishError::InvalidSignature)?;

    // verify signature over canonical bytes
    let msg = entry.canonical_bytes();
    if !ks.verify(&msg, &sig) {
        return Err(PublishError::InvalidSignature);
    }

    // create audit event
    let ae = AuditEvent::new(
        AuditEventType::UpdateSigned,
        "author".to_string(),
        &serde_json::to_string(entry)?,
        "ZZ".to_string(),
        "8.0.0".to_string(),
        "".to_string(),
    );

    ledger.append(&ae).map_err(|e| PublishError::Io(std::io::Error::other(format!("append failed: {}", e))))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_publish_verification_success() {
        let ks = crate::keys::KeyStore::generate_testpair();
        let tf = NamedTempFile::new().expect("tmp");
        let ledger = AppendOnlyLedger::new(tf.path());

        let mut u = UpdateEntry::new(
            "cid-001".into(),
            "2.0.0".into(),
            crate::domain::ReleaseChannel::Alpha,
            "1.0.0".into(),
            String::new(),
            "http://changelog".into(),
            2048u64,
            crate::domain::UpdateType::Major,
        );

        // sign canonical bytes
        let sig = ks.sign(&u.canonical_bytes());
        let s = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());
        u.signature = s.clone();

        assert!(publish_update(&ks, &ledger, &u).is_ok());

        let events = ledger.load_all().expect("load");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, crate::domain::AuditEventType::UpdateSigned);
    }

    #[test]
    fn test_publish_invalid_signature() {
        let ks = crate::keys::KeyStore::generate_testpair();
        let tf = NamedTempFile::new().expect("tmp");
        let ledger = AppendOnlyLedger::new(tf.path());

        let u = UpdateEntry::new(
            "cid-002".into(),
            "2.0.1".into(),
            crate::domain::ReleaseChannel::Beta,
            "1.0.0".into(),
            "bogussig".into(),
            "http://changelog".into(),
            512u64,
            crate::domain::UpdateType::Patch,
        );

        assert!(matches!(publish_update(&ks, &ledger, &u), Err(PublishError::InvalidSignature)));
    }
}