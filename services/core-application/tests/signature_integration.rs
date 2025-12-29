use sc_manager_app::signing::{DomainEvent, SignedEvent, sign_event, verify_signature, generate_test_keypair};

#[test]
fn integration_sign_verify_pub_sub() {
    let kp = generate_test_keypair();
    let ev = DomainEvent { id: "evt-1".into(), kind: "UserCreated".into(), payload: serde_json::json!({"user":"alice"}) };
    let s = sign_event(&kp, &ev).expect("sign").expect("sign");
    assert!(verify_signature(&s));
}
