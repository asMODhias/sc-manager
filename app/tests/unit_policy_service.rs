use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_core::repositories::{MemberRepository, RoleRepository};

#[test]
fn can_create_event_checks_permission() {
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

    let allowed = sc_manager_app::services::policy_service::PolicyService::can_create_event(
        "eve",
        None,
        &member_repo,
        &role_repo,
        &perm_repo,
    )
    .unwrap();
    assert!(allowed);
}

#[test]
fn can_start_session_checks_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r2", "SessionStarter");
    r.add_permission("session.start");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("sam"))
        .unwrap();
    let mut m = member_repo.get("sam").unwrap();
    m.assign_role("r2", Some("orgA".to_string()));
    member_repo.update(m).unwrap();

    // Allowed for orgA
    let allowed = sc_manager_app::services::policy_service::PolicyService::can_start_session(
        "sam",
        Some("orgA"),
        &member_repo,
        &role_repo,
        &perm_repo,
    )
    .unwrap();
    assert!(allowed);

    // Denied for other org
    let denied = sc_manager_app::services::policy_service::PolicyService::can_start_session(
        "sam",
        Some("orgB"),
        &member_repo,
        &role_repo,
        &perm_repo,
    )
    .unwrap();
    assert!(!denied);
}
