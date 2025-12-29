use axum::{routing::get, Router, extract::Extension, body::to_bytes};
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt; // for oneshot
use sc_manager_gateway::health_handler;

#[tokio::test]
async fn health_includes_inmemory_discord() {
    // Build a registry and register the in-memory discord adapter
    let mut registry = sc_manager_adapters::AdapterRegistry::new();
    registry.register(
        std::sync::Arc::new(sc_manager_adapters::discord::InMemoryDiscordAdapter::new()) as std::sync::Arc<dyn sc_manager_adapters::adapter_api::DataAdapter>,
        sc_manager_adapters::Schedule::OnDemand,
    );
    let metrics = prometheus::Registry::new();
    registry.start_all(None, Some(std::sync::Arc::new(metrics))).await.expect("start adapters");
    let registry = Arc::new(registry);

    // Build a router mimicking main() routes
    let app = Router::new()
        .route("/health", get(health_handler))
        .layer(Extension(registry));

    let req: axum::http::Request<axum::body::Body> = axum::http::Request::builder().uri("/health").body(axum::body::Body::empty()).unwrap();
    let response = app
        .oneshot(req)
        .await
        .expect("request failed");
    assert_eq!(response.status(), 200);

    let body = to_bytes(response.into_body(), 64 * 1024).await.expect("body");
    let v: Value = serde_json::from_slice(&body).expect("json");
    assert!(v.get("inmemory-discord").is_some());
}