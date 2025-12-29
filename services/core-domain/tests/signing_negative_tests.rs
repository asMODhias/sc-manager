use sc_manager_core::events::{DomainEventPayload, SignedEvent, sign_event, verify_signature, generate_test_keypair};

#[test]
fn invalid_public_key_returns_false() {
    let kp = generate_test_keypair();
    let ev = DomainEventPayload { id: "e1".into(), kind: "Test".into(), payload: serde_json::json!({"x":1}) };
    let s = sign_event(&kp, &ev);
    let bad = SignedEvent { event: s.event.clone(), public_key: vec![0u8], signature: s.signature.clone() };
    assert!(!verify_signature(&bad));
}

#[test]
fn invalid_signature_bytes_returns_false() {
    let kp = generate_test_keypair();
    let ev = DomainEventPayload { id: "e2".into(), kind: "Test".into(), payload: serde_json::json!({"x":2}) };
    let s = sign_event(&kp, &ev);
    let bad = SignedEvent { event: s.event.clone(), public_key: s.public_key.clone(), signature: vec![0u8] };
    assert!(!verify_signature(&bad));
}
