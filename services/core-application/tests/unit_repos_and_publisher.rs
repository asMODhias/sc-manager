use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_core::domain::Member;
use sc_manager_core::repositories::{RepositoryError, MemberRepository};

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


