use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateEntry {
    /// Content-addressed hash (CID)
    pub content_hash: String,

    /// Semantic version
    pub version: String,

    /// Release channel
    pub channel: ReleaseChannel,

    /// Minimum supported version for upgrade
    pub min_supported_version: String,

    /// Author signature (base64)
    pub signature: String,

    /// Timestamp of signing
    pub signed_at: DateTime<Utc>,

    /// Changelog URL
    pub changelog_url: String,

    /// File size in bytes
    pub size_bytes: u64,

    /// Update type
    pub update_type: UpdateType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReleaseChannel {
    Alpha,
    Beta,
    ReleaseCandidate,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    Hotfix,
}

impl UpdateEntry {
    pub fn new(
        content_hash: String,
        version: String,
        channel: ReleaseChannel,
        min_supported_version: String,
        signature: String,
        changelog_url: String,
        size_bytes: u64,
        update_type: UpdateType,
    ) -> Self {
        Self {
            content_hash,
            version,
            channel,
            min_supported_version,
            signature,
            signed_at: Utc::now(),
            changelog_url,
            size_bytes,
            update_type,
        }
    }

    /// Canonical serialization (without changing `signature` field) used for signing/verifying
    pub fn canonical_bytes(&self) -> Vec<u8> {
        // For now, sign/verify the JSON representation without the `signature` field.
        let mut copy = self.clone();
        copy.signature = String::new();
        serde_json::to_vec(&copy).expect("serialize update entry")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_serialize() {
        let u = UpdateEntry::new(
            "cid-123".into(),
            "1.0.0".into(),
            ReleaseChannel::Alpha,
            "0.9.0".into(),
            "sig".into(),
            "http://changelog".into(),
            1234u64,
            UpdateType::Minor,
        );

        let b = serde_json::to_vec(&u).expect("serialize");
        let u2: UpdateEntry = serde_json::from_slice(&b).expect("deserialize");
        assert_eq!(u, u2);
        assert_eq!(u.signed_at.timestamp(), u2.signed_at.timestamp());
    }

    #[test]
    fn test_canonical_bytes_empty_signature() {
        let mut u = UpdateEntry::new(
            "cid-abc".into(),
            "1.2.3".into(),
            ReleaseChannel::Stable,
            "1.0.0".into(),
            "base64sig".into(),
            "http://chg".into(),
            1024u64,
            UpdateType::Patch,
        );
        let c = u.canonical_bytes();
        // signature should be cleared in canonical representation
        assert!(!c.is_empty());
        let s = String::from_utf8(c).unwrap();
        assert!(!s.contains("base64sig"));
        // original still has signature
        assert_eq!(u.signature, "base64sig");

        // if signature already empty, canonical is same as normal removal
        u.signature = String::new();
        let c2 = u.canonical_bytes();
        assert!(!c2.is_empty());
    }
}
