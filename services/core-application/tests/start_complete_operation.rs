use sc_manager_eventbus_nats::{InMemoryEventBus, EventBus};
use sc_manager_app::commands::{StartOperation, handle_start_operation, CompletePhase, handle_complete_phase};
use sc_manager_core::events::signing::{generate_test_keypair, verify_signature};
use sc_manager_core::events::SignedEvent;

#[test]
fn start_operation_publishes_signed_event() {
    let bus = InMemoryEventBus::new();
    let kp = generate_test_keypair().expect("generate test keypair");

    let cmd = StartOperation { operation_id: "op-2".into(), org_id: "org-1".into() };

    let mut sub = bus.subscribe("domain.operations").expect("subscribe");

    handle_start_operation(&bus, &kp, &cmd).expect("handle start");

    let received = sub.next().expect("should receive event");
    let received_signed: SignedEvent = serde_json::from_value(received.payload).expect("deserialize signed event");
    assert!(verify_signature(&received_signed));
    assert_eq!(received_signed.event.kind, "OperationStarted");
}

#[test]
fn complete_phase_publishes_signed_event() {
    let bus = InMemoryEventBus::new();
    let kp = generate_test_keypair().expect("generate test keypair");

    let cmd = CompletePhase { operation_id: "op-2".into(), phase_id: "phase1".into(), org_id: "org-1".into() };

    let mut sub = bus.subscribe("domain.operations").expect("subscribe");

    handle_complete_phase(&bus, &kp, &cmd).expect("handle complete");

    let received = sub.next().expect("should receive event");
    let received_signed: SignedEvent = serde_json::from_value(received.payload).expect("deserialize signed event");
    assert!(verify_signature(&received_signed));
    assert_eq!(received_signed.event.kind, "PhaseCompleted");
}