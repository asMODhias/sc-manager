use crate::adapter_api::{AdapterData, AdapterHealth, DataAdapter, HealthStatus, RateLimit};
use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;
use std::time::Duration;

pub struct InMemoryDiscordAdapter {
    pub name: String,
}

impl InMemoryDiscordAdapter {
    pub fn new() -> Self { Self { name: "inmemory-discord".into() } }
}

#[async_trait]
impl DataAdapter for InMemoryDiscordAdapter {
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { "0.1" }
    fn source_url(&self) -> &str { "" }
    fn attribution(&self) -> &str { "Internal Test Adapter" }
    fn license(&self) -> &str { "MIT" }

    async fn health_check(&self) -> Result<AdapterHealth, String> {
        Ok(AdapterHealth { status: HealthStatus::Healthy, last_success: Some(Utc::now()), last_error: None, uptime_percentage: 100.0, requests_per_hour: 0 })
    }

    async fn fetch(&self) -> Result<AdapterData, String> {
        // Return a simple synthetic 'news' payload representing recent slash commands handled
        Ok(AdapterData::News(json!([{"cmd":"ping","handled_by":"inmemory"}])))
    }

    fn rate_limit(&self) -> RateLimit { RateLimit { requests_per_hour: 1000, burst: 50 } }
    fn cache_ttl(&self) -> Duration { Duration::from_secs(30) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdapterRegistry;
    use std::collections::HashMap;
    use crate::adapter_api::AdapterHealth;

    #[tokio::test]
    async fn disco_register_fetch_health() {
        let mut r: AdapterRegistry = AdapterRegistry::new();
        let a = std::sync::Arc::new(InMemoryDiscordAdapter::new()) as std::sync::Arc<dyn DataAdapter>;
        r.register(a, crate::adapter_api::Schedule::Fixed(std::time::Duration::from_secs(3600)));
        r.start_all(None, None).await.expect("start");
        let h: HashMap<String, AdapterHealth> = r.get_health().await;
        assert!(h.contains_key("inmemory-discord"));
        let ad = r.get("inmemory-discord").unwrap();
        let data = ad.fetch().await.expect("fetch");
        match data {
            AdapterData::News(v) => { assert!(v.is_array()); }
            // TODO(SOT): Replace `panic!("unexpected variant")` with proper error handling/return to avoid panics in production
            _ => panic!("unexpected variant"),
        }
    }
}