use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AdapterHealth {
    pub status: HealthStatus,
    pub last_success: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub uptime_percentage: f32,
    pub requests_per_hour: u32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Down,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RateLimit {
    pub requests_per_hour: u32,
    pub burst: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AdapterData {
    Ships(Value),
    Loadouts(Value),
    Systems(Value),
    Prices(Value),
    News(Value),
    Other(Value),
}

#[async_trait]
pub trait DataAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn source_url(&self) -> &str;
    fn attribution(&self) -> &str;
    fn license(&self) -> &str;

    async fn health_check(&self) -> Result<AdapterHealth>;
    async fn fetch(&self) -> Result<AdapterData>;

    fn rate_limit(&self) -> RateLimit;
    fn cache_ttl(&self) -> Duration;

    async fn initialize(&self) -> Result<()> { Ok(()) }
    async fn shutdown(&self) -> Result<()> { Ok(()) }
}

// lightweight scheduler and health monitor with Schedule support


#[derive(Clone)]
pub enum Schedule {
    Fixed(Duration),        // Every N seconds/minutes/hours
    Cron(String),           // Cron expression (supports seconds with "with-seconds")
    OnDemand,               // Manual trigger only
}

pub struct AdapterScheduler {
    pub schedules: std::collections::HashMap<String, Schedule>,
}
impl AdapterScheduler {
    pub fn new() -> Self { AdapterScheduler { schedules: std::collections::HashMap::new() } }

    pub fn schedule(&mut self, adapter_name: String, schedule: Schedule) {
        self.schedules.insert(adapter_name, schedule);
    }
}

pub struct HealthMonitor;
impl HealthMonitor {
    pub fn new() -> Self { HealthMonitor }
    pub fn monitor(&mut self, _name: String) {}
    pub async fn start(&self) -> Result<()> { Ok(()) }
}

#[cfg(test)]
mod reg_tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::pin::Pin;
    use std::future::Future;
    use tokio::time::Duration;

    struct MockPublisher { events: Arc<Mutex<Vec<(String, Vec<u8>)>>> }
    impl MockPublisher { fn new() -> Self { Self { events: Arc::new(Mutex::new(Vec::new())) } }
    fn events(&self) -> Arc<Mutex<Vec<(String, Vec<u8>)>>> { self.events.clone() }
    }

    impl EventPublisher for MockPublisher {
        fn publish<'a>(&'a self, subject: &'a str, payload: Vec<u8>) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
            let subj = subject.to_string();
            let events = self.events.clone();
            Box::pin(async move {
                // TODO(SOT): Replace lock usage with proper error handling to avoid poisoning panics in production
                let mut guard = match events.lock() {
                    Ok(g) => g,
                    Err(e) => {
                        tracing::error!("MockPublisher publish failed: mutex poisoned: {}", e);
                        return Ok(());
                    }
                };
                guard.push((subj, payload));
                Ok(())
            })
        }
    }

    struct TestAdapter;

    #[async_trait]
    impl DataAdapter for TestAdapter {
        fn name(&self) -> &str { "test" }
        fn version(&self) -> &str { "0.1.0" }
        fn source_url(&self) -> &str { "local" }
        fn attribution(&self) -> &str { "test" }
        fn license(&self) -> &str { "MIT" }

        async fn health_check(&self) -> crate::Result<AdapterHealth> {
            Ok(AdapterHealth {
                status: HealthStatus::Healthy,
                last_success: Some(chrono::Utc::now()),
                last_error: None,
                uptime_percentage: 100.0,
                requests_per_hour: 0,
            })
        }

        async fn fetch(&self) -> crate::Result<AdapterData> {
            Ok(AdapterData::Other(serde_json::json!({"x":"y"})))
        }

        fn rate_limit(&self) -> RateLimit { RateLimit { requests_per_hour: 10, burst: 2 } }
        fn cache_ttl(&self) -> Duration { Duration::from_secs(1) }
    }

    #[tokio::test]
    async fn registry_publishes_adapter_data() {
        let mut reg = AdapterRegistry::new();
        let adapter = std::sync::Arc::new(TestAdapter);
        reg.register(adapter, Schedule::Fixed(std::time::Duration::from_secs(1)));

        let pubm = MockPublisher::new();
        let events = pubm.events();
        let arc_pubm = std::sync::Arc::new(pubm);
        reg.start_all(Some(arc_pubm), None).await.expect("start_all failed in test");

        // give the scheduler a bit of time to run one iteration
        tokio::time::sleep(Duration::from_secs(2)).await;

        let guard = match events.lock() {
            Ok(g) => g,
            Err(e) => { tracing::error!("events mutex poisoned in test: {}", e); panic!("events mutex poisoned"); }
        }; 
        assert!(!guard.is_empty(), "No events published by adapter scheduler");

        // Also test the direct fetch_and_update method
        let publisher2 = MockPublisher::new();
        let events2 = publisher2.events();
        let arc_pub2 = std::sync::Arc::new(publisher2);
        reg.fetch_and_update("test", Some(arc_pub2)).await.expect("fetch_and_update failed in test");
        let guard2 = match events2.lock() {
            Ok(g) => g,
            Err(e) => { tracing::error!("events2 mutex poisoned in test: {}", e); panic!("events2 mutex poisoned"); }
        }; 
        assert_eq!(guard2.len(), 1, "fetch_and_update did not publish expected event");
    }
}

