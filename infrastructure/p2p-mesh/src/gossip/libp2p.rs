#![cfg(feature = "libp2p")]

use crate::gossip::{GossipUpdate, HashGossipMessage};
use libp2p::identity::Keypair;
use libp2p::core::Multiaddr;
use libp2p::PeerId;
use serde_json;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::sync::Arc;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, Mutex};

// Discovery broker shared across both stub and real libp2p paths so that announcements
// always use the *actual* listen addr as observed (not the provided `/tcp/0` placeholder).
use once_cell::sync::Lazy;
use std::sync::Mutex as StdMutex;

static DISCOVERY_BROKER: Lazy<StdMutex<Vec<tokio::sync::mpsc::UnboundedSender<String>>>> = Lazy::new(|| StdMutex::new(Vec::new()));
static DISCOVERY_ADDRS: Lazy<StdMutex<Vec<String>>> = Lazy::new(|| StdMutex::new(Vec::new()));

// Full libp2p behaviour (gossipsub + mdns + kademlia) is gated behind feature.
// Temporary approach: the `libp2p_full` feature currently delegates to the
// deterministic TCP stub until the real libp2p adapter is stabilized.


/// Control messages sent to the libp2p swarm task
#[derive(Debug)]
pub enum Control {
    /// Publish raw message bytes on the gossip topic
    Publish(Vec<u8>),
    /// Dial a multiaddr
    Dial(Multiaddr),
    /// Add explicit gossipsub peer (force mesh entry)
    AddPeer(PeerId),
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
// TODO: Implement actual NetworkBehaviour (Gossipsub + mDNS + Kademlia) here.
// For now we use the TCP-backed stub to keep `libp2p_full` compilable and deterministic in CI.
struct BehaviourPlaceholder;


/// libp2p-backed gossip node using Gossipsub + mDNS + Kademlia.
/// Returns a control sender (for Publish/Dial) and an update receiver stream.
pub async fn new_and_run(
    keypair: Keypair,
    listen_addr: Option<Multiaddr>,
) -> Result<(UnboundedSender<Control>, UnboundedReceiver<GossipUpdate>), GossipError> {
    // Channels
    let (update_tx, update_rx) = unbounded_channel();
let (control_tx, control_rx) = unbounded_channel::<Control>();

    // Optional: full libp2p behaviour (Gossipsub-only minimal implementation).
    // Gated behind `libp2p_full` feature to allow iterative development.
    #[cfg(feature = "libp2p_full")]
    {
        // Real Gossipsub adapter (libp2p 0.48 family)
        use libp2p::gossipsub::{Gossipsub, GossipsubConfigBuilder, IdentTopic, MessageAuthenticity};
        use libp2p::swarm::Swarm;
        use libp2p::PeerId;
        use futures::StreamExt;

        let peer_id = PeerId::from(keypair.public());

        // Configure gossipsub
        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(1))
            .build()
            .map_err(|e| GossipError::Init(format!("{:?}", e)))?;

        use libp2p::gossipsub::IdentityTransform;
        let mut gossipsub: Gossipsub<IdentityTransform> = Gossipsub::new(MessageAuthenticity::Signed(keypair.clone()), gossipsub_config)
            .map_err(|e| GossipError::Init(format!("{:?}", e)))?;

        let topic = IdentTopic::new("scmanager-hash-gossip");
        gossipsub.subscribe(&topic).map_err(|e| GossipError::Init(format!("{:?}", e)))?;

        // mDNS discovery will run in a separate task; for now only init gossipsub here.
        // (Kademlia will be added in a follow-up iteration.)

        // Create a tokio-compatible transport using the development helper for the gossip Swarm
        let transport = libp2p::development_transport(keypair.clone()).await
            .map_err(|e| GossipError::Init(format!("transport init: {:?}", e)))?;

        // Build the gossip-only Swarm (Gossipsub). Discovery will run in a separate mdns task
        let mut swarm = Swarm::new(transport, gossipsub, peer_id.clone());

        // small jitter to allow listeners to fully subscribe before discovery floods
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;

        // If listen addr provided, start listening
        if let Some(ma) = &listen_addr {
            if let Err(e) = swarm.listen_on(ma.clone()) {
                return Err(GossipError::Init(format!("listen failed: {}", e)));
            }
        }

        // In-process discovery broker (emulates mDNS for deterministic local tests).
        use once_cell::sync::Lazy;
        use std::sync::Mutex as StdMutex;

        static DISCOVERY_BROKER: Lazy<StdMutex<Vec<tokio::sync::mpsc::UnboundedSender<String>>>> = Lazy::new(|| StdMutex::new(Vec::new()));
        static DISCOVERY_ADDRS: Lazy<StdMutex<Vec<String>>> = Lazy::new(|| StdMutex::new(Vec::new()));

        // Register a local discovery receiver for this node
        let (disc_tx, mut disc_rx) = unbounded_channel::<String>();
        {
            let mut broker = DISCOVERY_BROKER.lock().unwrap();
            // When a new node registers, send it known announcements so it can discover existing peers
            let addrs = DISCOVERY_ADDRS.lock().unwrap().clone();
            for a in addrs {
                let _ = disc_tx.send(a);
            }

            broker.push(disc_tx.clone());
        }

        // Do NOT announce the *provided* listen_addr (may be `/tcp/0`).  Instead we will
        // announce the real listen address when we receive `NewListenAddr` from the Swarm.
        // (This avoids publishing addresses with port 0 which cause dials to fail.)
        // The per-node registration below will still supply previously-recorded announcements
        // to new joiners.
        // (no-op here)

        // Channel for parsed discovery events to Kademlia
        let (kad_tx, kad_rx) = unbounded_channel::<(String, Multiaddr)>();

        // Spawn a task that listens for discovered addresses and issues Dials to the gossip Swarm
        // and forwards parsed peer_id+addr pairs to the Kademlia task
        let control_tx_discovery = control_tx.clone();
        tokio::spawn(async move {
            while let Some(addr_str) = disc_rx.recv().await {
                // parse optional '<peer_id>|<addr>' format
                if addr_str.contains('|') {
                    let mut parts = addr_str.splitn(2, '|');
                    if let (Some(pid), Some(a)) = (parts.next(), parts.next()) {
                        if let Ok(ma) = a.parse::<Multiaddr>() {
                            eprintln!("discovery: received announcement {} -> {}", pid, a);
                            let _ = control_tx_discovery.send(Control::Dial(ma.clone()));
                            if let Err(e) = kad_tx.send((pid.to_string(), ma.clone())) { eprintln!("discovery: kad_tx send failed: {:?}", e); }
                            continue;
                        } else {
                            eprintln!("discovery: failed to parse addr from announcement: {}", a);
                        }
                    }
                }

                if let Ok(ma) = addr_str.parse::<Multiaddr>() {
                    let _ = control_tx_discovery.send(Control::Dial(ma));
                }
            }
        });

        // Spawn Kademlia task: it runs a Kademlia Swarm and listens for parsed discovery events
        let update_tx_kad = update_tx.clone();
        let mut kad_rx_local = kad_rx; // move into task
        let keypair_kad = keypair.clone();
        let peer_id_kad = peer_id.clone();
        let control_tx_kad = control_tx.clone();
        tokio::spawn(async move {
            use libp2p::kad::{Kademlia, store::MemoryStore, KademliaEvent};
            use std::str::FromStr;

            // Create transport for Kademlia
            let transport = match libp2p::development_transport(keypair_kad.clone()).await {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("kademlia transport init failed: {:?}", e);
                    return;
                }
            };

            let store = MemoryStore::new(peer_id_kad.clone());
            let kademlia = Kademlia::new(peer_id_kad.clone(), store);

            let mut kad_swarm = Swarm::new(transport, kademlia, peer_id_kad);

            loop {
                tokio::select! {
                    event = kad_swarm.select_next_some() => {
                        match event {
                            libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                eprintln!("kademlia: connection established: {}", peer_id);
                                let _ = update_tx_kad.send(GossipUpdate::PeerConnected(peer_id.to_string()));
                                // Also inform gossipsub behaviour to add explicit peer to the mesh
                                let _ = control_tx_kad.send(Control::AddPeer(peer_id));
                            }
                            libp2p::swarm::SwarmEvent::Behaviour(KademliaEvent::RoutingUpdated { peer, .. }) => {
                                eprintln!("kademlia: routing updated for peer: {}", peer);
                                let _ = update_tx_kad.send(GossipUpdate::PeerConnected(peer.to_string()));
                            }
                            _ => {}
                        }
                    }
                    some = kad_rx_local.recv() => {
                        if let Some((peer_str, ma)) = some {
                            eprintln!("kademlia task: got candidate {} -> {}", peer_str, ma);

                            // Ignore announcements that contain /tcp/0 (not yet usable)
                            if ma.to_string().contains("/tcp/0") {
                                eprintln!("kademlia: ignoring candidate with tcp/0: {} -> {}", peer_str, ma);
                                continue;
                            }

                            if let Ok(pid) = PeerId::from_str(&peer_str) {
                                // add address to kademlia and attempt to dial
                                kad_swarm.behaviour_mut().add_address(&pid, ma.clone());
                                if let Err(e) = kad_swarm.dial(ma.clone()) {
                                    eprintln!("kademlia: dial failed: {:?}", e);
                                } else {
                                    eprintln!("kademlia: dial initiated to {}", ma);
                                }
                            } else {
                                eprintln!("kademlia: failed to parse peer id: {}", peer_str);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        });

        // Single task handles gossip swarm events and control messages
        let up = update_tx.clone();
        tokio::spawn(async move {
            let mut swarm = swarm;
            let mut control_rx = control_rx;
            loop {
                tokio::select! {
                    event = swarm.select_next_some() => {
                        match event {
                            libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                                let addr_str = address.to_string();
                                eprintln!("listen: local addr = {}", addr_str);
                                let _ = up.send(GossipUpdate::LocalListenAddr(addr_str.clone()));

                                // Announce to discovery broker using peer id + observed address
                                let s = format!("{}|{}", peer_id.to_string(), addr_str.clone());
                                {
                                    let list = DISCOVERY_BROKER.lock().unwrap().clone();
                                    for tx in list {
                                        let _ = tx.send(s.clone());
                                        eprintln!("discovery: announced {}", s);
                                    }
                                }
                                // record for future joiners
                                {
                                    let mut addrs = DISCOVERY_ADDRS.lock().unwrap();
                                    addrs.push(s.clone());
                                }
                            }
                            libp2p::swarm::SwarmEvent::Behaviour(libp2p::gossipsub::GossipsubEvent::Message { message, .. }) => {
                                eprintln!("gossip: swarm received gossipsub message ({} bytes) from {:?}", message.data.len(), message.source);
                                if let Ok(msg) = serde_json::from_slice::<HashGossipMessage>(&message.data) {
                                    let peer = message.source.map(|p| p.to_string()).unwrap_or_else(|| "".to_string());
                                    eprintln!("gossip: parsed HashGossipMessage from peer {} entity {}", peer, msg.entity_id);
                                    let _ = up.send(GossipUpdate::HashReceived { peer_id: peer, entity_id: msg.entity_id.clone(), hash: msg.state_hash.clone() });
                                }
                            }
                            libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                let _ = up.send(GossipUpdate::PeerConnected(peer_id.to_string()));
                            }
                            libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                let _ = up.send(GossipUpdate::PeerDisconnected(peer_id.to_string()));
                            }
                            _ => {}
                        }
                    }
                    ctrl = control_rx.recv() => {
                        match ctrl {
                            Some(Control::Dial(ma)) => {
                                eprintln!("gossip: control requested dial {}", ma);
                                if let Err(e) = swarm.dial(ma) { eprintln!("gossip: dial error: {:?}", e); }
                            }
                            Some(Control::Publish(data)) => {
                                eprintln!("gossip: publish requested ({} bytes)", data.len());
                                match swarm.behaviour_mut().publish(topic.clone(), data) {
                                    Ok(mid) => eprintln!("gossip: publish ok: {:?}", mid),
                                    Err(e) => eprintln!("gossip: publish error: {:?}", e),
                                }
                            }
                            Some(Control::AddPeer(pid)) => {
                                eprintln!("gossip: adding explicit peer {} to gossipsub mesh", pid);
                                swarm.behaviour_mut().add_explicit_peer(&pid);
                            }
                            None => { break; }
                        }
                    }
                }
            }
        });

        return Ok((control_tx, update_rx));
    }
    // Use TCP stub implementation (shared helper). Pass peer id string so stub can announce
    // its observed listen address to the discovery broker.
    let peer_str = PeerId::from(keypair.public()).to_string();
    spawn_tcp_stub(listen_addr.clone(), update_tx.clone(), control_rx, peer_str).await?;
    return Ok((control_tx, update_rx));
}

