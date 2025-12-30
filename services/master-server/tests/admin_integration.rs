use axum::body::Body;
use axum::http::{Request, StatusCode};
use reqwest::Client;
use std::net::TcpListener;
use std::sync::Arc;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn integration_admin_compaction_flow() {
    // Setup master with ledger in temp file and token
    let tf = tempfile::NamedTempFile::new().expect("tmp");
    let ledger_path = tf.path().to_path_buf();
    let mut ms = sc_manager_master_server::MasterServer::new_with_ledger("ms-integ", ledger_path);
    ms.admin_token = Some("int-sekret".into());
    let arc = Arc::new(ms);

    // Build router and bind to ephemeral port
    let app = sc_manager_master_server::api::router(arc.clone());
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind tcp");
    let addr = listener.local_addr().unwrap();

    let server = axum::Server::from_tcp(listener).unwrap().serve(app.into_make_service());
    let handle = tokio::spawn(server);

    // small helper reqwest client
    let client = Client::new();
    let url = format!("http://{}:{}/api/v1/admin/marketplace/compact", addr.ip(), addr.port());

    // 1) Missing header -> unauthorized
    let res = client.post(&url).send().await.expect("req");
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // 2) Wrong header -> unauthorized
    let res = client.post(&url).header("x-admin-token", "wrong").send().await.expect("req2");
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // 3) Correct header -> ok
    let res = client.post(&url).header("x-admin-token", "int-sekret").send().await.expect("req3");
    assert_eq!(res.status(), StatusCode::OK);

    // shutdown server
    handle.abort();
    let _ = timeout(Duration::from_secs(1), handle).await;
}
