use axum::{extract::Extension, Json};
use serde_json::Value;

pub async fn health_handler(Extension(registry): Extension<std::sync::Arc<sc_manager_adapters::AdapterRegistry>>) -> Json<Value> {
    let h = registry.get_health().await;
    let mut obj = serde_json::Map::new();
    for (name, info) in h {
        obj.insert(name, serde_json::to_value(info).unwrap_or_else(|_| serde_json::json!({}))); 
    }
    Json(Value::Object(obj))
}

use axum::response::IntoResponse;
use prometheus::{Encoder, TextEncoder};

pub async fn metrics_handler(Extension(registry): Extension<std::sync::Arc<prometheus::Registry>>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let mf = registry.gather();
    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&mf, &mut buffer) {
        tracing::error!("failed to encode metrics: {}", e);
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, [("content-type", encoder.format_type().to_string())], Vec::new());
    }
    (axum::http::StatusCode::OK, [("content-type", encoder.format_type().to_string())], buffer)
}
