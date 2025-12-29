use services::core_domain::events::signing::{DomainEventPayload, generate_test_keypair, sign_event, verify_signature};
use serde_json::json;

#[test]
fn sign_and_verify_operation_event() {
    let payload = DomainEventPayload {
        id: "evt1".to_string(),
        kind: "OperationPhaseCompleted".to_string(),
        payload: json!({"operation_id":"op1","phase_id":"p1","success":true}),
    };

    let kp = generate_test_keypair();
    let signed = sign_event(&kp, &payload);
    assert!(verify_signature(&signed));
}
