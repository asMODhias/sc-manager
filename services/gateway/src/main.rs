use axum::{routing::post, Json, Router};
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::env;
use log::info;
mod key_loader;
mod nats;



use axum::extract::Extension;
use std::sync::Arc;
use sc_manager_gateway::handlers;
use base64::Engine;

async fn events_handler(
    Extension(nats_client): Extension<Arc<nats::NatsClient>>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    use sc_manager_core::events::{DomainEventPayload, sign_event, KeyPair};
    use uuid::Uuid;

    let subj = "domain.events";

    // Load keypair (from env/file or deterministic test pair)
    let kp = match key_loader::load_gateway_keypair() {
        Ok(k) => k,
        Err(e) => return Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    // Build DomainEventPayload
    let kind = body.get("kind").and_then(|v| v.as_str()).unwrap_or("gateway.event").to_string();
    let ev = DomainEventPayload { id: Uuid::new_v4().to_string(), kind, payload: body };

    // Sign and serialize
    let signed = sign_event(&kp, &ev).map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("sign: {}", e)))?;
    let payload = serde_json::to_vec(&signed).map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("serialize signed: {}", e)))?;

    nats_client.publish(subj, &payload).await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(json!({"status":"ok"})))
}

// Key loading implementation moved to `key_loader.rs` (supports env var and file-based keys)



#[tokio::main]
async fn main() {
    // Initialize logging from RUST_LOG env var (default: info)
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).init();

    // Simple HTTP server with one endpoint
    let app = Router::new()
        .route("/events", post(events_handler))
        .route("/health", axum::routing::get(crate::handlers::health_handler));
    let bind = std::env::var("GATEWAY_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let addr: SocketAddr = match bind.parse() {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Invalid GATEWAY_BIND '{}': {}. Falling back to 0.0.0.0:8080", bind, e);
            "0.0.0.0:8080".parse().expect("default bind is a valid socket address")
        }
    };
    info!("Gateway running on {}", addr);

    // Connect to NATS once and share the client with handlers
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".into());
    let nats_client = match nats::NatsClient::connect(&nats_url).await {
        Ok(c) => std::sync::Arc::new(c),
        Err(e) => {
            eprintln!("failed to connect to nats at {}: {}", nats_url, e);
            std::process::exit(1);
        }
    };

    // Initialize adapters and register in registry (config-driven)
    let mut registry = sc_manager_adapters::AdapterRegistry::new();
    let enabled_adapters = env::var("ADAPTERS").unwrap_or_else(|_| "discord".into());

    if enabled_adapters.split(',').any(|s| s.trim().eq_ignore_ascii_case("discord")) {
        let secs: u64 = env::var("DISCORD_SCHEDULE_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(60 * 60);
        let discord_adapter = std::sync::Arc::new(sc_manager_adapters::discord::InMemoryDiscordAdapter::new()) as std::sync::Arc<dyn sc_manager_adapters::adapter_api::DataAdapter>;
        registry.register(discord_adapter, sc_manager_adapters::Schedule::Fixed(std::time::Duration::from_secs(secs)));
    }

    // Load gateway signing key for publisher
    let kp = match key_loader::load_gateway_keypair() {
        Ok(k) => k,
        Err(e) => { eprintln!("failed to load gateway keypair for publisher: {}", e); std::process::exit(1); }
    };
    let kp = std::sync::Arc::new(kp);

    // NATS publisher that signs adapter payloads as domain events
    struct NatsPublisher { client: std::sync::Arc<nats::NatsClient>, kp: std::sync::Arc<sc_manager_core::events::KeyPair> }
    impl sc_manager_adapters::EventPublisher for NatsPublisher {
        fn publish<'a>(&'a self, subject: &'a str, payload: Vec<u8>) -> std::pin::Pin<Box<dyn std::future::Future<Output = sc_manager_adapters::Result<()>> + Send + 'a>> {
            let client = self.client.clone();
            let kp = self.kp.clone();
            let subj = subject.to_string();
            Box::pin(async move {
                // Try to parse payload as JSON, fallback to base64 raw
                let val: serde_json::Value = match serde_json::from_slice(&payload) {
                    Ok(v) => v,
                    Err(_) => serde_json::json!({"raw_b64": base64::engine::general_purpose::STANDARD.encode(&payload)})
                };
                use sc_manager_core::events::{DomainEventPayload, sign_event};
                let ev = DomainEventPayload { id: uuid::Uuid::new_v4().to_string(), kind: subj, payload: val };
                let signed = sign_event(&*kp, &ev).map_err(|e| e.to_string())?;
                let b = serde_json::to_vec(&signed).map_err(|e| e.to_string())?;
                client.publish("domain.events", &b).await.map_err(|e| e.to_string())
            })
        }
    }

    // Setup metrics registry and register adapter metrics
    let metrics_registry = prometheus::Registry::new();

    let publisher = std::sync::Arc::new(NatsPublisher { client: nats_client.clone(), kp });
    registry.start_all(Some(publisher), Some(std::sync::Arc::new(metrics_registry.clone()))).await.expect("start adapters");
    let registry = std::sync::Arc::new(registry);

    // Attach the client, registry and metrics as axum Extensions
    let app = app
        .route("/metrics", axum::routing::get(crate::handlers::metrics_handler))
        .layer(Extension(nats_client))
        .layer(Extension(registry))
        .layer(Extension(std::sync::Arc::new(metrics_registry.clone())));

    // Use axum-server for binding which provides a cross-platform helper
    if let Err(e) = axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("server error: {}", e);
        std::process::exit(1);
    }
}
