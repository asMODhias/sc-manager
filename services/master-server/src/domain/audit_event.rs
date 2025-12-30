use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};

/// All data in Master Server is append-only
/// NO updates, NO deletes, ONLY inserts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    /// SHA3-512 hash of event content
    pub event_id: String,

    /// UTC timestamp
    pub timestamp: DateTime<Utc>,

    /// Event category
    pub event_type: AuditEventType,

    /// Anonymized source hash (User/Org/Node)
    pub source_hash: String,

    /// SHA3 hash of payload (never cleartext)
    pub payload_hash: String,

    /// ISO-3166-1 country code (NO IP storage)
    pub geo_region: String,

    /// Software version (e.g., "8.0.0-alpha.0.0.1")
    pub software_version: String,

    /// Author signature (Ed25519, base64)
    pub signature: Option<String>,

    /// Previous event hash (Merkle chain)
    pub previous_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    UpdateSigned,
    UpdateDistributed,
    UpdateApplied,
    UpdateFailed,
    UpdateRolledBack,

    PluginRegistered,
    PluginApproved,
    PluginBlocked,
    PluginInstalled,
    PluginUninstalled,
    PluginCrash,

    ToSViolation,
    PermissionViolation,
    SandboxEscape,
    IntegrityFailure,
    P2PMismatch,

    InstanceRegistered,
    InstanceActive,
    InstanceInactive,
    ErrorReported,
    BugReported,

    FeatureUsed,
    ActionPerformed,
    SessionStarted,
    SessionEnded,
}

impl AuditEvent {
    pub fn new(
        event_type: AuditEventType,
        source_hash: impl Into<String>,
        payload: &str,
        geo_region: impl Into<String>,
        software_version: impl Into<String>,
        previous_hash: impl Into<String>,
    ) -> Self {
        let timestamp = Utc::now();
        let payload_hash = Self::hash(payload);

        let source_hash = source_hash.into();
        let geo_region = geo_region.into();
        let software_version = software_version.into();

        let event_content = format!(
            "{}|{}|{}|{}|{}|{}",
            timestamp.to_rfc3339(),
            serde_json::to_string(&event_type).unwrap(),
            source_hash,
            payload_hash,
            geo_region,
            software_version
        );

        let event_id = Self::hash(&event_content);

        Self {
            event_id,
            timestamp,
            event_type,
            source_hash,
            payload_hash,
            geo_region,
            software_version,
            signature: None,
            previous_hash: previous_hash.into(),
        }
    }

    pub fn hash(data: &str) -> String {
        let mut hasher = Sha3_512::new();
        hasher.update(data.as_bytes());
        let out = hasher.finalize();
        // hex formatting
        hex::encode(out)
    }

    /// Verify that this event links to the provided previous event
    pub fn verify_chain(&self, previous_event: &AuditEvent) -> bool {
        self.previous_hash == previous_event.event_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_consistent() {
        let h1 = AuditEvent::hash("payload1");
        let h2 = AuditEvent::hash("payload1");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 128); // 512 bits = 64 bytes = 128 hex chars
    }

    #[test]
    fn test_new_and_chain_verification() {
        let e1 = AuditEvent::new(
            AuditEventType::InstanceRegistered,
            "node-1",
            "payload-a",
            "DE",
            "8.0.0",
            "",
        );

        let e2 = AuditEvent::new(
            AuditEventType::InstanceActive,
            "node-1",
            "payload-b",
            "DE",
            "8.0.0",
            e1.event_id.clone(),
        );

        assert!(e2.verify_chain(&e1));
        assert!(!e1.verify_chain(&e2));

        // serialization roundtrip
        let s = serde_json::to_string(&e2).expect("serialize");
        let e2_rt: AuditEvent = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(e2, e2_rt);

        // ensure event_id is derived
        assert_eq!(e2.event_id, AuditEvent::hash(&format!(
            "{}|{}|{}|{}|{}|{}",
            e2.timestamp.to_rfc3339(),
            serde_json::to_string(&e2.event_type).unwrap(),
            e2.source_hash,
            e2.payload_hash,
            e2.geo_region,
            e2.software_version
        )));

        // make sure the hash length is correct
        assert_eq!(e2.event_id.len(), 128);
    }
}
