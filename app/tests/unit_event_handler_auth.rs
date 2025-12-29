use sc_manager_app::handlers::event_handler::EventHandler;
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_core::repositories::{EventRepository, MemberRepository, RoleRepository};

#[test]
fn event_create_allowed_with_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r1", "EventCreator");
    r.add_permission("event.create");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("eve"))
        .unwrap();
    let mut m = member_repo.get("eve").unwrap();
    m.assign_role("r1", None);
    member_repo.update(m).unwrap();

    let mut repo = InMemoryEventRepo::new();
    let mut handler = EventHandler::new(&mut repo);

    let cmd = sc_manager_app::commands::CreateEventCommand::new("e1", "Test", 1610001000, None);
    let res = handler.create_with_auth("eve", cmd, &member_repo, &role_repo, &perm_repo);
    assert!(res.is_ok());

    let all = repo.list_all().unwrap();
    assert_eq!(all.len(), 1);
}

#[test]
fn event_create_denied_without_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    // role without event.create
    let r = sc_manager_core::domain::Role::new("r-no", "NoEvents");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("nope"))
        .unwrap();

    let mut repo = InMemoryEventRepo::new();
    let mut handler = EventHandler::new(&mut repo);

    let cmd = sc_manager_app::commands::CreateEventCommand::new("e2", "Test2", 1610002000, None);
    let res = handler.create_with_auth("nope", cmd, &member_repo, &role_repo, &perm_repo);
    assert_eq!(
        res.unwrap_err(),
        sc_manager_core::repositories::RepositoryError::Unauthorized
    );
}
