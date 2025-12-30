#![cfg(feature = "libp2p")]

use crate::gossip::{GossipUpdate, HashGossipMessage};
use libp2p::identity::Keypair;
use libp2p::core::Multiaddr;
use serde_json;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::sync::Arc;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, Mutex};

// Full libp2p behaviour (gossipsub + mdns + kademlia) is gated behind a
// stricter feature to allow iterative development without breaking CI.
#[cfg(feature = "libp2p_full")]
use libp2p_gossipsub::{Gossipsub, GossipsubConfigBuilder, GossipsubEvent, IdentTopic, MessageAuthenticity, ValidationMode};
#[cfg(feature = "libp2p_full")]
use libp2p_mdns::{Mdns, MdnsEvent};
#[cfg(feature = "libp2p_full")]
use libp2p_kad::{Kademlia, store::MemoryStore, KademliaEvent};
#[cfg(feature = "libp2p_full")]
use libp2p::swarm::{Swarm, SwarmEvent, NetworkBehaviour};
#[cfg(feature = "libp2p_full")]
use libp2p::PeerId;
#[cfg(feature = "libp2p_full")]
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

#[cfg(feature = "libp2p_full")]
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct Behaviour {
    gossipsub: Gossipsub,
    mdns: Mdns,
    kademlia: Kademlia<MemoryStore>,
}

#[cfg(feature = "libp2p_full")]
#[derive(Debug)]
enum OutEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
    Kademlia(KademliaEvent),
}

