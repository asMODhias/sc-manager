use base64::engine::general_purpose::STANDARD as base64_std;
use base64::Engine as _;
use ed25519_dalek::{SecretKey, PublicKey, Signature};
use signature::{Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// Canonical DomainEvent payload (simplified for demo)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DomainEventPayload {
    pub id: String,
    pub kind: String,
    pub payload: serde_json::Value,
}

/// SignedEvent wrapper stored on the bus
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SignedEvent {
    pub event: DomainEventPayload,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

/// Lightweight compatibility KeyPair used by the workspace (wraps raw bytes)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPair {
    pub id: String,
    pub public_key_b64: String,
    pub secret_bytes: [u8; 32],
}

impl KeyPair {
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Reconstruct secret/public and sign using Keypair
        let sk = SecretKey::from_bytes(&self.secret_bytes).map_err(|e| format!("secret key error: {}", e))?;
        let pk = PublicKey::from(&sk);
        let kp = ed25519_dalek::Keypair { secret: sk, public: pk };
        let sig: Signature = kp.sign(data);
        Ok(sig.to_bytes().to_vec())
    }

    pub fn public_bytes(&self) -> Result<Vec<u8>, String> {
        base64_std.decode(&self.public_key_b64).map_err(|e| format!("invalid base64 pk: {}", e))
    }

    pub fn from_secret_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != 32 { return Err(format!("invalid key length: {}", bytes.len())); }
        let mut arr = [0u8; 32]; arr.copy_from_slice(bytes);
        let sk = SecretKey::from_bytes(&arr).map_err(|e| format!("secret key error: {}", e))?;
        // Compute public key from secret
        let pk = PublicKey::from(&sk);
        let pk_b64 = base64_std.encode(pk.to_bytes());
        Ok(KeyPair {
            id: format!("node-{}", &pk_b64[0..8.min(pk_b64.len())]),
            public_key_b64: pk_b64,
            secret_bytes: arr,
        })
    }

    pub fn generate() -> Result<Self, String> {
        // deterministic for tests
        let arr = [1u8; 32];
        KeyPair::from_secret_bytes(&arr)
    }
}

/// Deterministic test keypair (for CI/testing only)
pub fn generate_test_keypair() -> Result<KeyPair, String> {
    KeyPair::generate()
}

/// Sign a DomainEventPayload
pub fn sign_event(kp: &KeyPair, event: &DomainEventPayload) -> Result<SignedEvent, String> {
    let payload = bincode::serialize(event).map_err(|e| format!("serialize event: {}", e))?;
    let signature = kp.sign(&payload).map_err(|e| format!("sign: {}", e))?;
    let public_key = kp.public_bytes()?;
    Ok(SignedEvent {
        event: event.clone(),
        public_key,
        signature,
    })
}

/// Verify a SignedEvent's signature
pub fn verify_signature(signed: &SignedEvent) -> bool {
    let payload = match bincode::serialize(&signed.event) {
        Ok(p) => p,
        Err(_) => return false,
    };
    let pk_bytes: [u8; 32] = match signed.public_key.as_slice().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };
    let pk = match PublicKey::from_bytes(&pk_bytes) {
        Ok(p) => p,
        Err(_) => return false,
    };
    let sig_bytes: [u8; 64] = match signed.signature.as_slice().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };
    let sig = match Signature::from_bytes(&sig_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };
    pk.verify(&payload, &sig).is_ok()
}


