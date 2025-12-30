use thiserror::Error;
use sha3::Digest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::signature;

#[derive(Debug, Error)]
pub enum AuthorError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// File-backed key record for persisting keypairs
#[derive(Debug, Serialize, Deserialize)]
struct KeyRecord {
    pub public_hex: String,
    pub secret_hex: String,
    pub created_unix: u64,
}

/// Simple keystore that stores keypairs as JSON files under a directory
pub struct KeyStore {
    dir: PathBuf,
}

impl KeyStore {
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self, AuthorError> {
        let dir = dir.as_ref().to_path_buf();
        fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    fn path_for(&self, name: &str) -> PathBuf {
        self.dir.join(format!("{}.key.json", name))
    }

    /// Create and persist a new keypair under `name`; returns public key bytes
    pub fn create_keypair(&self, name: &str) -> Result<Vec<u8>, AuthorError> {
        let (pubk, seck) = signature::generate_keypair();
        let record = KeyRecord {
            public_hex: hex::encode(&pubk),
            secret_hex: hex::encode(&seck),
            created_unix: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        let path = self.path_for(name);
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, serde_json::to_string(&record)?)?;
        fs::rename(&tmp, &path)?;
        Ok(pubk)
    }

    pub fn load_keypair(&self, name: &str) -> Result<(Vec<u8>, Vec<u8>), AuthorError> {
        let path = self.path_for(name);
        let s = fs::read_to_string(&path)?;
        let record: KeyRecord = serde_json::from_str(&s)?;
        let pubk = hex::decode(record.public_hex).map_err(|e| AuthorError::InvalidInput(format!("hex decode pub: {}", e)))?;
        let seck = hex::decode(record.secret_hex).map_err(|e| AuthorError::InvalidInput(format!("hex decode sec: {}", e)))?;
        Ok((pubk, seck))
    }

    /// Rotate the keypair for `name`: move existing to `.bak.<ts>` and create a new key
    pub fn rotate_keypair(&self, name: &str) -> Result<Vec<u8>, AuthorError> {
        let path = self.path_for(name);
        if path.exists() {
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let bak = self.dir.join(format!("{}.key.json.bak.{}", name, ts));
            fs::rename(&path, &bak)?;
        }
        self.create_keypair(name)
    }
}

/// Minimal authoring service.
pub struct AuthorService {
    keystore: KeyStore,
}

impl AuthorService {
    pub fn new_with_keystore<P: AsRef<Path>>(dir: P) -> Result<Self, AuthorError> {
        let ks = KeyStore::new(dir)?;
        Ok(Self { keystore: ks })
    }

    pub fn new() -> Self {
        // default: use current working directory `.update_keys`
        let ks = KeyStore::new(".update_keys").expect("create keystore dir");
        Self { keystore: ks }
    }

    /// Legacy SHA3-based signature (kept for tests/backcompat)
    pub fn sign_manifest_sha3(&self, manifest: &str, key: &str) -> Result<String, AuthorError> {
        if manifest.is_empty() {
            return Err(AuthorError::InvalidInput("manifest empty".into()));
        }
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(manifest.as_bytes());
        hasher.update(key.as_bytes());
        let digest = hasher.finalize();
        Ok(hex::encode(digest))
    }

    /// Sign manifest using a stored keypair by name; returns hex-encoded signature
    pub fn sign_manifest_with_key(&self, name: &str, manifest: &str) -> Result<String, AuthorError> {
        if manifest.is_empty() {
            return Err(AuthorError::InvalidInput("manifest empty".into()));
        }
        let (pubk, seck) = self.keystore.load_keypair(name)?;
        match signature::sign_with_keypair(manifest, &seck, &pubk) {
            Ok(sig) => Ok(hex::encode(sig)),
            Err(e) => Err(AuthorError::SignatureError(format!("{}", e)))
        }
    }

    /// Create a new keypair in the keystore for `name`, returning public key bytes
    pub fn create_keypair(&self, name: &str) -> Result<Vec<u8>, AuthorError> {
        self.keystore.create_keypair(name).map_err(|e| e)
    }

    /// Rotate keypair for `name`
    pub fn rotate_keypair(&self, name: &str) -> Result<Vec<u8>, AuthorError> {
        self.keystore.rotate_keypair(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn sign_and_compare_sha3() {
        let svc = AuthorService::new_with_keystore(tempdir().unwrap()).unwrap();
        let manifest = r#"{\"version\":\"1.0.0\"}"#;
        let sig = svc.sign_manifest_sha3(manifest, "secret").unwrap();
        assert!(!sig.is_empty());

        // Deterministic: same inputs produce same signature
        let sig2 = svc.sign_manifest_sha3(manifest, "secret").unwrap();
        assert_eq!(sig, sig2);
    }

    #[test]
    fn sign_ed25519_roundtrip() {
        let dir = tempdir().unwrap();
        let svc = AuthorService::new_with_keystore(dir.path()).unwrap();
        let manifest = "{\"version\":\"1.0.0\"}";
        let pubk = svc.create_keypair("test").unwrap();
        let sig_hex = svc.sign_manifest_with_key("test", manifest).unwrap();
        assert!(!sig_hex.is_empty());

        // verify using the signature module directly
        let sig_bytes = hex::decode(sig_hex).unwrap();
        assert!(crate::signature::verify_signature_ed25519(manifest, &sig_bytes, &pubk).unwrap());
    }

    #[test]
    fn rotate_changes_key() {
        let dir = tempdir().unwrap();
        let svc = AuthorService::new_with_keystore(dir.path()).unwrap();
        let pub1 = svc.create_keypair("rtest").unwrap();
        let pub2 = svc.rotate_keypair("rtest").unwrap();
        assert_ne!(hex::encode(pub1), hex::encode(pub2));

        // ensure backup file exists
        let files: Vec<_> = std::fs::read_dir(dir.path()).unwrap().map(|r| r.unwrap().file_name().to_string_lossy().into_owned()).collect();
        assert!(files.iter().any(|s| s.contains("rtest.key.json.bak")));
    }
}
