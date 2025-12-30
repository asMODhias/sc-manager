#![cfg(feature = "libp2p")]

use crate::gossip::{GossipUpdate, HashGossipMessage};
use libp2p::identity::Keypair;
use libp2p::core::Multiaddr;
use serde_json;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use libp2p_gossipsub::{Gossipsub, GossipsubConfigBuilder, GossipsubEvent, IdentTopic, MessageAuthenticity, ValidationMode};
use libp2p_mdns::{Mdns, MdnsEvent};
use libp2p_kad::{Kademlia, store::MemoryStore, KademliaEvent};
use libp2p_quic::tokio::Transport as QuicTransport;
use libp2p::swarm::{Swarm, SwarmEvent, NetworkBehaviour};
use libp2p::PeerId;
use futures::StreamExt;

/// Control messages sent to the libp2p swarm task
#[derive(Debug)]
pub enum Control {
    /// Publish raw message bytes on the gossip topic
    Publish(Vec<u8>),
    /// Dial a multiaddr
    Dial(Multiaddr),
}

#[derive(Debug, Error)]
pub enum GossipError {
    #[error("Init error: {0}")]
    Init(String),
    #[error("Publish error: {0}")]
    Publish(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct Behaviour {
    gossipsub: Gossipsub,
    mdns: Mdns,
    kademlia: Kademlia<MemoryStore>,
}

#[derive(Debug)]
enum OutEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
    Kademlia(KademliaEvent),
}

impl From<GossipsubEvent> for OutEvent { fn from(e: GossipsubEvent) -> Self { OutEvent::Gossipsub(e) } }
impl From<MdnsEvent> for OutEvent { fn from(e: MdnsEvent) -> Self { OutEvent::Mdns(e) } }
impl From<KademliaEvent> for OutEvent { fn from(e: KademliaEvent) -> Self { OutEvent::Kademlia(e) } }

/// libp2p-backed gossip node using Gossipsub + mDNS + Kademlia.
/// Returns a control sender (for Publish/Dial) and an update receiver stream.
pub async fn new_and_run(
    keypair: Keypair,
    listen_addr: Option<Multiaddr>,
) -> Result<(UnboundedSender<Control>, UnboundedReceiver<GossipUpdate>), GossipError> {
    // Channels
    let (update_tx, update_rx) = unbounded_channel();
    let (control_tx, mut control_rx) = unbounded_channel();

    // Peer id
    let peer_id = PeerId::from(keypair.public());

    // Gossipsub
    let gossipsub_config = GossipsubConfigBuilder::default()
        .heartbeat_interval(std::time::Duration::from_secs(1))
        .validation_mode(ValidationMode::Strict)
        .build()
        .map_err(|e| GossipError::Init(format!("{:?}", e)))?;

    let mut gossipsub = Gossipsub::new(
        MessageAuthenticity::Signed(keypair.clone()),
        gossipsub_config,
    ).map_err(|e| GossipError::Init(format!("{:?}", e)))?;
    let topic = IdentTopic::new("scmanager-hash-gossip");
    gossipsub.subscribe(&topic).map_err(|e| GossipError::Init(format!("{:?}", e)))?;

    // mDNS (for local discovery)
    let mdns = Mdns::new(Default::default()).await.map_err(|e| GossipError::Init(format!("{:?}", e)))?;

    // Kademlia
    let store = MemoryStore::new(peer_id);
    let kademlia = Kademlia::new(peer_id, store);

    let behaviour = Behaviour { gossipsub, mdns, kademlia };

    // Transport: use development transport (TCP + Noise + Yamux) for reliability in tests.
    // TODO: move to QUIC transport when environment supports it fully.
    // Build a Swarm using the available SwarmBuilder helpers which configure
    // a tokio runtime and QUIC transport when available. This mirrors the
    // intended production stack (QUIC + tokio).
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_quic()
        .with_behaviour(|_| behaviour)
        .map_err(|e| GossipError::Init(format!("{:?}", e)))?
        .build();
    // If listen addr provided, start listening
    if let Some(ma) = listen_addr {
        if let Err(e) = swarm.listen_on(ma) {
            return Err(GossipError::Init(format!("listen failed: {}", e)));
        }
    }

    // Spawn the swarm event loop
    tokio::spawn(async move {
        let topic = IdentTopic::new("scmanager-hash-gossip");
        loop {
            tokio::select! {
                // Control channel handling
                ctrl = control_rx.recv() => {
                    match ctrl {
                        Some(Control::Dial(ma)) => {
                            let _ = swarm.dial(ma);
                        }
                        Some(Control::Publish(data)) => {
                            let _ = swarm.behaviour_mut().gossipsub.publish(topic.clone(), data);
                        }
                        None => break,
                    }
                }
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            let _ = update_tx.send(GossipUpdate::LocalListenAddr(address.to_string()));
                        }
                        SwarmEvent::Behaviour(OutEvent::Gossipsub(GossipsubEvent::Message { message, .. })) => {
                            if let Ok(msg) = serde_json::from_slice::<HashGossipMessage>(&message.data) {
                                let peer = message.source.unwrap_or_else(|| "".into()).to_string();
                                let _ = update_tx.send(GossipUpdate::HashReceived {
                                    peer_id: peer,
                                    entity_id: msg.entity_id.clone(),
                                    hash: msg.state_hash.clone(),
                                });
                            }
                        }
                        SwarmEvent::Behaviour(OutEvent::Mdns(_)) => {
                            // mdns should have updated kademlia via behaviour automatically
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            let _ = update_tx.send(GossipUpdate::PeerConnected(peer_id.to_string()));
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            let _ = update_tx.send(GossipUpdate::PeerDisconnected(peer_id.to_string()));
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    Ok((control_tx, update_rx))
}

#[cfg(all(test, feature = "libp2p"))]
mod tests {
    use super::*;
    use libp2p::identity;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_libp2p_gossip_publish_and_receive() {
        // Generate keys
        let k1 = identity::Keypair::generate_ed25519();
        let k2 = identity::Keypair::generate_ed25519();

        // Start node1 and listen on ephemeral TCP port
        let (_ctrl1, mut rx1) = new_and_run(k1.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap())).await.expect("node1 start");

        // Wait for LocalListenAddr on node1
        let mut node1_addr = None;
        for _ in 0..20 {
            if let Some(msg) = rx1.recv().await {
                if let GossipUpdate::LocalListenAddr(addr) = msg {
                    node1_addr = Some(addr);
                    break;
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
        let node1_addr = node1_addr.expect("node1 listen addr");
        let node1_multi: Multiaddr = node1_addr.parse().expect("parse multiaddr");

        // Start node2
        let (ctrl2, _rx2) = new_and_run(k2.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap())).await.expect("node2 start");

        // Dial node1 from node2
        ctrl2.send(Control::Dial(node1_multi.clone())).expect("dial request send");

        sleep(Duration::from_secs(1)).await; // allow connection/discovery

        // Node2 publishes a hash
        let msg = HashGossipMessage {
            entity_id: "global".into(),
            state_hash: { let mut h = Sha3_256::new(); h.update(b"state-v1"); hex::encode(h.finalize()) },
            timestamp: 0,
            peer_id: "node2".into(),
        };
        let data = serde_json::to_vec(&msg).expect("serialize");
        ctrl2.send(Control::Publish(data)).expect("publish");

        // Node1 should receive HashReceived via rx1
        let mut saw = false;
        for _ in 0..40 {
            if let Some(u) = rx1.recv().await {
                if let GossipUpdate::HashReceived { peer_id: _peer_id, entity_id, hash } = u {
                    if entity_id == "global" && hash == msg.state_hash {
                        saw = true;
                        break;
                    }
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
        assert!(saw, "node1 should have received the hash");
    }
}
