use axum::{routing::get, Router, extract::Extension, body::to_bytes};
use std::sync::Arc;
use tower::util::ServiceExt; // for oneshot
use sc_manager_gateway::handlers;

#[tokio::test]
async fn metrics_endpoint_exposes_adapter_metrics() {
    // Build adapter registry and register a fast schedule
    let mut registry = sc_manager_adapters::AdapterRegistry::new();
    registry.register(
        std::sync::Arc::new(sc_manager_adapters::discord::InMemoryDiscordAdapter::new()) as std::sync::Arc<dyn sc_manager_adapters::adapter_api::DataAdapter>,
        sc_manager_adapters::Schedule::Fixed(std::time::Duration::from_secs(1)),
    );

    let metrics = prometheus::Registry::new();
    registry.start_all(None, Some(Arc::new(metrics.clone()))).await.expect("start adapters");
    let registry = Arc::new(registry);

    // Build router with metrics route
    let app = Router::new()
        .route("/metrics", get(handlers::metrics_handler))
        .layer(Extension(Arc::new(metrics)));

    // Wait a bit for at least one publish
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

    let req = axum::http::Request::builder().uri("/metrics").body(axum::body::Body::empty()).unwrap();
    let resp = app.oneshot(req).await.expect("request");
    assert_eq!(resp.status(), 200);
    let body = to_bytes(resp.into_body(), 64 * 1024).await.expect("body");
    let s = String::from_utf8_lossy(&body);
    assert!(s.contains("adapter_fetch_success_total"), "metrics did not include adapter metric: {}", s);
}