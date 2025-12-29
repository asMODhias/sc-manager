use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::*;

#[tokio::test(flavor = "multi_thread")]
async fn metrics_are_recorded_for_fetch() {
    let mut r = AdapterRegistry::new();
    let a = Arc::new(crate::discord::InMemoryDiscordAdapter::new()) as Arc<dyn DataAdapter>;
    r.register(a, Schedule::Fixed(Duration::from_secs(1)));

    let metrics = prometheus::Registry::new();
    r.start_all(None, Some(Arc::new(metrics.clone()))).await.expect("start");

    // Allow one tick
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // Gather metrics and assert our counters exist and have values
    let mf = metrics.gather();
    let names: Vec<_> = mf.iter().map(|m| m.get_name().to_string()).collect();
    assert!(names.iter().any(|n| n == "adapter_fetch_success_total"));
}