use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use base64::Engine;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("key parse error: {0}")]
    Parse(String),
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

    /// Load root key from a PEM-like file (simple base64 raw for now)
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, KeyError> {
        let data = std::fs::read(path)?;
        let s = String::from_utf8_lossy(&data);
        let bytes = base64::engine::general_purpose::STANDARD.decode(s.trim()).map_err(|e| KeyError::Parse(e.to_string()))?;
        let kp = Keypair::from_bytes(&bytes).map_err(|e| KeyError::Parse(e.to_string()))?;
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

    #[test]
    fn test_generate_sign_verify() {
        let ks = KeyStore::generate_testpair();
        let payload = b"hello";
        let sig = ks.sign(payload);
        assert!(ks.verify(payload, &sig));
    }
}
