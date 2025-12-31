use axum::{
    extract::State,
    routing::{post, get},
    Json, Router,
    http::StatusCode,
};
use std::net::SocketAddr;
use std::sync::Arc;
use serde::Deserialize;

use crate::domain::UpdateEntry;
use crate::publish;
use crate::{MasterServer, MasterError};

#[derive(Clone)]
pub struct AppState {
    pub master: Arc<MasterServer>,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    // Expect the UpdateEntry in the request body; signature included
    pub update: UpdateEntry,
}


/// Build router for master server API
pub fn router(master: Arc<MasterServer>) -> Router {
    let s = AppState { master };
    Router::new()
        .route("/api/v1/updates", post(handle_publish_update))
        .route("/api/v1/audit/events", get(handle_list_audit_events))
        .route("/api/v1/marketplace/items", get(handle_marketplace_list).post(handle_marketplace_create))
        .route("/api/v1/admin/marketplace/compact", post(handle_admin_compact))
        .route("/health", get(handle_health))
        .with_state(s)
}

/// List marketplace items
pub async fn handle_marketplace_list(State(state): State<AppState>) -> Result<Json<Vec<crate::marketplace::Item>>, (StatusCode, String)> {
    let mp = state.master.marketplace.clone();
    let mp = mp.read().await;
    let list = mp.list_items().await;
    Ok(Json(list))
}

#[derive(Deserialize)]
pub struct CreateItemPayload {
    pub id: String,
    pub owner: String,
    pub price: u64,
    pub metadata: String,
}

/// Create a marketplace item
pub async fn handle_marketplace_create(State(state): State<AppState>, Json(payload): Json<CreateItemPayload>) -> Result<StatusCode, (StatusCode, String)> {
    let item = crate::marketplace::Item { id: payload.id, owner: payload.owner, price: payload.price, metadata: payload.metadata };
    let mp = state.master.marketplace.clone();
    let mp = mp.read().await;
    match mp.insert_item(item).await {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(crate::marketplace::MarketplaceError::Exists) => Err((StatusCode::CONFLICT, "item already exists".into())),
        Err(crate::marketplace::MarketplaceError::Storage(e)) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("storage error: {}", e))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "marketplace error".into())),
    }
}

/// Health check endpoint
pub async fn handle_health() -> StatusCode {
    StatusCode::OK
}

/// List audit events from the ledger
pub async fn handle_list_audit_events(State(state): State<AppState>) -> Result<Json<Vec<crate::domain::AuditEvent>>, (StatusCode, String)> {
    let ledger = &state.master.ledger;
    match ledger.load_all() {
        Ok(events) => Ok(Json(events)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("ledger error: {}", e))),
    }
}

/// Admin endpoint: trigger marketplace compaction (requires admin token header `x-admin-token`)
pub async fn handle_admin_compact(State(state): State<AppState>, headers: axum::http::HeaderMap) -> Result<StatusCode, (StatusCode, String)> {
    // Validate admin token if configured
    let master = &state.master;
    match &master.admin_token {
        Some(expected) => {
            let provided = headers.get("x-admin-token").and_then(|v| v.to_str().ok()).unwrap_or("");
            if provided != expected {
                return Err((StatusCode::UNAUTHORIZED, "invalid admin token".into()));
            }
        }
        None => {
            return Err((StatusCode::FORBIDDEN, "admin endpoints disabled".into()));
        }
    }

    // Trigger compaction
    let mp = master.marketplace.clone();
    let mp = mp.read().await;
    match mp.compact().await {
        Ok(()) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("compaction failed: {}", e))),
    }
}


/// Handler to accept signed UpdateEntry
pub async fn handle_publish_update(
    State(state): State<AppState>,
    Json(payload): Json<UpdateEntry>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Use the master's keystore and a ledger instance (master provides)
    let master = &state.master;

    publish::publish_update(&master.ks, &master.ledger, &payload)
        .map_err(|e| match e {
            crate::publish::PublishError::InvalidSignature => (StatusCode::UNAUTHORIZED, "invalid signature".into()),
            crate::publish::PublishError::Io(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("io error: {}", err)),
            crate::publish::PublishError::Serde(err) => (StatusCode::BAD_REQUEST, format!("serialize error: {}", err)),
        })?;

    Ok(StatusCode::OK)
}

/// Run server (blocking future)
pub async fn run_server(master: Arc<MasterServer>, addr: SocketAddr) -> Result<(), MasterError> {
    let app = router(master);
    let server = axum::Server::try_bind(&addr)
        .map_err(|e| MasterError::Io(std::io::Error::other(format!("bind error: {}", e))))?
        .serve(app.into_make_service());

    // run the server until cancelled
    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("server run error: {}", e);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MasterServer;
    use tempfile::NamedTempFile;
    use base64::Engine;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_handle_publish_update_ok() {
        let tf = NamedTempFile::new().expect("tmp");
        let ledger_path = tf.path().to_path_buf();
        let ms = MasterServer::new_with_ledger("ms-test", ledger_path.clone());
        let arc = std::sync::Arc::new(ms);

        // Build a signed update
        let mut u = crate::domain::UpdateEntry::new(
            "cid-xyz".into(),
            "1.0.0".into(),
            crate::domain::ReleaseChannel::Alpha,
            "0.9.0".into(),
            String::new(),
            "http://chg".into(),
            100u64,
            crate::domain::UpdateType::Patch,
        );
        // Sign with the master's key so the handler verifies correctly
        let sig = arc.ks.sign(&u.canonical_bytes());
        let s = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());
        u.signature = s;

        // Call handler directly
        let result = handle_publish_update(State(AppState { master: arc.clone() }), Json(u)).await;
        assert!(result.is_ok());

        // Ensure an audit event was appended
        let ledger = &arc.ledger;
        let events = ledger.load_all().expect("load events");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, crate::domain::AuditEventType::UpdateSigned);

        // Test list audit events handler
        let res = handle_list_audit_events(State(AppState { master: arc.clone() })).await;
        assert!(res.is_ok());
        let Json(list) = res.unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_admin_compact_requires_token() {
        let tf = NamedTempFile::new().expect("tmp");
        let ledger_path = tf.path().to_path_buf();
        let mut ms = MasterServer::new_with_ledger("ms-admin", ledger_path.clone());
        // set a token
        ms.admin_token = Some("sekret".into());
        let arc = std::sync::Arc::new(ms);

        // No header -> should be unauthorized
        let res = handle_admin_compact(State(AppState { master: arc.clone() }), HeaderMap::new()).await;
        assert!(matches!(res, Err((StatusCode::UNAUTHORIZED, _))));

        // Wrong token
        let mut hm = HeaderMap::new();
        hm.insert("x-admin-token", "wrong".parse().unwrap());
        let res = handle_admin_compact(State(AppState { master: arc.clone() }), hm).await;
        assert!(matches!(res, Err((StatusCode::UNAUTHORIZED, _))));

        // Correct token -> OK
        let mut hm2 = HeaderMap::new();
        hm2.insert("x-admin-token", "sekret".parse().unwrap());
        let res = handle_admin_compact(State(AppState { master: arc.clone() }), hm2).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_health_ok() {
        let status = handle_health().await;
        assert_eq!(status, StatusCode::OK);
    }
}
