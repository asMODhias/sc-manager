use sc_manager_core::events::signing::{DomainEventPayload, generate_test_keypair, sign_event, verify_signature, SignedEvent};
use serde_json::json;

#[test]
fn sign_and_verify_operation_event() {
    let payload = DomainEventPayload {
        id: "evt-op-1".to_string(),
        kind: "OperationCreated".to_string(),
        payload: json!({"operation_id":"op1","name":"Recon","org_id":"org-1"}),
    };

    let kp = generate_test_keypair();
    let signed = sign_event(&kp, &payload).expect("sign event");
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
    let signed = sign_event(&kp, &payload).expect("sign event");

    // Serialize/deserialize roundtrip (simulate publish as JSON)
    let json_payload = serde_json::to_value(&signed).expect("serialize signed event");
    let received_signed: SignedEvent = serde_json::from_value(json_payload).expect("deserialize signed event");
    assert!(verify_signature(&received_signed));
    assert_eq!(received_signed.event.kind, "OperationStarted");
}
