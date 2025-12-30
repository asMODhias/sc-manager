use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crossbeam_channel::{Sender, Receiver, unbounded};

/// Simple message envelope used by transports
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportMessage {
    pub from: String,
    pub to: String,
    pub payload: Vec<u8>,
}

/// Transport trait - kept intentionally small so adapters can use either real QUIC or the in-memory mock
pub trait Transport: Send + Sync {
    fn send(&self, to: &str, payload: Vec<u8>) -> Result<(), String>;
    fn id(&self) -> &str;
    fn subscribe(&self) -> Box<dyn Iterator<Item = TransportMessage> + Send>;
}

/// A global in-memory registry used only for tests and local simulations. Not for production.
#[derive(Clone)]
pub struct InMemoryTransportRegistry {
    map: Arc<Mutex<HashMap<String, Sender<TransportMessage>>>>,
}

impl InMemoryTransportRegistry {
    pub fn new() -> Self {
        Self { map: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// Register a sender for a peer ID
    pub fn register(&self, id: &str, tx: Sender<TransportMessage>) {
        match self.map.lock() {
            Ok(mut m) => { m.insert(id.to_string(), tx); }
            Err(e) => tracing::error!("Mutex poisoned in InMemoryTransportRegistry::register: {}", e),
        }
    }

    pub fn send_direct(&self, msg: &TransportMessage) -> Result<(), String> {
        let map = self.map.lock().map_err(|e| format!("lock error: {}", e))?;
        if let Some(tx) = map.get(&msg.to) {
            tx.send(msg.clone()).map_err(|e| format!("send error: {}", e))?;
            Ok(())
        } else {
            Err("destination not found".into())
        }
    }
}

impl Default for InMemoryTransportRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock QUIC transport backed by in-process channels
pub struct MockQuicTransport {
    id: String,
    rx: Receiver<TransportMessage>,
    registry: InMemoryTransportRegistry,
}

impl MockQuicTransport {
    pub fn new(id: impl Into<String>, registry: InMemoryTransportRegistry) -> Self {
        let (tx, rx) = unbounded();
        let id_s = id.into();
        registry.register(&id_s, tx);
        Self { id: id_s, rx, registry }
    }
}

impl Transport for MockQuicTransport {
    fn send(&self, to: &str, payload: Vec<u8>) -> Result<(), String> {
        let msg = TransportMessage { from: self.id.clone(), to: to.into(), payload };
        self.registry.send_direct(&msg)
    }

    fn id(&self) -> &str { &self.id }

    fn subscribe(&self) -> Box<dyn Iterator<Item = TransportMessage> + Send> {
        Box::new(MockTransportIter { rx: self.rx.clone() })
    }
}

struct MockTransportIter {
    rx: Receiver<TransportMessage>,
}

impl Iterator for MockTransportIter {
    type Item = TransportMessage;

    fn next(&mut self) -> Option<Self::Item> {
            self.rx.recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_manager_core::events::KeyPair;
    use crate::SignedEvent;
    use base64::Engine;

    #[test]
    fn transport_send_receive_direct() {
        let reg = InMemoryTransportRegistry::new();
        let a = MockQuicTransport::new("node-a", reg.clone());
        let b = MockQuicTransport::new("node-b", reg.clone());

        let payload = b"hello".to_vec();
        a.send("node-b", payload.clone()).expect("send");

        let mut it = b.subscribe();
        let m = it.next().expect("should receive");
        assert_eq!(m.from, "node-a");
        assert_eq!(m.to, "node-b");
        assert_eq!(m.payload, payload);
    }

    #[test]
    fn transport_signed_event_over_mock_quic() {
        let reg = InMemoryTransportRegistry::new();
        let a = MockQuicTransport::new("node-a", reg.clone());
        let b = MockQuicTransport::new("node-b", reg.clone());

        // TODO(SOT) [TRACKED-001]: Replace unwrap-style usage with explicit error propagation or Result handling in production code.
        // See docs/TRACKED_TODOS.md#TRACKED-001
        let kp = KeyPair::generate().expect("generate keypair in test");
        let payload = "op:announce".to_string();
        let sig_bytes = kp.sign(payload.as_bytes()).expect("sign");
        let sig = base64::engine::general_purpose::STANDARD.encode(sig_bytes);
        let ev = SignedEvent { id: "s1".into(), payload: payload.clone(), signer_id: kp.id.clone(), signature: sig };
        // TODO(SOT) [TRACKED-001]: Replace unwrap-style usage with proper error handling to avoid panics in production.
        // See docs/TRACKED_TODOS.md#TRACKED-001
        let bytes = serde_json::to_vec(&ev).expect("serialize event in test");

        a.send("node-b", bytes).expect("send ev");

        let mut it = b.subscribe();
        // TODO(SOT) [TRACKED-001]: avoid using direct unwrap-style iterator results; handle Option properly to avoid panics in production.
        let m = it.next().expect("should receive");
        // TODO(SOT) [TRACKED-001]: replace serde_json::from_slice(...) usage with proper error handling and returning a Result where appropriate
        let received_ev: SignedEvent = serde_json::from_slice(&m.payload).expect("deserialize event in test");
        // verify signature using public key from sender (simulated by having access to kp here)
        assert!(received_ev.verify(&kp));
        assert_eq!(received_ev.payload, payload);
    }
}
