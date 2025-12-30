use crate::crdt::CRDTStateManager;
use crate::gossip::{HashGossipNode, GossipUpdate};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

/// Master client stub for reporting to author master
#[derive(Clone)]
pub struct MasterClient {
    base_url: String,
}

impl MasterClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn report_health(&self, _node_id: &str) -> Result<(), MasterClientError> {
        // Minimal stub for now
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MasterClientError {
    #[error("HTTP error: {0}")]
    Http(String),
}

/// Mini-Master Node
///
/// Coordinates CRDT and HashGossip to report and monitor state.
pub struct MiniMaster {
    node_id: String,
    pub crdt: Arc<CRDTStateManager>,
    /// In-memory gossip node (optional)
    pub gossip: Option<Arc<RwLock<HashGossipNode>>>,
    /// Optional control channel for TCP/libp2p-backed gossip (feature gated)
    #[cfg(feature = "libp2p")]
    pub gossip_control: Option<tokio::sync::mpsc::UnboundedSender<crate::gossip::libp2p::Control>>,
    #[cfg(feature = "libp2p")]
    pub gossip_rx: Option<tokio::sync::mpsc::UnboundedReceiver<GossipUpdate>>,

    pub master_client: Arc<MasterClient>,
    pub is_author: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum MiniMasterError {
    #[error("CRDT error: {0}")]
    CRDT(String),
    #[error("Gossip error: {0}")]
    Gossip(String),
}

impl MiniMaster {
    pub fn new(
        node_id: String,
        crdt: CRDTStateManager,
        gossip: HashGossipNode,
        master_client: MasterClient,
        is_author: bool,
    ) -> Self {
        Self {
            node_id,
            crdt: Arc::new(crdt),
            gossip: Some(Arc::new(RwLock::new(gossip))),
            #[cfg(feature = "libp2p")]
            gossip_control: None,
            #[cfg(feature = "libp2p")]
            gossip_rx: None,
            master_client: Arc::new(master_client),
            is_author,
        }
    }

