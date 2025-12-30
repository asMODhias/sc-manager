use sc_manager_master_server::domain::UpdateEntry;
use sc_manager_master_server::domain::{ReleaseChannel, UpdateType};
use tempfile::NamedTempFile;
use base64::Engine;

#[tokio::test]
async fn test_publish_signed_update_direct_call() {
    let tf = NamedTempFile::new().expect("tmp");
    let path = tf.path().to_path_buf();

    // Prepare UpdateEntry
    let mut u = UpdateEntry::new(
        "cid-integ-1".into(),
        "3.0.0".into(),
        ReleaseChannel::Alpha,
        "2.0.0".into(),
        String::new(),
        "http://chg".into(),
        4096u64,
        UpdateType::Major,
    );

    // call publish pipeline directly after signing with server key
    let ms = sc_manager_master_server::MasterServer::new_with_ledger("local-call", path.clone());

    let sig = ms.ks.sign(&u.canonical_bytes());
    let s = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());
    u.signature = s;

    let res = sc_manager_master_server::publish::publish_update(&ms.ks, &ms.ledger, &u);
    assert!(res.is_ok());
}
