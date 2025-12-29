use sc_manager_p2p::transport::{InMemoryTransportRegistry, MockQuicTransport};
use sc_manager_p2p::{KeyPair, SignedEvent, Signer, Transport};

#[test]
fn multi_peer_quic_simulation_broadcast() {
    // Simulate a small in-process QUIC-like network and broadcast a signed event
    let reg = InMemoryTransportRegistry::new();
    let t0 = MockQuicTransport::new("node-0", reg.clone());
    let t1 = MockQuicTransport::new("node-1", reg.clone());
    let t2 = MockQuicTransport::new("node-2", reg.clone());

    // Create an event from node-0
    let kp = KeyPair::generate().unwrap();
    let payload = "op:multi-announce".to_string();
    let sig = kp.sign(&payload);
    let ev = SignedEvent { id: "evt-q1".into(), payload: payload.clone(), signer_id: kp.id.clone(), signature: sig };
    let bytes = serde_json::to_vec(&ev).unwrap();

    // Broadcast by sending to each node
    t0.send("node-1", bytes.clone()).expect("send1");
    t0.send("node-2", bytes.clone()).expect("send2");

    // Verify node-1 and node-2 received and verified the event
    let r1 = t1.subscribe().next().expect("r1");
    let r2 = t2.subscribe().next().expect("r2");

    let e1: SignedEvent = serde_json::from_slice(&r1.payload).unwrap();
    let e2: SignedEvent = serde_json::from_slice(&r2.payload).unwrap();

    assert_eq!(e1.payload, payload);
    assert_eq!(e2.payload, payload);
    assert!(e1.verify(&kp));
    assert!(e2.verify(&kp));
}
