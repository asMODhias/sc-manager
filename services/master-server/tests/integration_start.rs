use sc_manager_master_server::MasterServer;

#[tokio::test]
async fn integration_new_master() {
    let s = MasterServer::new_with_defaults("integration-master");
    assert_eq!(s.id, "integration-master");
}
