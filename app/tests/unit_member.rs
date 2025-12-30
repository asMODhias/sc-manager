use sc_manager_app::commands::{AddMemberCommand, RemoveMemberCommand, UpdateMemberCommand};
use sc_manager_app::handlers::member_handler::MemberHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_core::repositories::RepositoryError;
use sc_manager_core::repositories::*;

#[test]
fn add_member_creates_entry() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);
    let cmd = AddMemberCommand::new("m1", Some("rsi123".to_string()), Some("org1".to_string()));
    handler.add(cmd).unwrap();
    let m = repo.get("m1").unwrap();
    assert_eq!(m.id, "m1");
    assert_eq!(m.rsi_handle.unwrap(), "rsi123");
    assert_eq!(m.org_id.unwrap(), "org1");
}

#[test]
fn update_member_changes_fields() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);
    handler
        .add(AddMemberCommand::new("m2", None, None))
        .unwrap();
    let mut upd = UpdateMemberCommand::new("m2");
    upd.rsi_handle = Some("newhandle".to_string());
    upd.online = Some(true);
    handler.update(upd).unwrap();
    let m = repo.get("m2").unwrap();
    assert_eq!(m.rsi_handle.unwrap(), "newhandle");
    assert!(m.online);
}

#[test]
fn removing_nonexistent_returns_error() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);
    let res = handler.remove(RemoveMemberCommand::new("nope"));
    assert_eq!(res.unwrap_err(), RepositoryError::NotFound);
}
