use sc_manager_eventbus_nats::EventBus;
use sc_manager_core::events::{DomainEventPayload, sign_event, KeyPair};
use serde_json::Value as JsonValue;

/// Sign `DomainEventPayload` and publish `SignedEvent` as JSON on the EventBus
pub fn sign_and_publish<E: EventBus>(bus: &E, topic: &str, kp: &KeyPair, ev: &DomainEventPayload) -> Result<(), String> {
    let signed = sign_event(kp, ev).map_err(|e| format!("sign: {}", e))?;
    let payload: JsonValue = serde_json::to_value(&signed).map_err(|e| format!("serialize: {}", e))?;
    bus.publish(topic, &payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_manager_eventbus_nats::InMemoryEventBus;
    use sc_manager_core::events::{generate_test_keypair, verify_signature};

    #[test]
    fn sign_and_publish_happy_path() {
        let bus = InMemoryEventBus::new();
        let kp = generate_test_keypair();
        let ev = DomainEventPayload { id: "evt-1".into(), kind: "UserCreated".into(), payload: serde_json::json!({"user":"bob"}) };
        let mut sub = bus.subscribe("domain.events").expect("subscribe");

        sign_and_publish(&bus, "domain.events", &kp, &ev).expect("publish");

        let received = sub.next().expect("should receive event");
        let received_signed: sc_manager_core::events::SignedEvent = serde_json::from_value(received.payload).expect("deserialize signed event");
        assert!(verify_signature(&received_signed));
    }
}