    #[cfg(feature = "libp2p")]
    pub fn new_with_control(
        node_id: String,
        crdt: CRDTStateManager,
        control_tx: tokio::sync::mpsc::UnboundedSender<crate::gossip::libp2p::Control>,
        update_rx: Option<tokio::sync::mpsc::UnboundedReceiver<GossipUpdate>>,
        master_client: MasterClient,
        is_author: bool,
    ) -> Self {
        let arc_crdt = Arc::new(crdt);

        // If an update receiver is provided, spawn a handler task right away to consume updates
        if let Some(mut rx) = update_rx {
            let crdt_clone = Arc::clone(&arc_crdt);
            tokio::spawn(async move {
                while let Some(update) = rx.recv().await {
                    match update {
                        GossipUpdate::HashReceived { peer_id: _peer, entity_id, hash } => {
                            if let Ok(state) = crdt_clone.export_state().await {
                                let local_hash = crate::gossip::HashGossipNode::hash_state(&state);
                                if local_hash != hash {
                                    eprintln!("mini-master(control): mismatch detected for {} (local {} != peer {})", entity_id, local_hash, hash);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            });
        }

        Self {
            node_id,
            crdt: arc_crdt,
            gossip: None,
            gossip_control: Some(control_tx),
            gossip_rx: None,
            master_client: Arc::new(master_client),
            is_author,
        }
    }

    /// Start background tasks (periodic broadcasting and handlers)
    pub async fn start(&self) -> Result<(), MiniMasterError> {
        self.start_hash_broadcast().await;
        self.start_gossip_handler().await;
        Ok(())
    }

    /// Helper to start periodic broadcasting with default interval (10s)
    async fn start_hash_broadcast(&self) {
        self.start_hash_broadcast_with_interval(Duration::from_secs(10)).await;
    }

    /// Start hash broadcasting with configurable interval (useful in tests)
    pub async fn start_hash_broadcast_with_interval(&self, dur: Duration) {
        let crdt = Arc::clone(&self.crdt);
        let gossip = self.gossip.clone();
        #[cfg(feature = "libp2p")]
        let control = self.gossip_control.clone();

        tokio::spawn(async move {
            let mut ticker = interval(dur);
            loop {
                ticker.tick().await;
                // Export state and broadcast
                match crdt.export_state().await {
                    Ok(state) => {
                        // in-memory broadcast
                        if let Some(g) = gossip.as_ref() {
                            let mut g = g.write().await;
                            if let Err(e) = g.broadcast_hash("global".to_string(), &state) {
                                eprintln!("mini-master: broadcast failed: {}", e);
                            }
                        }

                        // control-based broadcast
                        #[cfg(feature = "libp2p")]
                        if let Some(ctrl) = control.as_ref() {
                            use crate::gossip::HashGossipMessage;
                            let msg = HashGossipMessage {
                                entity_id: "global".into(),
                                state_hash: crate::gossip::HashGossipNode::hash_state(&state),
                                timestamp: 0,
                                peer_id: "mini-master".into(),
                            };
                            if let Ok(data) = serde_json::to_vec(&msg) {
                                let _ = ctrl.send(crate::gossip::libp2p::Control::Publish(data));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("mini-master: export_state failed: {}", e);
                    }
                }
            }
        });
    }

    /// Start a placeholder gossip handler (could subscribe to updates)
    async fn start_gossip_handler(&self) {
        // For TCP/libp2p-backed gossip, handler tasks are spawned during construction when an update receiver is provided.
        // For in-memory gossip, event handling is done via its receiver in tests or other components.
    }

    /// Perform a single broadcast step synchronously (useful for tests)
    pub async fn broadcast_once(&self) -> Result<(), MiniMasterError> {
        let state = self
            .crdt
            .export_state()
            .await
            .map_err(|e| MiniMasterError::CRDT(e.to_string()))?;
        if let Some(gopt) = self.gossip.as_ref() {
            let mut g = gopt.write().await;
            g.broadcast_hash("global".to_string(), &state)
                .map_err(|e| MiniMasterError::Gossip(e))
        } else if cfg!(feature = "libp2p") {
            // If we have control-based gossip, caller should use control channel directly
            Err(MiniMasterError::Gossip("no in-memory gossip backend".into()))
        } else {
            Err(MiniMasterError::Gossip("no gossip backend".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_broadcast_once_emits_hash() {
        let crdt = CRDTStateManager::new("actor-test");
        crdt
            .update_org("org-x", "name", serde_json::Value::String("Acme".into()))
            .await
            .expect("update");

        let (g, mut rx) = HashGossipNode::new_local("peer-mini");
        let mc = MasterClient::new("http://master.local".into());
        let mm = MiniMaster::new("node-1".into(), crdt, g, mc, false);

        mm.broadcast_once().await.expect("broadcast_once");

        if let Some(crate::gossip::GossipUpdate::HashReceived { peer_id, entity_id, hash }) = rx.recv().await {
            assert_eq!(peer_id, "peer-mini");
            assert_eq!(entity_id, "global");
            assert_eq!(hash.len(), 64);
        } else {
            panic!("expected HashReceived");
        }
    }

    #[tokio::test]
    async fn test_periodic_broadcast_runs() {
        let crdt = CRDTStateManager::new("actor-test-2");
        crdt
            .update_org("org-y", "member_count", serde_json::Value::Number(serde_json::Number::from(7)))
            .await
            .expect("update");

        let (g, mut rx) = HashGossipNode::new_local("peer-mm");
        let mc = MasterClient::new("http://master.local".into());
        let mm = MiniMaster::new("node-2".into(), crdt, g, mc, false);

        // start periodic broadcasting with short interval
        mm.start_hash_broadcast_with_interval(Duration::from_millis(20)).await;

        // Wait for a couple of broadcasts
        sleep(Duration::from_millis(60)).await;

        // We should have received at least one HashReceived
        let mut found = false;
        while let Ok(u) = rx.try_recv() {
            if let crate::gossip::GossipUpdate::HashReceived { peer_id, entity_id, .. } = u {
                assert_eq!(peer_id, "peer-mm");
                assert_eq!(entity_id, "global");
                found = true;
                break;
            }
        }

        assert!(found, "expected at least one HashReceived event");
    }

    // E2E test using TCP-backed libp2p stub control channels
    #[cfg(feature = "libp2p")]
    #[tokio::test]
    async fn test_mini_master_e2e_with_control() {
        use crate::gossip::libp2p::new_and_run;
        use crate::gossip::libp2p::Control;
        use libp2p::identity;
        use tokio::time::sleep;

        // node1 (will receive)
        let k1 = identity::Keypair::generate_ed25519();
        let (ctrl1, mut rx1) = new_and_run(k1.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap()))
            .await
            .expect("start node1");

        // wait for listen addr
        let mut node1_addr = None;
        for _ in 0..20 {
            if let Some(msg) = rx1.recv().await {
                if let crate::gossip::GossipUpdate::LocalListenAddr(addr) = msg {
                    node1_addr = Some(addr);
                    break;
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
        let node1_addr = node1_addr.expect("node1 listen addr");
        let node1_multi: libp2p::core::Multiaddr = node1_addr.parse().expect("parse");

        // node2 (will dial and publish)
        let k2 = identity::Keypair::generate_ed25519();
        let (ctrl2, _rx2) = new_and_run(k2.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap()))
            .await
            .expect("start node2");

        // dial node1
        ctrl2.send(Control::Dial(node1_multi.clone())).expect("dial request send");

        // Prepare CRDT states: node2 has a different state
        let crdt1 = CRDTStateManager::new("a");
        crdt1.update_org("org-e2e", "name", serde_json::Value::String("A".into())).await.expect("update1");
        let crdt2 = CRDTStateManager::new("b");
        crdt2.update_org("org-e2e", "name", serde_json::Value::String("B".into())).await.expect("update2");

        // Create MiniMaster wrappers (node1: give ctrl1 but keep rx1 for assertions; node2: give ctrl2)
        let mc = MasterClient::new("http://master.local".into());
        let mm1 = MiniMaster::new_with_control("node-1".into(), crdt1, ctrl1.clone(), None, mc.clone(), false);
        let mm2 = MiniMaster::new_with_control("node-2".into(), crdt2, ctrl2.clone(), None, mc, false);

        // start broadcasting on node2 with short interval
        mm2.start_hash_broadcast_with_interval(Duration::from_millis(20)).await;

        // node1's rx1 should receive HashReceived from node2
        let mut saw = false;
        for _ in 0..40 {
            if let Some(u) = rx1.recv().await {
                if let crate::gossip::GossipUpdate::HashReceived { peer_id: _p, entity_id, hash } = u {
                    if entity_id == "global" {
                        // compute node1 local hash and ensure mismatch (since states differ)
                        let local_state = mm1.crdt.export_state().await.expect("local state");
                        let local_hash = crate::gossip::HashGossipNode::hash_state(&local_state);
                        assert_ne!(local_hash, hash);
                        saw = true;
                        break;
                    }
                }
            }
            sleep(Duration::from_millis(25)).await;
        }
        assert!(saw, "expected node1 to receive a HashReceived from node2");
    }
}