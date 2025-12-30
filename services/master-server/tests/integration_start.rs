use sc_manager_master_server::MasterServer;

#[tokio::test]
async fn integration_start_master() {
    let s = MasterServer::new_with_defaults("integration-master");
    s.start().await.expect("start");
}
