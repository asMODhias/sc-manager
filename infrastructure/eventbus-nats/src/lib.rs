use serde_json::Value as JsonValue;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub subject: String,
    pub payload: JsonValue,
}

/// EventBus trait to be implemented by NATS client or in-memory mock
pub trait EventBus: Send + Sync {
    fn publish(&self, subject: &str, payload: &JsonValue) -> Result<(), String>;
    fn subscribe(&self, subject: &str) -> Result<Box<dyn Iterator<Item = Event> + Send>, String>;
}

/// In-memory EventBus: used for tests and local harness
#[derive(Clone)]
pub struct InMemoryEventBus {
    events: Arc<Mutex<Vec<Event>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self { Self { events: Arc::new(Mutex::new(vec![])) } }

    pub fn next(&self) -> Option<Event> {
        let mut evs = self.events.lock().unwrap();
        if evs.len() == 0 { None } else { Some(evs.remove(0)) }
    }
}

impl EventBus for InMemoryEventBus {
    fn publish(&self, subject: &str, payload: &JsonValue) -> Result<(), String> {
        let mut evs = self.events.lock().unwrap();
        evs.push(Event { subject: subject.to_string(), payload: payload.clone() });
        Ok(())
    }

    fn subscribe(&self, _subject: &str) -> Result<Box<dyn Iterator<Item = Event> + Send>, String> {
        // naive: return an iterator over current snapshot (cloned)
        let evs = self.events.lock().unwrap().clone();
        let v = evs.into_iter();
        Ok(Box::new(v))
    }
}
