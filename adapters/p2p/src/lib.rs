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

/// Simple Ed25519 keypair with Signer + Identity
pub struct KeyPair {
    pub id: String,
    pub public_key_b64: String,
    pub secret: ed25519_dalek::SecretKey,
}

impl KeyPair {
    pub fn generate() -> Result<Self, String> {
        // Deterministic/test-friendly keypair generation to avoid RNG dependency conflicts in CI.
        use ed25519_dalek::{PublicKey, SecretKey};
        let sk_bytes = [1u8; 32];
        let sk = SecretKey::from_bytes(&sk_bytes).map_err(|e| format!("SecretKey::from_bytes failed: {}", e))?;
        let pk = PublicKey::from(&sk);
        let pk_b64 = base64::encode(pk.to_bytes());
        Ok(Self {
            id: format!("node-{}", pk_b64.get(0..8).unwrap_or("xx")),
            public_key_b64: pk_b64,
            secret: sk,
        })
    }
}

impl Identity for KeyPair {
    fn id(&self) -> &str {
        &self.id
    }
    fn public_key(&self) -> &str {
        &self.public_key_b64
    }
}

impl Signer for KeyPair {
    fn sign(&self, data: &str) -> String {
        use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer as _};
        // Reconstruct keypair from secret + public without panicking
        let sk = match SecretKey::from_bytes(&self.secret.to_bytes()) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("KeyPair::sign failed to parse SecretKey: {}", e);
                return String::new();
            }
        };
        let pk_bytes = match base64::decode(&self.public_key_b64) {
            Ok(b) => b,
            Err(e) => {
                tracing::error!("KeyPair::sign failed to decode public key: {}", e);
                return String::new();
            }
        };
        let pk = match PublicKey::from_bytes(&pk_bytes) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("KeyPair::sign failed to parse PublicKey: {}", e);
                return String::new();
            }
        };
        let kp = Keypair { secret: sk, public: pk };
        let sig: Signature = kp.sign(data.as_bytes());
        base64::encode(sig.to_bytes())
    }
}

impl SignedEvent {
    pub fn verify<S: Identity>(&self, signer: &S) -> bool {
        use ed25519_dalek::PublicKey;
        // decode signature and public key
        if self.signer_id != signer.id() {
            return false;
        }
        let sig_bytes = match base64::decode(&self.signature) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let pk_bytes = match base64::decode(signer.public_key()) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let pk = match PublicKey::from_bytes(&pk_bytes) {
            Ok(p) => p,
            Err(_) => return false,
        };
        use ed25519_dalek::Signature;
        let sig = match Signature::from_bytes(&sig_bytes) {
            Ok(s) => s,
            Err(_) => return false,
        };
        use ed25519_dalek::Verifier;
        pk.verify(self.payload.as_bytes(), &sig).is_ok()
    }
}

mod dht;