/// EventPublisher: abstraction for publishing adapter events (used by scheduler)
pub trait EventPublisher: Send + Sync + 'static {
    fn publish<'a>(&'a self, subject: &'a str, payload: Vec<u8>) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;
}

pub struct AdapterRegistry {
    adapters: HashMap<String, Arc<dyn DataAdapter>>,
    scheduler: AdapterScheduler,
    health_monitor: HealthMonitor,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            scheduler: AdapterScheduler::new(),
            health_monitor: HealthMonitor::new(),
        }
    }

    pub fn register(&mut self, adapter: Arc<dyn DataAdapter>, schedule: Schedule) {
        let name = adapter.name().to_string();
        self.adapters.insert(name.clone(), adapter);
        self.scheduler.schedule(name.clone(), schedule);
        self.health_monitor.monitor(name);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn DataAdapter>> {
        self.adapters.get(name).cloned()
    }

    /// Fetch data from a named adapter and publish a serialized event (if publisher provided)
    pub async fn fetch_and_update(&self, name: &str, publisher: Option<std::sync::Arc<dyn EventPublisher>>) -> Result<()> {
        if let Some(adapter) = self.adapters.get(name) {
            let adapter = adapter.clone();
            match adapter.fetch().await {
                Ok(data) => {
                    if let Some(p) = publisher {
                        match serde_json::to_vec(&data) {
                            Ok(payload) => {
                                let subj = format!("adapters.{}.data", name);
                                if let Err(e) = p.publish(&subj, payload).await {
                                    tracing::error!("Failed to publish adapter {} data: {}", name, e);
                                }
                            }
                            Err(e) => tracing::error!("Failed to serialize adapter {} data: {}", name, e),
                        }
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Err(format!("Adapter not found: {}", name))
        }
    }

    /// Start all adapters and spawn scheduler tasks for those with fixed schedules.
    pub async fn start_all(&mut self, publisher: Option<std::sync::Arc<dyn EventPublisher>>, metrics_registry: Option<std::sync::Arc<prometheus::Registry>>) -> Result<()> {
        for (name, adapter) in &self.adapters {
            tracing::info!("Initializing adapter: {}", name);
            adapter.initialize().await?;
        }

        // Prepare metrics if requested
        let metric_handles = if let Some(reg) = metrics_registry.as_ref() {
            // create a small set of vectors (labeled by adapter name when used)
            match (
                prometheus::IntCounterVec::new(
                    prometheus::Opts::new("adapter_fetch_success_total", "Adapter fetch successes"),
                    &["adapter"],
                ),
                prometheus::IntCounterVec::new(
                    prometheus::Opts::new("adapter_fetch_failure_total", "Adapter fetch failures"),
                    &["adapter"],
                ),
                prometheus::IntCounterVec::new(
                    prometheus::Opts::new("adapter_publish_total", "Adapter publishes"),
                    &["adapter"],
                ),
                prometheus::HistogramVec::new(
                    prometheus::HistogramOpts::new("adapter_fetch_seconds", "Adapter fetch latency seconds"),
                    &["adapter"],
                ),
            ) {
                (Ok(fetch_success), Ok(fetch_failure), Ok(publish_total), Ok(fetch_latency)) => {
                    reg.register(Box::new(fetch_success.clone())).ok();
                    reg.register(Box::new(fetch_failure.clone())).ok();
                    reg.register(Box::new(publish_total.clone())).ok();
                    reg.register(Box::new(fetch_latency.clone())).ok();

                    Some(std::sync::Arc::new((fetch_success, fetch_failure, publish_total, fetch_latency)))
                }
                (e1, e2, e3, e4) => {
                    tracing::error!("Failed to create metrics: {:?}, {:?}, {:?}, {:?}", e1.err(), e2.err(), e3.err(), e4.err());
                    None
                }
            }
        } else { None };

        // Collect spawn tasks without borrowing `self` for the task lifetime
        let mut launches: Vec<(String, Schedule, Arc<dyn DataAdapter>)> = Vec::new();
        for (name, schedule) in &self.scheduler.schedules {
            if let Some(adapter) = self.adapters.get(name) {
                launches.push((name.clone(), schedule.clone(), adapter.clone()));
            }
        }

        for (name, schedule, adapter) in launches {
            match schedule {
                Schedule::Fixed(interval) => {
                    let adapter = adapter.clone();
                    let n = name.clone();
                    let intv = interval;
                    let pub_opt = publisher.clone();
                    let metric_counters = metric_handles.clone();
                    let _task = tokio::spawn(async move {
                        let mut ticker = tokio::time::interval(intv);
                        loop {
                            ticker.tick().await;
                            // Pass the metric handles Option through; function will handle absence gracefully
                            run_fetch_with_retries_metrics(&n, &adapter, &pub_opt, metric_counters.clone()).await;
                        }
                    });
                }
                Schedule::Cron(expr) => {
                    let adapter = adapter.clone();
                    let n = name.clone();
                    let cron_expr = expr.clone();
                    let pub_opt = publisher.clone();
                    let metric_counters = metric_handles.clone();
                    let _task = tokio::spawn(async move {
                        // Use the cron crate for full cron expressions (supports seconds when specified)
                        match std::str::FromStr::from_str(&cron_expr) {
                            Ok(schedule_parsed) => {
                                let mut upcoming = cron::Schedule::upcoming(&schedule_parsed, chrono::Utc);
                                loop {
                                    if let Some(next_dt) = upcoming.next() {
                                        let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                                        let dur = match (next_dt - now).to_std() {
                                            Ok(d) => d,
                                            Err(_) => std::time::Duration::from_secs(0),
                                        };
                                        tokio::time::sleep(dur).await;
                                        // Pass the metric handles Option through; function will handle absence gracefully
                                        run_fetch_with_retries_metrics(&n, &adapter, &pub_opt, metric_counters.clone()).await;
                                    } else {
                                        tracing::warn!("Cron schedule produced no upcoming times: {}", cron_expr);
                                        break;
                                    }
                                }
                            }
                            Err(e) => tracing::error!("Invalid cron expression for adapter {}: {}: {}", name, cron_expr, e),
                        }
                    });
                }
                Schedule::OnDemand => {}
            }
        }

        self.health_monitor.start().await?;
        Ok(())
    }

    pub async fn get_health(&self) -> HashMap<String, AdapterHealth> {
        let mut health = HashMap::new();
        for (name, adapter) in &self.adapters {
            match adapter.health_check().await {
                Ok(h) => { health.insert(name.clone(), h); }
                Err(e) => { tracing::error!("Health check failed for {}: {}", name, e); }
            }
        }
        health
    }
}

// Helper: run fetch with retries and record minimal metrics
#[allow(dead_code)]
async fn run_fetch_with_retries(name: &str, adapter: &Arc<dyn DataAdapter>, publisher: &Option<std::sync::Arc<dyn EventPublisher>>, metric_handles: Option<std::sync::Arc<(prometheus::IntCounterVec, prometheus::IntCounterVec, prometheus::IntCounterVec, prometheus::HistogramVec)>>) {
    // Pass through the Option; the inner function will handle presence/absence of metrics gracefully
    run_fetch_with_retries_metrics(name, adapter, publisher, metric_handles).await;
}

async fn run_fetch_with_retries_metrics(name: &str, adapter: &Arc<dyn DataAdapter>, publisher: &Option<std::sync::Arc<dyn EventPublisher>>, metrics_opt: Option<std::sync::Arc<(prometheus::IntCounterVec, prometheus::IntCounterVec, prometheus::IntCounterVec, prometheus::HistogramVec)>>) {
    let max_retries: u32 = std::env::var("ADAPTER_FETCH_MAX_RETRIES").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
    let base_backoff_ms: u64 = std::env::var("ADAPTER_FETCH_BACKOFF_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(500);

    // Clone out the metric tuple if available; otherwise operate without metrics
    let metrics: Option<(prometheus::IntCounterVec, prometheus::IntCounterVec, prometheus::IntCounterVec, prometheus::HistogramVec)> =
        metrics_opt.as_ref().map(|m| m.as_ref().clone());

    let mut attempt = 0u32;
    loop {
        attempt += 1;
        let start = std::time::Instant::now();
        match adapter.fetch().await {
            Ok(data) => {
                let elapsed = start.elapsed();
                if let Some((fetch_success, _fetch_failure, _publish_total, fetch_latency)) = metrics.as_ref() {
                    fetch_success.with_label_values(&[name]).inc();
                    fetch_latency.with_label_values(&[name]).observe(elapsed.as_secs_f64());
                }
                if let Some(p) = &publisher {
                    match serde_json::to_vec(&data) {
                        Ok(payload) => {
                            let subj = format!("adapters.{}.data", name);
                            if let Err(e) = p.publish(&subj, payload).await {
                                tracing::error!("Failed to publish adapter {} data: {}", name, e);
                            } else if let Some((_fetch_success, _fetch_failure, publish_total, _fetch_latency)) = metrics.as_ref() {
                                publish_total.with_label_values(&[name]).inc();
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to serialize adapter {} data: {}", name, e);
                        }
                    }
                }
                break; // success
            }
            Err(e) => {
                if let Some((_fetch_success, fetch_failure, _publish_total, _fetch_latency)) = metrics.as_ref() {
                    fetch_failure.with_label_values(&[name]).inc();
                }
                tracing::warn!("Adapter {} fetch attempt {} failed: {}", name, attempt, e);
                if attempt >= max_retries {
                    tracing::error!("Adapter {} exhausted retries ({}), giving up", name, max_retries);
                    break;
                }
                let backoff = std::time::Duration::from_millis(base_backoff_ms * (1u64 << (attempt - 1)));
                tokio::time::sleep(backoff).await;
            }
        }
    }
}