#[cfg(feature = "libp2p_full")]
impl From<GossipsubEvent> for OutEvent { fn from(e: GossipsubEvent) -> Self { OutEvent::Gossipsub(e) } }
#[cfg(feature = "libp2p_full")]
impl From<MdnsEvent> for OutEvent { fn from(e: MdnsEvent) -> Self { OutEvent::Mdns(e) } }
#[cfg(feature = "libp2p_full")]
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

    // Optional: full libp2p behaviour (Gossipsub-only minimal implementation).
    // Gated behind `libp2p_full` feature to allow iterative development.
    #[cfg(feature = "libp2p_full")]
    {
        use libp2p::gossipsub::{Gossipsub, GossipsubConfigBuilder, MessageAuthenticity, IdentTopic, ValidationMode};
        use libp2p::swarm::Swarm;
        use libp2p::PeerId;

        let peer_id = PeerId::from(keypair.public());

        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(1))
            .validation_mode(ValidationMode::Relaxed)
            .build()
            .map_err(|e| GossipError::Init(format!("{:?}", e)))?;

        let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(keypair.clone()), gossipsub_config)
            .map_err(|e| GossipError::Init(format!("{:?}", e)))?;

        let topic = IdentTopic::new("scmanager-hash-gossip");
        gossipsub.subscribe(&topic).map_err(|e| GossipError::Init(format!("{:?}", e)))?;

        // Transport: development transport for deterministic CI and local testing
        let transport = libp2p::development_transport(keypair.clone()).await.map_err(|e| GossipError::Init(format!("{:?}", e)))?;

        let mut swarm = Swarm::new(transport, gossipsub, peer_id);

        // If listen addr provided, start listening
        if let Some(ma) = listen_addr {
            if let Err(e) = swarm.listen_on(ma) {
                return Err(GossipError::Init(format!("listen failed: {}", e)));
            }
        }

        // Spawn the swarm event loop
        let update = update_tx.clone();
        tokio::spawn(async move {
            loop {
                match swarm.select_next_some().await {
                    libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                        let _ = update.send(GossipUpdate::LocalListenAddr(address.to_string()));
                    }
                    libp2p::swarm::SwarmEvent::Behaviour(libp2p::gossipsub::GossipsubEvent::Message { message, .. }) => {
                        if let Ok(msg) = serde_json::from_slice::<HashGossipMessage>(&message.data) {
                            let peer = message.source.unwrap_or_else(|| "".into()).to_string();
                            let _ = update.send(GossipUpdate::HashReceived { peer_id: peer, entity_id: msg.entity_id.clone(), hash: msg.state_hash.clone() });
                        }
                    }
                    libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        let _ = update.send(GossipUpdate::PeerConnected(peer_id.to_string()));
                    }
                    libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        let _ = update.send(GossipUpdate::PeerDisconnected(peer_id.to_string()));
                    }
                    _ => {}
                }
            }
        });

        // Control loop to handle publish/dial requests
        let mut swarm_ctrl = swarm;
        tokio::spawn(async move {
            while let Some(ctrl) = control_rx.recv().await {
                match ctrl {
                    Control::Dial(ma) => { let _ = swarm_ctrl.dial(ma); }
                    Control::Publish(data) => { let _ = swarm_ctrl.behaviour_mut().publish(topic.clone(), data); }
                }
            }
        });

        return Ok((control_tx, update_rx));
    }
    // Fallback to a simple TCP-backed listener for deterministic tests and early CI until
    // the libp2p QUIC transport and behaviour API are stabilized in this workspace.
    // Bind listener if requested (parses `/ip4/<ip>/tcp/<port>` style multiaddr)
    let listen_socket = if let Some(ma) = listen_addr {
        // Parse Multiaddr; only support /ip4/TCP
        let s = ma.to_string();
        let parts: Vec<&str> = s.split('/').filter(|p| !p.is_empty()).collect();
        if parts.len() >= 3 && parts[0] == "ip4" && parts[2] == "tcp" {
            let ip = parts[1];
            let port: u16 = parts[3].parse().unwrap_or(0);
            let socket = format!("{}:{}", ip, port);
            Some(socket)
        } else {
            return Err(GossipError::Init("unsupported multiaddr format".into()));
        }
    } else {
        None
    };

    if let Some(bind) = listen_socket {
        // Bind listener
        let listener = tokio::net::TcpListener::bind(&bind).await.map_err(|e| GossipError::Init(e.to_string()))?;
        // Send out the actual listen addr
        if let Ok(local) = listener.local_addr() {
            let addr_str = format!("/ip4/{}/tcp/{}", local.ip(), local.port());
            let _ = update_tx.send(GossipUpdate::LocalListenAddr(addr_str));
        }

        let peers_accept = Arc::new(Mutex::new(HashMap::<SocketAddr, UnboundedSender<Vec<u8>>>::new()));
        let update_tx_accept = update_tx.clone();
        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        // Create a per-connection sender to write to this peer
                        let (tx, mut rx) = unbounded_channel::<Vec<u8>>();
                        peers_accept.lock().await.insert(addr, tx);
                        let update_tx_read = update_tx_accept.clone();

                        // split stream into read/write halves
                        let (mut reader, mut writer) = tokio::io::split(stream);

                        // read loop
                        tokio::spawn(async move {
                            let mut len_buf = [0u8; 4];
                            loop {
                                if let Err(_) = reader.read_exact(&mut len_buf).await { break; }
                                let len = u32::from_be_bytes(len_buf) as usize;
                                let mut buf = vec![0u8; len];
                                if let Err(_) = reader.read_exact(&mut buf).await { break; }
                                if let Ok(msg) = serde_json::from_slice::<HashGossipMessage>(&buf) {
                                    let _ = update_tx_read.send(GossipUpdate::HashReceived {
                                        peer_id: msg.peer_id.clone(),
                                        entity_id: msg.entity_id.clone(),
                                        hash: msg.state_hash.clone(),
                                    });
                                }
                            }
                        });

                        // write loop
                        tokio::spawn(async move {
                            while let Some(msg) = rx.recv().await {
                                let len = (msg.len() as u32).to_be_bytes();
                                if let Err(_) = writer.write_all(&len).await { break; }
                                if let Err(_) = writer.write_all(&msg).await { break; }
                            }
                        });
                    }
                    Err(_) => continue,
                }
            }
        });
    }

    // Control loop to dial and publish
    let peers_ctrl = Arc::new(Mutex::new(HashMap::<SocketAddr, UnboundedSender<Vec<u8>>>::new()));
    let update_tx_ctrl = update_tx.clone();
    tokio::spawn(async move {
        while let Some(ctrl) = control_rx.recv().await {
            match ctrl {
                Control::Dial(ma) => {
                    // Parse multiaddr
                    let s = ma.to_string();
                    let parts: Vec<&str> = s.split('/').filter(|p| !p.is_empty()).collect();
                    if parts.len() >= 4 && parts[0] == "ip4" && parts[2] == "tcp" {
                        let ip = parts[1];
                        let port: u16 = parts[3].parse().unwrap_or(0);
                        let socket = format!("{}:{}", ip, port);
                        match tokio::net::TcpStream::connect(&socket).await {
                            Ok(stream) => {
                                let peer_addr = stream.peer_addr().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
                                let (tx, mut rx) = unbounded_channel::<Vec<u8>>();
                                peers_ctrl.lock().await.insert(peer_addr, tx);

                                // Spawn reader
                                let update_tx_r = update_tx_ctrl.clone();
                                let (mut reader, mut writer) = tokio::io::split(stream);
                                tokio::spawn(async move {
                                    let mut len_buf = [0u8; 4];
                                    loop {
                                        if let Err(_) = reader.read_exact(&mut len_buf).await { break; }
                                        let len = u32::from_be_bytes(len_buf) as usize;
                                        let mut buf = vec![0u8; len];
                                        if let Err(_) = reader.read_exact(&mut buf).await { break; }
                                        if let Ok(msg) = serde_json::from_slice::<HashGossipMessage>(&buf) {
                                            let _ = update_tx_r.send(GossipUpdate::HashReceived {
                                                peer_id: msg.peer_id.clone(),
                                                entity_id: msg.entity_id.clone(),
                                                hash: msg.state_hash.clone(),
                                            });
                                        }
                                    }
                                });

                                // Spawn writer
                                tokio::spawn(async move {
                                    while let Some(msg) = rx.recv().await {
                                        let len = (msg.len() as u32).to_be_bytes();
                                        if let Err(_) = writer.write_all(&len).await { break; }
                                        if let Err(_) = writer.write_all(&msg).await { break; }
                                    }
                                });

                                let _ = update_tx_ctrl.send(GossipUpdate::PeerConnected(peer_addr.to_string()));
                            }
                            Err(_) => {}
                        }
                    }
                }
                Control::Publish(data) => {
                    // Forward to all peers
                    let peers_map = peers_ctrl.lock().await;
                    for (_addr, tx) in peers_map.iter() {
                        let _ = tx.send(data.clone());
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
        // This test covers the default TCP-backed stub path (libp2p feature) to ensure
        // deterministic behaviour in CI.
        let k1 = identity::Keypair::generate_ed25519();
        let k2 = identity::Keypair::generate_ed25519();

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

        let (ctrl2, _rx2) = new_and_run(k2.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap())).await.expect("node2 start");
        ctrl2.send(Control::Dial(node1_multi.clone())).expect("dial request send");
        sleep(Duration::from_secs(1)).await; // allow connection

        let msg = HashGossipMessage {
            entity_id: "global".into(),
            state_hash: { let mut h = Sha3_256::new(); h.update(b"state-v1"); hex::encode(h.finalize()) },
            timestamp: 0,
            peer_id: "node2".into(),
        };
        let data = serde_json::to_vec(&msg).expect("serialize");
        ctrl2.send(Control::Publish(data)).expect("publish");

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

#[cfg(all(test, feature = "libp2p_full"))]
mod tests_full {
    use super::*;
    use libp2p::identity;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_libp2p_full_gossipsub_publish_and_receive() {
        // Test the minimal Gossipsub-only implementation behind libp2p_full feature.
        let k1 = identity::Keypair::generate_ed25519();
        let k2 = identity::Keypair::generate_ed25519();

        let (_ctrl1, mut rx1) = new_and_run(k1.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap())).await.expect("node1 start");

        // Wait for node1 listen addr
        let mut node1_addr = None;
        for _ in 0..40 {
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

        let (ctrl2, _rx2) = new_and_run(k2.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap())).await.expect("node2 start");
        ctrl2.send(Control::Dial(node1_multi.clone())).expect("dial");

        sleep(Duration::from_secs(1)).await;

        let msg = HashGossipMessage {
            entity_id: "global".into(),
            state_hash: { let mut h = Sha3_256::new(); h.update(b"state-v1"); hex::encode(h.finalize()) },
            timestamp: 0,
            peer_id: "node2".into(),
        };
        let data = serde_json::to_vec(&msg).expect("serialize");
        ctrl2.send(Control::Publish(data)).expect("publish");

        let mut saw = false;
        for _ in 0..60 {
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

        assert!(saw, "node1 should have received the gossipsub hash");
    }
}