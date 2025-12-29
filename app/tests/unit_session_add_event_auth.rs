use sc_manager_app::handlers::session_handler::SessionHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_core::repositories::{MemberRepository, RoleRepository, SessionRepository};

#[test]
fn add_event_allowed_with_permission() {
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

    let mut session_repo = InMemorySessionRepo::new();
    // create session directly
    session_repo
        .create(sc_manager_core::domain::Session::new(
            "sess-1", 1610000000, None, None,
        ))
        .unwrap();

    let mut handler = SessionHandler::new(&mut session_repo);

    let res = handler.add_event_to_active_session_with_auth(
        "eve",
        "evt-1",
        &member_repo,
        &role_repo,
        &perm_repo,
    );
    assert!(res.is_ok());
}

#[test]
fn add_event_denied_without_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let r = sc_manager_core::domain::Role::new("r-no", "NoEvents");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("nope"))
        .unwrap();

    let mut session_repo = InMemorySessionRepo::new();
    session_repo
        .create(sc_manager_core::domain::Session::new(
            "sess-2", 1610000000, None, None,
        ))
        .unwrap();

    let mut handler = SessionHandler::new(&mut session_repo);

    let res = handler.add_event_to_active_session_with_auth(
        "nope",
        "evt-2",
        &member_repo,
        &role_repo,
        &perm_repo,
    );
    assert_eq!(
        res.unwrap_err(),
        sc_manager_core::repositories::RepositoryError::Unauthorized
    );
}
