use sc_manager_app::commands::AddMemberCommand;
use sc_manager_app::handlers::member_handler::MemberHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_core::repositories::*;

#[test]
fn integration_add_and_list_by_org() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);
    handler
        .add(AddMemberCommand::new("i1", None, Some("orgX".to_string())))
        .unwrap();
    handler
        .add(AddMemberCommand::new("i2", None, Some("orgX".to_string())))
        .unwrap();
    let list = repo.list_by_org("orgX").unwrap();
    assert_eq!(list.len(), 2);
}
