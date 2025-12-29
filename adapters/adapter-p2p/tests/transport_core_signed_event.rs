use sc_manager_p2p::{Transport, transport::{InMemoryTransportRegistry, MockQuicTransport}};
use sc_manager_core::events::{DomainEventPayload, sign_event, verify_signature, generate_test_keypair, SignedEvent};
use serde_json::json;

#[test]
fn transport_core_signed_event_roundtrip() {
    let reg = InMemoryTransportRegistry::new();
    let a = MockQuicTransport::new("node-a", reg.clone());
    let b = MockQuicTransport::new("node-b", reg.clone());

    // Use the deterministic test keypair from core (same secret) for signing
    let kp = generate_test_keypair();
    let ev = DomainEventPayload { id: "evt-01".into(), kind: "UserCreated".into(), payload: json!({"user":"alice"}) };
    let s: SignedEvent = sign_event(&kp, &ev);

    // Send core SignedEvent JSON over the MockQuicTransport
    let bytes = serde_json::to_vec(&s).expect("serialize core signed event");
    a.send("node-b", bytes).expect("send");

    // Receiver deserializes and verifies using the embedded public_key
    let mut it = b.subscribe();
    let m = it.next().expect("should receive message");
    let received: SignedEvent = serde_json::from_slice(&m.payload).expect("deserialize core signed event");
    assert!(verify_signature(&received));
    assert_eq!(received.event.payload, json!({"user":"alice"}));
}
