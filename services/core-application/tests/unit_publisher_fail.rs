use sc_manager_app::services::publisher::sign_and_publish;
use sc_manager_eventbus_nats::EventBus;
use sc_manager_core::events::{DomainEventPayload, generate_test_keypair};

struct FailBus;
impl EventBus for FailBus {
    fn publish(&self, _subject: &str, _payload: &serde_json::Value) -> Result<(), String> {
        Err("boom".to_string())
    }

    fn subscribe(&self, _subject: &str) -> Result<Box<dyn Iterator<Item = sc_manager_eventbus_nats::Event> + Send>, String> {
        Err("no subs".to_string())
    }
}

#[test]
fn sign_and_publish_returns_err_on_bus_publish_fail() {
    let bus = FailBus;
    let kp = generate_test_keypair().expect("generate test keypair");
    let ev = DomainEventPayload { id: "evt-1".into(), kind: "UserCreated".into(), payload: serde_json::json!({"user":"bob"}) };

    let res = sign_and_publish(&bus, "domain.events", &kp, &ev);
    assert!(res.is_err());
}
