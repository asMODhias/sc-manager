use serde_json::Value as JsonValue;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread::sleep;

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
        let mut evs = match self.events.lock() {
            Ok(g) => g,
            Err(_) => return None,
        };
        if evs.len() == 0 { None } else { Some(evs.remove(0)) }
    }
}

struct Subscription {
    events: Arc<Mutex<Vec<Event>>>,
    idx: usize,
    timeout_ms: u64,
}

impl Subscription {
    fn new(events: Arc<Mutex<Vec<Event>>>, timeout_ms: u64) -> Self {
        Self { events, idx: 0, timeout_ms }
    }
}

impl Iterator for Subscription {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        let start = Instant::now();
        loop {
            let mut evs = match self.events.lock() {
                Ok(g) => g,
                Err(_) => return None,
            };
            if self.idx < evs.len() {
                // remove the event at current index and return it
                let ev = evs.remove(self.idx);
                return Some(ev);
            }
            drop(evs);
            if start.elapsed() > Duration::from_millis(self.timeout_ms) {
                return None;
            }
            sleep(Duration::from_millis(10));
        }
    }
}

impl EventBus for InMemoryEventBus {
    fn publish(&self, subject: &str, payload: &JsonValue) -> Result<(), String> {
        let mut evs = self.events.lock().map_err(|_| "eventbus lock poisoned".to_string())?;
        evs.push(Event { subject: subject.to_string(), payload: payload.clone() });
        Ok(())
    }

    fn subscribe(&self, _subject: &str) -> Result<Box<dyn Iterator<Item = Event> + Send>, String> {
        // return an iterator that waits up to 1 second for events
        Ok(Box::new(Subscription::new(self.events.clone(), 1000)))
    }
}
