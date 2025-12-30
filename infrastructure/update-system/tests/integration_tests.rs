use update_system::{AuthorService, ClientService, delta_chunks, create_backup};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn full_flow_minimal() {
    let dir = tempfile::tempdir().unwrap();
    let author = AuthorService::new_with_keystore(dir.path()).unwrap();
    let client = ClientService::new();

    let manifest = r#"{\"version\":\"1.0.0\"}"#;
    // use keystore-backed ed25519 flow
    let pubk = author.create_keypair("integration").unwrap();
    let sig_hex = author.sign_manifest_with_key("integration", manifest).unwrap();
    assert!(client.verify_manifest_ed25519(manifest, &sig_hex, &pubk).unwrap());

    let data = vec![0u8; 4096];
    let chunks = delta_chunks(&data, 1024);
    assert_eq!(chunks.len(), 4);

    // backup test
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "test").unwrap();
    let bak = create_backup(f.path()).unwrap();
    assert!(bak.exists());
}
