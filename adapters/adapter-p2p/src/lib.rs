//! P2P Foundation scaffold: identity and signed event traits

use serde::{Deserialize, Serialize};

/// Lightweight identity abstraction (Ed25519 key reference)
pub trait Identity {
    fn id(&self) -> &str;
    fn public_key(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SignedEvent {
    pub id: String,
    pub payload: String,
    pub signer_id: String,
    pub signature: String,
}

pub trait Signer {
    fn sign(&self, data: &str) -> String;
}

// Reuse the core KeyPair compatibility type
use sc_manager_core::events::KeyPair as CoreKeyPair;
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use base64::Engine;

/// Use the core KeyPair for generation/signing
impl Identity for CoreKeyPair {
    fn id(&self) -> &str {
        &self.id
    }
    fn public_key(&self) -> &str {
        &self.public_key_b64
    }
}

impl Signer for CoreKeyPair {
    fn sign(&self, data: &str) -> String {
        let sig_bytes = self.sign(data.as_bytes());
        base64::engine::general_purpose::STANDARD.encode(sig_bytes)
    }
}

impl SignedEvent {
    pub fn verify<S: Identity>(&self, signer: &S) -> bool {
        // decode signature and public key
        if self.signer_id != signer.id() {
            return false;
        }
        let sig_bytes = match base64::engine::general_purpose::STANDARD.decode(&self.signature) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let pk_bytes = match base64::engine::general_purpose::STANDARD.decode(signer.public_key()) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let pk_arr: [u8; 32] = match pk_bytes.as_slice().try_into() {
            Ok(a) => a,
            Err(_) => return false,
        };
        let vk = match VerifyingKey::from_bytes(&pk_arr) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let sig_arr: [u8; 64] = match sig_bytes.as_slice().try_into() {
            Ok(a) => a,
            Err(_) => return false,
        };
        let sig = Signature::from_bytes(&sig_arr);
        vk.verify(self.payload.as_bytes(), &sig).is_ok()
    }
}

mod dht;
pub mod transport;

pub use dht::{DhtRegistry, InMemoryDht};
pub use transport::{TransportMessage, Transport, InMemoryTransportRegistry, MockQuicTransport};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn keypair_can_sign_and_verify() {
        let kp = KeyPair::generate().expect("generate keypair in test");
        let payload = "payload-123";
        let sig = kp.sign(payload);
        let ev = SignedEvent {
            id: "e1".to_string(),
            payload: payload.to_string(),
            signer_id: kp.id.clone(),
            signature: sig,
        };
        assert!(ev.verify(&kp));
    }

    // other tests omitted for brevity (kept in original repo)
}
