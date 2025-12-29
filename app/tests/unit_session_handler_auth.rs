use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_core::repositories::{MemberRepository, RoleRepository};

#[test]
fn start_session_requires_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    // role without session.start
    let r = sc_manager_core::domain::Role::new("r-no", "NoStart");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("nope"))
        .unwrap();
    let mut m = member_repo.get("nope").unwrap();
    m.assign_role("r-no", None);
    member_repo.update(m).unwrap();

    let mut repo = InMemorySessionRepo::new();
    let mut handler = sc_manager_app::handlers::session_handler::SessionHandler::new(&mut repo);

    let res = handler.start_with_auth(
        "nope",
        "s1",
        1610000000,
        None,
        None,
        &member_repo,
        &role_repo,
        &perm_repo,
    );
    assert_eq!(
        res.unwrap_err(),
        sc_manager_core::repositories::RepositoryError::Unauthorized
    );
}

#[test]
fn start_session_allowed_when_scoped() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r-ok", "Start");
    r.add_permission("session.start");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("sam"))
        .unwrap();
    let mut m = member_repo.get("sam").unwrap();
    m.assign_role("r-ok", Some("orgA".to_string()));
    member_repo.update(m).unwrap();

    let mut repo = InMemorySessionRepo::new();
    let mut handler = sc_manager_app::handlers::session_handler::SessionHandler::new(&mut repo);

    let res = handler.start_with_auth(
        "sam",
        "s2",
        1610000000,
        Some("orgA".to_string()),
        Some("sam".to_string()),
        &member_repo,
        &role_repo,
        &perm_repo,
    );
    assert!(res.is_ok());
}
