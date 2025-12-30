use std::env;
use std::time::Duration;
use tokio::time::timeout;
use futures_util::StreamExt;
use uuid::Uuid;

#[tokio::test]
async fn nats_to_redis_integration() {
    // Only run in E2E mode
    if env::var("RUN_INTEGRATION_TESTS").unwrap_or_else(|_| "0".into()) != "1" {
        eprintln!("Skipping Redis integration test; set RUN_INTEGRATION_TESTS=1 to run");
        return;
    }

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".into());
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());

    // Connect to Redis (async)
    let client = redis::Client::open(redis_url.as_str()).expect("create redis client");
    let mut conn = client.get_multiplexed_async_connection().await.expect("redis connect");

    // Connect to NATS
    let nc = async_nats::connect(nats_url).await.expect("connect nats");

    let mut sub = nc.subscribe("domain.events").await.expect("subscribe");

    // Wait up to 30s for a message published by adapters during E2E
    let got = tokio::spawn(async move {
        match timeout(Duration::from_secs(30), sub.next()).await {
            Ok(Some(msg)) => {
                let b = msg.payload;
                // store in redis as a simple key (it:events:<id>) if event contains id
                let s = String::from_utf8_lossy(&b).to_string();
                // store raw
                let key = format!("it:events:{}", Uuid::new_v4());
                let _: () = redis::cmd("SET").arg(&[&key, &s]).query_async(&mut conn).await.expect("redis set");
                Some(key)
            }
            _ => None,
        }
    });

    let res = got.await.expect("task result");
    if res.is_none() {
        panic!("No domain.events message received within timeout");
    }
    let key = res.unwrap();

    // Now verify the key exists in Redis
    let mut conn2 = client.get_multiplexed_async_connection().await.expect("redis connect2");
    let val: Option<String> = redis::cmd("GET").arg(&[&key]).query_async(&mut conn2).await.expect("redis get");
    assert!(val.is_some(), "Expected value stored in Redis by test relay");
}
