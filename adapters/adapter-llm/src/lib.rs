use async_trait::async_trait;
use chrono::Utc;
use sc_manager_adapters::{AdapterData, AdapterHealth, DataAdapter, HealthStatus, RateLimit};
use serde_json::json;
use std::time::Duration;

pub struct LlmAdapter;

#[async_trait]
impl DataAdapter for LlmAdapter {
    fn name(&self) -> &str { "LLM" }
    fn version(&self) -> &str { "0.1.0" }
    fn source_url(&self) -> &str { "https://llm.local" }
    fn attribution(&self) -> &str { "LLM Integration (stub)" }
    fn license(&self) -> &str { "Proprietary" }

    async fn health_check(&self) -> sc_manager_adapters::Result<AdapterHealth> {
        Ok(AdapterHealth {
            status: HealthStatus::Healthy,
            last_success: Some(Utc::now()),
            last_error: None,
            uptime_percentage: 99.9,
            requests_per_hour: 0,
        })
    }

    async fn fetch(&self) -> sc_manager_adapters::Result<AdapterData> {
        Ok(AdapterData::Other(json!({"hint":"llm stub"})))
    }

    fn rate_limit(&self) -> RateLimit {
        RateLimit { requests_per_hour: 1000, burst: 100 }
    }

    fn cache_ttl(&self) -> Duration {
        Duration::from_secs(60 * 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_ok() {
        let a = LlmAdapter;
        let res = a.fetch().await;
        assert!(res.is_ok());
    }
}
