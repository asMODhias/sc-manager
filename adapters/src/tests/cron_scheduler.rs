use std::sync::Arc;
use std::time::Duration;
use crate::*;

#[tokio::test(flavor = "multi_thread")]
async fn cron_expression_triggers_fetch() {
    let mut r = AdapterRegistry::new();
    let a = Arc::new(crate::discord::InMemoryDiscordAdapter::new()) as Arc<dyn DataAdapter>;
    // Cron expression every second (six-field with seconds): "*/1 * * * * *"
    r.register(a, Schedule::Cron("*/1 * * * * *".into()));

    let mock = super::tests::MockPublisher::new();
    let msgs = mock.messages();
    r.start_all(Some(std::sync::Arc::new(mock)), None).await.expect("start");

    // Wait up to 2s for at least one publish
    tokio::time::sleep(Duration::from_millis(1800)).await;

    let m = msgs.lock().unwrap();
    assert!(!m.is_empty(), "expected at least one published message from cron");
}
