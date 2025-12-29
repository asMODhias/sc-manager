use sc_manager_eventbus_nats::{InMemoryEventBus, EventBus};
use sc_manager_core::events::signing::{DomainEventPayload, generate_test_keypair, sign_event, verify_signature, SignedEvent};
use serde_json::json;

#[test]
fn publish_signed_event_roundtrip_using_local_sign() {
    let bus = InMemoryEventBus::new();
    let kp = generate_test_keypair();
    let ev = DomainEventPayload { id: "evt-app-1".into(), kind: "OperationCreated".into(), payload: json!({"operation_id":"opX"}) };

    let signed = sign_event(&kp, &ev);
    let mut sub = bus.subscribe("domain.events").expect("subscribe");

    let json_payload = serde_json::to_value(&signed).expect("serialize signed event");
    bus.publish("domain.events", &json_payload).expect("publish");

    let received = sub.next().expect("should receive event");
    let received_signed: SignedEvent = serde_json::from_value(received.payload).expect("deserialize signed event");
    assert!(verify_signature(&received_signed));
    assert_eq!(received_signed.event.id, "evt-app-1");
}

#[test]
fn verify_signature_returns_false_for_bad_key() {
    let payload = DomainEventPayload { id: "evt-bad-1".into(), kind: "X".into(), payload: json!({}) };
    let kp = generate_test_keypair();
    let mut signed = sc_manager_core::events::sign_event(&kp, &payload);

    // Corrupt the public key
    signed.public_key[0] = signed.public_key[0].wrapping_add(1);

    assert!(!verify_signature(&signed));
}

#[test]
fn verify_signature_returns_false_for_bad_sig() {
    let payload = DomainEventPayload { id: "evt-bad-2".into(), kind: "X".into(), payload: json!({}) };
    let kp = generate_test_keypair();
    let mut signed = sc_manager_core::events::sign_event(&kp, &payload);

    // Corrupt the signature
    signed.signature[0] = signed.signature[0].wrapping_add(1);

    assert!(!verify_signature(&signed));
}
