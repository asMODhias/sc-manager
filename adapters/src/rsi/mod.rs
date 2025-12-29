//! RSI Handle verification adapter (outbound)
//!
//! Provides a small trait for verifying RSI handles and an in-memory client used for tests.

use std::collections::HashSet;

/// Minimal RSI client trait for handle verification.
pub trait RsiClient {
    /// Verify whether an RSI handle exists.
    /// Returns Ok(true) if verified, Ok(false) if handle not found, Err(_) for client errors.
    fn verify_handle(&self, handle: &str) -> Result<bool, String>;
}

/// Simple in-memory RSI client used for tests and local dev.
pub struct SimpleRsiClient {
    known: HashSet<String>,
    simulate_error: bool,
    pub name: String,
}

impl SimpleRsiClient {
    pub fn new<K: Into<String>>(handles: impl IntoIterator<Item = K>) -> Self {
        Self {
            known: handles.into_iter().map(|k| k.into()).collect(),
            simulate_error: false,
            name: "simple-rsi".into(),
        }
    }

    pub fn with_error(mut self, val: bool) -> Self {
        self.simulate_error = val;
        self
    }
}

impl RsiClient for SimpleRsiClient {
    fn verify_handle(&self, handle: &str) -> Result<bool, String> {
        if self.simulate_error {
            return Err("simulated network error".to_string());
        }
        Ok(self.known.contains(handle))
    }
}

// Implement the canonical DataAdapter trait so this adapter can be managed by the registry
use crate::adapter_api::{AdapterData, AdapterHealth, DataAdapter, HealthStatus, RateLimit};
use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;
use std::time::Duration;

impl SimpleRsiClient {
    // expose friendly name
    pub fn with_name(mut self, name: &str) -> Self { self.name = name.to_string(); self }
}

#[async_trait]
impl DataAdapter for SimpleRsiClient {
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { "0.1" }
    fn source_url(&self) -> &str { "" }
    fn attribution(&self) -> &str { "Internal Test Adapter" }
    fn license(&self) -> &str { "MIT" }

    async fn health_check(&self) -> Result<AdapterHealth, String> {
        Ok(AdapterHealth { status: HealthStatus::Healthy, last_success: Some(Utc::now()), last_error: None, uptime_percentage: 100.0, requests_per_hour: 0 })
    }

    async fn fetch(&self) -> Result<AdapterData, String> {
        let arr: Vec<_> = self.known.iter().map(|h| json!({"handle": h})).collect();
        Ok(AdapterData::Other(json!(arr)))
    }

    fn rate_limit(&self) -> RateLimit { RateLimit { requests_per_hour: 10000, burst: 100 } }
    fn cache_ttl(&self) -> Duration { Duration::from_secs(60 * 60) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdapterRegistry;
    use std::collections::HashMap;
    use crate::adapter_api::AdapterHealth;

    #[tokio::test]
    async fn verify_known_handle() {
        let c = SimpleRsiClient::new(vec!["alice", "bob"]);
        assert!(c.verify_handle("alice").expect("verify_handle failed in test"));
        assert!(!c.verify_handle("charlie").expect("verify_handle failed in test"));
    }

    #[tokio::test]
    async fn simulate_error() {
        let c = SimpleRsiClient::new(Vec::<&str>::new()).with_error(true);
        let r = c.verify_handle("any");
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn rsi_register_and_verify() {
        let mut r: AdapterRegistry = AdapterRegistry::new();
        let a = std::sync::Arc::new(SimpleRsiClient::new(vec!["alice", "bob"]).with_name("simple-rsi")) as std::sync::Arc<dyn crate::adapter_api::DataAdapter>;
        r.register(a, crate::adapter_api::Schedule::Fixed(std::time::Duration::from_secs(60 * 60 * 24)));
        r.start_all(None, None).await.expect("start");
        let h: HashMap<String, AdapterHealth> = r.get_health().await;
        assert!(h.contains_key("simple-rsi"));
        let ad = r.get("simple-rsi").unwrap();
        let data = ad.fetch().await.expect("fetch");
        match data {
            AdapterData::Other(v) => { assert!(v.is_array()); }
            _ => unreachable!("unexpected variant"),
        }
    }
}
