use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use ed25519_dalek::{Keypair, Signature, Signer};
use ring::pbkdf2;
use std::num::NonZeroU32;
use rand::rngs::OsRng;
use rand::RngCore;
use std::path::Path;
use thiserror::Error;
use std::fs;

#[derive(Debug, Error)]
pub enum KeyError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("key parse error: {0}")]
    Parse(String),
    #[error("serde json error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("decryption failed")]
    Decrypt,
}

/// Minimal in-memory keystore for the master server root key.
pub struct KeyStore {
    pub root: Keypair,
}

impl KeyStore {
    /// Generate an ephemeral test keypair
    pub fn generate_testpair() -> Self {
        let mut csprng = OsRng {};
        let kp = Keypair::generate(&mut csprng);
        Self { root: kp }
    }

    /// Load root key from an unencrypted base64 file (legacy)
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, KeyError> {
        let data = fs::read(path)?;
        let s = String::from_utf8_lossy(&data);
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(s.trim())
            .map_err(|e| KeyError::Parse(e.to_string()))?;
        let kp = Keypair::from_bytes(&bytes).map_err(|e| KeyError::Parse(e.to_string()))?;
        Ok(Self { root: kp })
    }

    /// Save the keypair to an encrypted file using passphrase
    /// Format: JSON with fields: salt, nonce, ciphertext (all base64), iterations, kdf
    pub fn save_encrypted(&self, path: impl AsRef<Path>, passphrase: &str) -> Result<(), KeyError> {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        let mut key = [0u8; 32];
        let iterations: u32 = 100_000;
        let niter = NonZeroU32::new(iterations).unwrap();
        pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, niter, &salt, passphrase.as_bytes(), &mut key);

        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| KeyError::Parse(e.to_string()))?;
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = self.root.to_bytes();
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).map_err(|_| KeyError::Decrypt)?;

        let obj = serde_json::json!({
            "kdf": "pbkdf2",
            "iterations": iterations,
            "salt": base64::engine::general_purpose::STANDARD.encode(salt),
            "nonce": base64::engine::general_purpose::STANDARD.encode(nonce_bytes),
            "ciphertext": base64::engine::general_purpose::STANDARD.encode(ciphertext)
        });
        fs::write(path, serde_json::to_string_pretty(&obj)?)?;
        Ok(())
    }

    /// Load keypair from encrypted file using passphrase
    pub fn load_encrypted(path: impl AsRef<Path>, passphrase: &str) -> Result<Self, KeyError> {
        let data = fs::read_to_string(path)?;
        let v: serde_json::Value = serde_json::from_str(&data)?;
        let salt_b64 = v.get("salt").and_then(|s| s.as_str()).ok_or_else(|| KeyError::Parse("missing salt".into()))?;
        let nonce_b64 = v.get("nonce").and_then(|s| s.as_str()).ok_or_else(|| KeyError::Parse("missing nonce".into()))?;
        let ct_b64 = v.get("ciphertext").and_then(|s| s.as_str()).ok_or_else(|| KeyError::Parse("missing ciphertext".into()))?;
        let iterations = v.get("iterations").and_then(|i| i.as_u64()).unwrap_or(100_000) as u32;

        let salt = base64::engine::general_purpose::STANDARD.decode(salt_b64).map_err(|e| KeyError::Parse(e.to_string()))?;
        let nonce_bytes = base64::engine::general_purpose::STANDARD.decode(nonce_b64).map_err(|e| KeyError::Parse(e.to_string()))?;
        let ciphertext = base64::engine::general_purpose::STANDARD.decode(ct_b64).map_err(|e| KeyError::Parse(e.to_string()))?;

        let mut key = [0u8; 32];
        let niter = NonZeroU32::new(iterations).unwrap();
        pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, niter, &salt, passphrase.as_bytes(), &mut key);

        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| KeyError::Parse(e.to_string()))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let plain = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|_| KeyError::Decrypt)?;

        let kp = Keypair::from_bytes(&plain).map_err(|e| KeyError::Parse(e.to_string()))?;
        Ok(Self { root: kp })
    }

    /// Sign bytes with the root key
    pub fn sign(&self, payload: &[u8]) -> Signature {
        self.root.sign(payload)
    }

    /// Verify a signature for given payload using root public key
    pub fn verify(&self, payload: &[u8], sig: &Signature) -> bool {
        self.root.verify(payload, sig).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_generate_sign_verify() {
        let ks = KeyStore::generate_testpair();
        let payload = b"hello";
        let sig = ks.sign(payload);
        assert!(ks.verify(payload, &sig));
    }

    #[test]
    fn test_save_and_load_encrypted() {
        let ks = KeyStore::generate_testpair();
        let tf = NamedTempFile::new().expect("tmp");
        let path = tf.path().to_path_buf();
        ks.save_encrypted(&path, "s3cr3t").expect("save");
        let ks2 = KeyStore::load_encrypted(&path, "s3cr3t").expect("load");
        let msg = b"ping";
        let s1 = ks.sign(msg);
        let s2 = ks2.sign(msg);
        assert!(ks.verify(msg, &s1));
        assert!(ks2.verify(msg, &s2));
    }
}