// Helper: spawn the TCP-backed stub (factorized for reuse by the experimental libp2p feature)
async fn spawn_tcp_stub(
    listen_addr: Option<Multiaddr>,
    update_tx: UnboundedSender<GossipUpdate>,
    mut control_rx: UnboundedReceiver<Control>,
    peer_id: String,
) -> Result<(), GossipError> {
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
            eprintln!("stub: observed listen addr {} for {}", addr_str, peer_id);
            let _ = update_tx.send(GossipUpdate::LocalListenAddr(addr_str.clone()));

            // announce via discovery broker (peer|addr) so other nodes can dial
            let s = format!("{}|{}", peer_id, addr_str.clone());
            {
                let list = DISCOVERY_BROKER.lock().unwrap().clone();
                for tx in list {
                    let _ = tx.send(s.clone());
                    eprintln!("stub discovery: announced {}", s);
                }
            }
            // record announcement for future joiners
            {
                let mut addrs = DISCOVERY_ADDRS.lock().unwrap();
                addrs.push(s.clone());
            }
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
                                    eprintln!("stub: listener read message from peer_id={} entity_id={}", msg.peer_id, msg.entity_id);
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
                    eprintln!("gossip: control requested dial {}", s);
                    if s.contains("/tcp/0") {
                        eprintln!("gossip: skipping dial to tcp/0 address: {}", s);
                        continue;
                    }
                    let parts: Vec<&str> = s.split('/').filter(|p| !p.is_empty()).collect();
                    if parts.len() >= 4 && parts[0] == "ip4" && parts[2] == "tcp" {
                        let ip = parts[1];
                        let port: u16 = parts[3].parse().unwrap_or(0);
                        let socket = format!("{}:{}", ip, port);
                        match tokio::net::TcpStream::connect(&socket).await {
                            Ok(stream) => {
                                eprintln!("gossip: dial succeeded to {}", socket);
                                let peer_addr = stream.peer_addr().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
                                let (tx, mut rx) = unbounded_channel::<Vec<u8>>();
                                peers_ctrl.lock().await.insert(peer_addr, tx);
                                eprintln!("gossip: inserted peer tx for {}", peer_addr);

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
                                            eprintln!("stub: client read message from peer_id={} entity_id={}", msg.peer_id, msg.entity_id);
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
                                        eprintln!("stub: client writer sending {} bytes to {}", msg.len(), peer_addr);
                                        if let Err(e) = writer.write_all(&len).await { eprintln!("stub: client writer write len failed: {:?}", e); break; }
                                        if let Err(e) = writer.write_all(&msg).await { eprintln!("stub: client writer write msg failed: {:?}", e); break; }
                                    }
                                });

                                let _ = update_tx_ctrl.send(GossipUpdate::PeerConnected(peer_addr.to_string()));
                            }
                            Err(e) => { eprintln!("gossip: dial error: {:?}", e); }
                        } // end match TcpStream::connect
                    } // end if parts.len() check
                } // end Control::Dial arm
                Control::Publish(data) => {
                    // Forward to all peers
                    let peers_map = peers_ctrl.lock().await;
                    eprintln!("stub: publish forwarding to {} peers", peers_map.len());
                    for (_addr, tx) in peers_map.iter() {
                        match tx.send(data.clone()) {
                            Ok(_) => eprintln!("stub: publish send succeeded"),
                            Err(e) => eprintln!("stub: publish send failed: {:?}", e),
                        }
                    }
                }
                Control::AddPeer(_) => {
                    // no-op for TCP stub
                }
            }
        }
    });

    Ok(())
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
        sleep(Duration::from_secs(2)).await; // allow connection and gossipsub mesh formation

        let msg = HashGossipMessage {
            entity_id: "global".into(),
            state_hash: { let mut h = Sha3_256::new(); h.update(b"state-v1"); hex::encode(h.finalize()) },
            timestamp: 0,
            peer_id: "node2".into(),
        };
        let data = serde_json::to_vec(&msg).expect("serialize");
        ctrl2.send(Control::Publish(data)).expect("publish");

        let mut saw = false;
        for _ in 0..80 {
            if let Some(u) = rx1.recv().await {
                eprintln!("test: rx1 got update: {:?}", u);
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
        // This test uses discovery (peer_id|addr) announcements and Kademlia skeleton to auto-connect.
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
        let _node1_multi: Multiaddr = node1_addr.parse().expect("parse multiaddr");

        // Start node2 and do NOT explicitly dial; discovery + kademlia should trigger connection
        let (_ctrl2, mut rx2) = new_and_run(k2.clone(), Some("/ip4/127.0.0.1/tcp/0".parse().unwrap())).await.expect("node2 start");

        // Wait for node2 to discover and connect
        let mut saw_connection = false;
        for _ in 0..80 {
            if let Some(msg) = rx2.recv().await {
                if let GossipUpdate::PeerConnected(peer) = msg {
                    // peer will be node1's peer id
                    if !peer.is_empty() {
                        eprintln!("test: node2 observed PeerConnected: {}", peer);
                        saw_connection = true;
                        break;
                    }
                }
            }
            sleep(Duration::from_millis(50)).await;
        }

        assert!(saw_connection, "node2 should have connected via discovery/kademlia");

        // Ensure node1 also observed the connection before publishing
        let mut node1_seen = false;
        for _ in 0..80 {
            if let Some(msg) = rx1.recv().await {
                if let GossipUpdate::PeerConnected(peer) = msg {
                    if !peer.is_empty() {
                        eprintln!("test: node1 observed PeerConnected: {}", peer);
                        node1_seen = true;
                        break;
                    }
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
        assert!(node1_seen, "node1 should also have connection established before publish");

        // Now publish from node2 and ensure node1 receives it
        let msg = HashGossipMessage {
            entity_id: "global".into(),
            state_hash: { let mut h = Sha3_256::new(); h.update(b"state-v1"); hex::encode(h.finalize()) },
            timestamp: 0,
            peer_id: "node2".into(),
        };
        let data = serde_json::to_vec(&msg).expect("serialize");
        // find a Control::Publish channel for node2 by creating a new node that sends via control - we have _ctrl2 but not used here
        // Use the spied behaviour: publish via the control channel (if exposed) - instead, send via a Dial+Publish flow

        // Wait a short while to ensure connection and gossipsub mesh formation
        sleep(Duration::from_secs(2)).await;

        // Use the discovery-based dial to send the message: create a fresh control and publish via it
        let ctrl_publish = _ctrl2;
        let _data = serde_json::to_vec(&msg).expect("serialize");
        ctrl_publish.send(Control::Publish(_data)).expect("publish");

        let mut saw = false;
        for _ in 0..80 {
            if let Some(u) = rx1.recv().await {
                eprintln!("test_full: rx1 got update: {:?}", u);
                if let GossipUpdate::HashReceived { peer_id: _peer_id, entity_id, hash } = u {
                    if entity_id == "global" && hash == msg.state_hash {
                        saw = true;
                        break;
                    }
                }
            }
            sleep(Duration::from_millis(50)).await;
        }

        assert!(saw, "node1 should have received the gossipsub hash via discovery+kad connection");
    }
}