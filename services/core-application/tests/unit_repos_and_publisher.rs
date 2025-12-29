use services_core_application::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_core::domain::Member;
use sc_manager_core::repositories::RepositoryError;

use sc_manager_core::events::{DomainEventPayload, generate_test_keypair};
use sc_manager_eventbus_nats::EventBus;
use services_core_application::services::publisher::sign_and_publish;
use serde_json::json;

// Dummy event bus that fails on publish
struct BrokenBus;
impl EventBus for BrokenBus {
    fn publish(&self, _subject: &str, _payload: &serde_json::Value) -> Result<(), String> {
        Err("bus unavailable".into())
    }
    fn subscribe(&self, _subject: &str) -> Result<Box<dyn Iterator<Item = sc_manager_eventbus_nats::Event> + Send>, String> {
        Err("not supported".into())
    }
}

#[test]
fn member_repo_crud_and_list_by_org() {
    let mut repo = InMemoryMemberRepo::new();
    let m = Member { id: "m1".into(), rsi_handle: None, online: false, org_id: Some("org1".into()), last_seen: None, last_session_id: None, roles: vec![] };
    assert!(repo.add(m.clone()).is_ok());

    let got = repo.get("m1").expect("get");
    assert_eq!(got.id, "m1");

    // update
    let mut updated = got.clone();
    updated.rsi_handle = Some("player42".into());
    assert!(repo.update(updated.clone()).is_ok());
    let after = repo.get("m1").unwrap();
    assert_eq!(after.rsi_handle.as_deref(), Some("player42"));

    // list_by_org
    let list = repo.list_by_org("org1").unwrap();
    assert_eq!(list.len(), 1);

    // remove
    assert!(repo.remove("m1").is_ok());
    assert!(matches!(repo.get("m1"), Err(RepositoryError::NotFound)));
}

#[test]
fn sign_and_publish_returns_error_on_bus_failure() {
    let bus = BrokenBus;
    let kp = generate_test_keypair();
    let ev = DomainEventPayload { id: "evt-err".into(), kind: "X".into(), payload: json!({"x":1}) };
    let res = sign_and_publish(&bus, "domain.events", &kp, &ev);
    assert!(res.is_err());
    let e = res.err().unwrap();
    assert!(e.contains("bus unavailable") || e.contains("publish"));
}