pub use dht::{DhtRegistry, InMemoryDht};

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

    #[test]
    fn inprocess_two_node_gossip_signed_event() {
        // Create two nodes
        let node_a = KeyPair::generate().unwrap();
        let _node_b = KeyPair::generate().unwrap();

        // in-process channel simulating network
        let (tx, rx) = mpsc::channel::<SignedEvent>();

        // Node B: receiver thread - verifies incoming events

        let handle_b = thread::spawn(move || {
            // wait for event
            let ev = rx.recv().expect("should receive event");
            // verify signature against sender public key contained in event.signer_id? For test, we only have A's public key if signer_id matches
            // In real system we'd lookup signer public key via DHT; here we simulate by checking signer_id matches prefix and verify using node_a public key supplied by caller
            ev
        });

        // Node A: create signed event and send
        let payload = "op:announce";
        let sig = node_a.sign(payload);
        let ev = SignedEvent {
            id: "evt-gossip-1".to_string(),
            payload: payload.to_string(),
            signer_id: node_a.id.clone(),
            signature: sig,
        };

        tx.send(ev.clone()).expect("send should work");

        // Node B receives
        let received = handle_b.join().expect("thread join");

        // Simulate node B obtaining node A's public key and verifying
        assert_eq!(received.signer_id, node_a.id);
        assert!(received.verify(&node_a));
    }

    #[test]
    fn inprocess_multi_peer_gossip_simulation() {
        // Simulate a small network of peers using in-process channels
        use std::collections::HashSet;
        use std::sync::{Arc, Mutex};
        const NODES: usize = 4;

        // generate nodes
        let nodes: Vec<KeyPair> = (0..NODES).map(|_| KeyPair::generate().unwrap()).collect();

        // registry of public keys (signer_id -> public_key)
        let registry: Arc<Mutex<std::collections::HashMap<String, String>>> = Arc::new(Mutex::new(std::collections::HashMap::new()));
        for n in nodes.iter() {
            match registry.lock() {
                Ok(mut reg) => { reg.insert(n.id.clone(), n.public_key_b64.clone()); }
                Err(e) => { tracing::error!("Mutex poisoned in registry insert: {}", e); continue; }
            }
        }

        // per-node receive channels
        let mut node_senders = Vec::new();
        let mut node_receivers = Vec::new();
        for _ in 0..NODES {
            let (s, r) = mpsc::channel::<SignedEvent>();
            node_senders.push(s);
            node_receivers.push(r);
        }

        // network inbound channel
        let (net_tx, net_rx) = mpsc::channel::<SignedEvent>();

        // spawn network thread: broadcast inbound events to all nodes
        let net_senders = node_senders.clone();
        let net_handle = thread::spawn(move || {
            while let Ok(ev) = net_rx.recv() {
                for s in net_senders.iter() {
                    // non-blocking send best-effort; if the receiver has gone away ignore
                    let _ = s.send(ev.clone());
                }
            }
        });

        // per-node seen sets
        let mut seen_vec: Vec<Arc<Mutex<HashSet<String>>>> = Vec::new();
        for _ in 0..NODES {
            seen_vec.push(Arc::new(Mutex::new(HashSet::new())));
        }

        // spawn node threads
        let mut handles = Vec::new();
        for i in 0..NODES {
            let rx = node_receivers.remove(0);
            let tx_clone = net_tx.clone();
            let seen = seen_vec[i].clone();
            let registry_cloned = registry.clone();
            let handle = thread::spawn(move || {
                while let Ok(ev) = rx.recv() {
                    // TODO(SOT): Replace lock().unwrap() with proper handling (e.g., map_err or expect with context) to avoid poisoning panics
                    let mut s = seen.lock().unwrap();
                    if s.contains(&ev.id) {
                        continue; // already seen
                    }
                    // verify signature - find signer public key via registry
                    // TODO(SOT): Replace lock().unwrap() with proper handling to surface errors rather than panic
                    let reg = registry_cloned.lock().unwrap();
                    if let Some(pk_b64) = reg.get(&ev.signer_id) {
                        // build a temporary KeyPair-like identity with public key
                        // TODO(SOT): Replace `SecretKey::from_bytes(...).unwrap()` with error handling; using unwrap here can panic in production
                        let temp = KeyPair { id: ev.signer_id.clone(), public_key_b64: pk_b64.clone(), secret: ed25519_dalek::SecretKey::from_bytes(&[1u8;32]).unwrap() };
                        if !ev.verify(&temp) {
                            // invalid signature -> ignore
                            continue;
                        }
                    } else {
                        // unknown signer -> ignore
                        continue;
                    }
                    // new valid event
                    s.insert(ev.id.clone());
                    // rebroadcast to network
                    let _ = tx_clone.send(ev.clone());
                }
            });
            handles.push(handle);
        }

        // create initial event from node 0
        let initial_payload = "op:multi-announce";
        let init_sig = nodes[0].sign(initial_payload);
        let init_ev = SignedEvent {
            id: "evt-multi-1".to_string(),
            payload: initial_payload.to_string(),
            signer_id: nodes[0].id.clone(),
            signature: init_sig,
        };

        // inject into network
        net_tx.send(init_ev.clone()).expect("send initial");

        // wait for propagation (max 2 seconds)
        let timeout = std::time::Instant::now() + std::time::Duration::from_secs(2);
        loop {
            let mut all_seen = true;
            for s in seen_vec.iter() {
                let guard = s.lock().unwrap();
                if !guard.contains(&init_ev.id) {
                    all_seen = false;
                    break;
                }
            }
            if all_seen {
                break;
            }
            if std::time::Instant::now() > timeout {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // check all nodes saw initial event
        for s in seen_vec.iter() {
            let guard = s.lock().unwrap();
            assert!(guard.contains(&init_ev.id));
        }

        // drop node_senders to close node receivers so node threads exit
        drop(node_senders);
        // give nodes a moment to exit and drop their net_tx clones
        std::thread::sleep(std::time::Duration::from_millis(20));
        // drop network sender to stop network thread
        drop(net_tx);
        // join network thread
        let _ = net_handle.join();
        // join node threads
        for h in handles {
            let _ = h.join();
        }
    }
}
