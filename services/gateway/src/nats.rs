use log::debug;
use std::sync::Arc;

/// A lightweight wrapper maintaining a persistent async-nats connection.
/// `NatsClient` is `Clone` via internal `Arc` so it can be shared across handlers.
#[derive(Clone)]
pub struct NatsClient {
    inner: Arc<async_nats::Client>,
}

impl NatsClient {
    /// Connect to the NATS server and return a managed client.
    pub async fn connect(nats_url: &str) -> Result<Self, String> {
        // connect once; let async-nats manage reconnections
        let client = async_nats::connect(nats_url).await.map_err(|e| format!("connect: {}", e))?;
        Ok(NatsClient { inner: Arc::new(client) })
    }

    /// Publish a message using the persistent client.
    pub async fn publish(&self, subject: &str, payload: &[u8]) -> Result<(), String> {
        // allow RUST_LOG=debug or explicit env override
        let env_override = std::env::var("GATEWAY_DEBUG_PUBLISH").map(|v| v == "1" || v.eq_ignore_ascii_case("true")).unwrap_or(false);
        let publish_debug = env_override || log::log_enabled!(log::Level::Debug);

        if publish_debug {
            debug!("publishing to nats: addr={} subject={} len={}", "<conn>", subject, payload.len());
        }

        let subj = subject.to_string();
        let data = payload.to_vec();
        self.inner.publish(subj, data.into()).await.map_err(|e| format!("publish: {}", e))?;

        // Ensure the server has processed the publish before returning (helps avoid timing/race flakiness).
        self.inner.flush().await.map_err(|e| format!("flush: {}", e))?;

        if publish_debug {
            debug!("published to nats: subject={} len={}", subject, payload.len());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connect_invalid_url_returns_err() {
        // invalid port should fail to connect
        let res = NatsClient::connect("nats://127.0.0.1:9").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn smoke_publish_when_local_nats() {
        if std::env::var("RUN_LOCAL_NATS_TESTS").ok().as_deref() != Some("1") {
            eprintln!("Skipping local NATS smoke test; set RUN_LOCAL_NATS_TESTS=1 to run");
            return;
        }

        let client = NatsClient::connect("nats://127.0.0.1:4222").await.expect("connect");
        client.publish("test.smoke", b"hello").await.expect("publish");
    }
}
