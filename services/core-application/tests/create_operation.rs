use sc_manager_eventbus_nats::{InMemoryEventBus, EventBus};
use sc_manager_app::commands::{CreateOperation, handle_create_operation};
use sc_manager_core::events::{SignedEvent};
use sc_manager_core::events::signing::{generate_test_keypair, verify_signature};

#[test]
fn create_operation_publishes_signed_event() {
    let bus = InMemoryEventBus::new();
    let kp = generate_test_keypair().expect("generate test keypair");

    let cmd = CreateOperation { id: "op-1".into(), name: "Recon".into(), mission_type: "explore".into(), org_id: "org-1".into() };

    let mut sub = bus.subscribe("domain.operations").expect("subscribe");

    handle_create_operation(&bus, &kp, &cmd).expect("handle create");

    let received = sub.next().expect("should receive event");
    let received_signed: SignedEvent = serde_json::from_value(received.payload).expect("deserialize signed event");
    assert!(verify_signature(&received_signed));
    assert_eq!(received_signed.event.kind, "OperationCreated");
}
