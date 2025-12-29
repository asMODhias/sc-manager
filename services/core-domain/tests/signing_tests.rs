use sc_manager_core::events::{DomainEventPayload, sign_event, verify_signature, generate_test_keypair};

#[test]
fn sign_verify_roundtrip() {
    let kp = generate_test_keypair();
    let ev = DomainEventPayload { id: "e1".into(), kind: "Test".into(), payload: serde_json::json!({"x":1}) };
    let s = sign_event(&kp, &ev);
    assert!(verify_signature(&s));
}
