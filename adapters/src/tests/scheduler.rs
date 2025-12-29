use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::*;

struct MockPublisher { msgs: Arc<Mutex<Vec<(String, Vec<u8>)>>> }
impl MockPublisher { fn new() -> Self { Self { msgs: Arc::new(Mutex::new(Vec::new())) } }
    fn messages(&self) -> Arc<Mutex<Vec<(String, Vec<u8>)>>> { self.msgs.clone() }
}
impl EventPublisher for MockPublisher {
    fn publish<'a>(&'a self, subject: &'a str, payload: Vec<u8>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        let msgs = self.msgs.clone();
        let subj = subject.to_string();
        Box::pin(async move {
            msgs.lock().unwrap().push((subj, payload));
            Ok(())
        })
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn scheduler_publishes_adapter_data() {
    let mut r = AdapterRegistry::new();
    let a = Arc::new(crate::discord::InMemoryDiscordAdapter::new()) as Arc<dyn DataAdapter>;
    r.register(a, Schedule::Fixed(Duration::from_secs(1)));

    let mock = MockPublisher::new();
    let msgs = mock.messages();
    r.start_all(Some(Arc::new(mock))).await.expect("start");

    // Wait for at least one tick (1s interval plus small buffer)
    tokio::time::sleep(Duration::from_millis(1500)).await;

    let m = msgs.lock().unwrap();
    assert!(!m.is_empty(), "expected at least one published message");
    // Optionally inspect payload JSON
    let (subj, payload) = &m[0];
    assert!(subj.starts_with("adapters."));
    let _v: serde_json::Value = serde_json::from_slice(payload).expect("valid json");
}