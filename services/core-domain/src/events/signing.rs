use base64::engine::general_purpose::STANDARD as base64_std;
use base64::Engine as _;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature};
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
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let sk = SigningKey::from_bytes(&self.secret_bytes);
        let sig: Signature = sk.sign(data);
        sig.to_bytes().to_vec()
    }

    pub fn public_bytes(&self) -> Vec<u8> {
        base64_std.decode(&self.public_key_b64).expect("valid base64 pk")
    }

    pub fn from_secret_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != 32 { return Err(format!("invalid key length: {}", bytes.len())); }
        let mut arr = [0u8; 32]; arr.copy_from_slice(bytes);
        let sk = SigningKey::from_bytes(&arr);
        let vk = sk.verifying_key();
        let pk_b64 = base64_std.encode(vk.to_bytes());
        Ok(KeyPair {
            id: format!("node-{}", &pk_b64[0..8.min(pk_b64.len())]),
            public_key_b64: pk_b64,
            secret_bytes: arr,
        })
    }

    pub fn generate() -> Self {
        // deterministic for tests
        let arr = [1u8; 32];
        KeyPair::from_secret_bytes(&arr).expect("generate test keypair")
    }
}

/// Sign a DomainEventPayload
pub fn sign_event(kp: &KeyPair, event: &DomainEventPayload) -> SignedEvent {
    let payload = bincode::serialize(event).expect("serialize event");
    let signature = kp.sign(&payload);
    SignedEvent {
        event: event.clone(),
        public_key: kp.public_bytes(),
        signature,
    }
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
    let vk = match VerifyingKey::from_bytes(&pk_bytes) {
        Ok(v) => v,
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
    vk.verify(&payload, &sig).is_ok()
}

/// Deterministic test keypair (for CI/testing only)
pub fn generate_test_keypair() -> KeyPair {
    KeyPair::generate()
}
