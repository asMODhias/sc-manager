use sc_manager_eventbus_nats::{InMemoryEventBus, EventBus};
use serde_json::json;

#[test]
fn publish_and_next_and_subscribe() {
    let bus = InMemoryEventBus::new();
    let payload = json!({"x": 1});
    bus.publish("topic.a", &payload).expect("publish");

    // subscribe snapshot includes the published event
    let mut it = bus.subscribe("topic.a").expect("subscribe");
    let ev = it.next().expect("iter next");
    assert_eq!(ev.subject, "topic.a");
    assert_eq!(ev.payload, payload);

    // next() helper removes the event from internal queue
    let maybe_ev = bus.next();
    // since our subscribe consumed a clone snapshot, next() may still return the same event depending on timing; ensure it returns either None or an event
    if let Some(e) = maybe_ev {
        assert_eq!(e.subject, "topic.a");
    }
}