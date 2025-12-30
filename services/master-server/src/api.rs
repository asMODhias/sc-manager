use axum::{
    extract::State,
    routing::post,
    Json, Router,
    http::StatusCode,
};
use std::net::SocketAddr;
use std::sync::Arc;
use serde::Deserialize;
use base64::Engine;

use crate::domain::UpdateEntry;
use crate::publish;
use crate::{MasterServer, MasterError};
use crate::storage::AppendOnlyLedger;

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
        .with_state(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MasterServer;
    use tempfile::NamedTempFile;
    use serde_json::json;
    use axum::body::Body;
    use axum::http::{Request};

    #[tokio::test]
    async fn test_handle_publish_update_ok() {
        let tf = NamedTempFile::new().expect("tmp");
        let ledger_path = tf.path().to_path_buf();
        let ms = MasterServer::new_with_ledger("ms-test", ledger_path);
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

        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/updates")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&u).unwrap()))
            .unwrap();

        // Call handler directly
        let result = handle_publish_update(State(AppState { master: arc }), Json(u)).await;
        assert!(result.is_ok());
    }
}

/// Handler to accept signed UpdateEntry
pub async fn handle_publish_update(
    State(state): State<AppState>,
    Json(payload): Json<UpdateEntry>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Use the master's keystore and a ledger instance (master provides)
    let master = &state.master;

    publish::publish_update(&master.ks, &*master.ledger, &payload)
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
        .map_err(|e| MasterError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("bind error: {}", e))))?
        .serve(app.into_make_service());

    // run the server until cancelled
    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("server run error: {}", e);
        }
    });

    Ok(())
}
