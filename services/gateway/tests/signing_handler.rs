use serde_json::json;
use sc_manager_core::events::{DomainEventPayload, sign_event, verify_signature, generate_test_keypair};

#[test]
fn create_signed_payload_and_verify() {
    let kp = generate_test_keypair().expect("generate test keypair");
    let body = json!({"kind":"test","foo":"bar"});
    let ev = DomainEventPayload { id: "e-test".into(), kind: "test".into(), payload: body.clone() };
    let s = sign_event(&kp, &ev);
    assert!(verify_signature(&s));
}
