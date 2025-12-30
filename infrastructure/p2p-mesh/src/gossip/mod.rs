use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

/// Message sent over the gossip network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashGossipMessage {
    pub entity_id: String,
    pub state_hash: String,
    pub timestamp: u64,
    pub peer_id: String,
}

/// Notifications emitted by the gossip node
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GossipUpdate {
    HashReceived {
        peer_id: String,
        entity_id: String,
        hash: String,
    },
    MismatchDetected {
        entity_id: String,
        local_hash: String,
        peer_hash: String,
        peer_id: String,
    },
    PeerConnected(String),
    PeerDisconnected(String),
    /// Local listen address announced by libp2p node (stringified `Multiaddr`)
    LocalListenAddr(String),
}

// Production libp2p-backed implementation
pub mod libp2p;

/// Simple in-memory HashGossipNode used for tests and early integration.
/// Production implementation should be backed by libp2p.
pub struct HashGossipNode {
    /// Local peer id (string)
    pub peer_id: String,

    /// Local state hashes
    pub state_hashes: HashMap<String, String>,

    /// Peer state hashes
    pub peer_hashes: HashMap<String, HashMap<String, String>>,

    /// Update channel to send notifications
    update_tx: UnboundedSender<GossipUpdate>,
}

impl HashGossipNode {
    /// Construct a new in-memory node with a paired receiver
    pub fn new_local(peer_id: impl Into<String>) -> (Self, UnboundedReceiver<GossipUpdate>) {
        let (tx, rx) = unbounded_channel();
        let node = Self {
            peer_id: peer_id.into(),
            state_hashes: HashMap::new(),
            peer_hashes: HashMap::new(),
            update_tx: tx,
        };
        (node, rx)
    }

    /// Broadcast local state hash (in this local variant we immediately "receive" it)
    pub fn broadcast_hash(&mut self, entity_id: String, state: &[u8]) -> Result<(), String> {
        let hash = Self::hash_state(state);
        self.state_hashes.insert(entity_id.clone(), hash.clone());

        let msg = HashGossipMessage {
            entity_id: entity_id.clone(),
            state_hash: hash.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_secs(),
            peer_id: self.peer_id.clone(),
        };

        // In a real network we would send serialized bytes; here we handle locally
        self.handle_message(&msg);
        Ok(())
    }

    /// Handle an incoming gossip message (merge into peer hashes and emit updates)
    pub fn handle_message(&mut self, msg: &HashGossipMessage) {
        // store peer hash
        let peer_entry = self.peer_hashes.entry(msg.peer_id.clone()).or_insert_with(HashMap::new);
        peer_entry.insert(msg.entity_id.clone(), msg.state_hash.clone());

        // emit HashReceived
        let _ = self.update_tx.send(GossipUpdate::HashReceived {
            peer_id: msg.peer_id.clone(),
            entity_id: msg.entity_id.clone(),
            hash: msg.state_hash.clone(),
        });

        // Check for mismatch
        if let Some(local_hash) = self.state_hashes.get(&msg.entity_id) {
            if local_hash != &msg.state_hash {
                let _ = self.update_tx.send(GossipUpdate::MismatchDetected {
                    entity_id: msg.entity_id.clone(),
                    local_hash: local_hash.clone(),
                    peer_hash: msg.state_hash.clone(),
                    peer_id: msg.peer_id.clone(),
                });
            }
        }
    }

    /// Compute SHA3-256 hex hash of state
    pub fn hash_state(state: &[u8]) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(state);
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_state() {
        let h = HashGossipNode::hash_state(b"hello");
        assert_eq!(h.len(), 64);
        // deterministic
        assert_eq!(h, HashGossipNode::hash_state(b"hello"));
    }

    #[tokio::test]
    async fn test_broadcast_and_receive() {
        let (mut n1, mut rx1) = HashGossipNode::new_local("peer-a");
        let (mut n2, mut rx2) = HashGossipNode::new_local("peer-b");

        // peer-a broadcasts (and receives own local event)
        n1.broadcast_hash("global".into(), b"state-v1").expect("broadcast");
        // consume peer-a's self event to keep later assertions deterministic
        if let Some(GossipUpdate::HashReceived { peer_id, .. }) = rx1.recv().await {
            assert_eq!(peer_id, "peer-a");
        }

        // simulate network delivery: deliver to peer-b
        let msg = HashGossipMessage {
            entity_id: "global".into(),
            state_hash: HashGossipNode::hash_state(b"state-v1"),
            timestamp: 0,
            peer_id: "peer-a".into(),
        };
        n2.handle_message(&msg);

        // peer-b should get HashReceived
        if let Some(GossipUpdate::HashReceived { peer_id, entity_id, hash }) = rx2.recv().await {
            assert_eq!(peer_id, "peer-a");
            assert_eq!(entity_id, "global");
            assert_eq!(hash, HashGossipNode::hash_state(b"state-v1"));
        } else {
            panic!("expected HashReceived");
        }

        // now peer-b broadcasts a different state (mismatch)
        n2.broadcast_hash("global".into(), b"state-v2").expect("broadcast2");
        let msg2 = HashGossipMessage {
            entity_id: "global".into(),
            state_hash: HashGossipNode::hash_state(b"state-v2"),
            timestamp: 0,
            peer_id: "peer-b".into(),
        };
        n1.handle_message(&msg2);

        // peer-a should receive both HashReceived and MismatchDetected
        // consume two messages
        let mut saw_hash = false;
        let mut saw_mismatch = false;
        for _ in 0..2 {
            if let Some(u) = rx1.recv().await {
                match u {
                    GossipUpdate::HashReceived { peer_id, entity_id, hash } => {
                        assert_eq!(peer_id, "peer-b");
                        assert_eq!(entity_id, "global");
                        saw_hash = true;
                    }
                    GossipUpdate::MismatchDetected { entity_id, local_hash, peer_hash, peer_id } => {
                        assert_eq!(peer_id, "peer-b");
                        assert_eq!(entity_id, "global");
                        assert_ne!(local_hash, peer_hash);
                        saw_mismatch = true;
                    }
                    _ => {}
                }
            }
        }
        assert!(saw_hash && saw_mismatch, "expected both events");
    }
}