use sc_manager_core::events::signing::{DomainEventPayload, generate_test_keypair, sign_event, verify_signature, SignedEvent};
use sc_manager_eventbus_nats::{InMemoryEventBus, EventBus};
use serde_json::json;

#[test]
fn sign_and_verify_operation_event() {
    let payload = DomainEventPayload {
        id: "evt-op-1".to_string(),
        kind: "OperationCreated".to_string(),
        payload: json!({"operation_id":"op1","name":"Recon","org_id":"org-1"}),
    };

    let kp = generate_test_keypair();
    let signed = sign_event(&kp, &payload);
    assert!(verify_signature(&signed));
}

#[test]
fn publish_signed_operation_event_roundtrip() {
    let payload = DomainEventPayload {
        id: "evt-op-2".to_string(),
        kind: "OperationStarted".to_string(),
        payload: json!({"operation_id":"op2","org_id":"org-1"}),
    };

    let kp = generate_test_keypair();
    let signed = sign_event(&kp, &payload);

    let bus = InMemoryEventBus::new();
    let mut sub = bus.subscribe("domain.operations").expect("subscribe");

    // Publish the signed event as JSON (what real publishers do)
    let json_payload = serde_json::to_value(&signed).expect("serialize signed event");
    bus.publish("domain.operations", &json_payload).expect("publish");

    let received = sub.next().expect("should receive event");
    let received_signed: SignedEvent = serde_json::from_value(received.payload).expect("deserialize signed event");
    assert!(verify_signature(&received_signed));
    assert_eq!(received_signed.event.kind, "OperationStarted");
}
