use sc_manager_adapters::game_log::{SimpleGameLogParser, GameLogParser};
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::services::session_service::SessionService;
use sc_manager_core::domain::Role;
use sc_manager_core::repositories::{MemberRepository, RoleRepository};

#[test]
fn session_start_requires_permission_and_updates_activity() {
    let parser = SimpleGameLogParser;
    let mut session_repo = InMemorySessionRepo::new();
    let mut event_repo = InMemoryEventRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let mut role_repo = InMemoryRoleRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    // create member m1 and add to repos
    member_repo.add(sc_manager_core::domain::Member::new("m1")).unwrap();

    // process session start line without permissions -> should be Unauthorized
    let line = "SessionStart 1610000000 member=m1";
    if let Some(env) = parser.parse_line(line) {
        let res = SessionService::process_envelope(
            env,
            &mut session_repo,
            &mut event_repo,
            Some(&mut member_repo),
            Some(&role_repo),
            Some(&perm_repo),
        );
        assert_eq!(res.unwrap_err(), sc_manager_core::repositories::RepositoryError::Unauthorized);
    }

    // add role with session.start and assign to member
    let mut r = Role::new("r_sess", "Session Starter");
    r.add_permission("session.start");
    role_repo.create(r).unwrap();
    let mut m = member_repo.get("m1").unwrap();
    m.assign_role("r_sess", None);
    member_repo.update(m).unwrap();

    // process again - should be OK
    if let Some(env) = parser.parse_line(line) {
        SessionService::process_envelope(
            env,
            &mut session_repo,
            &mut event_repo,
            Some(&mut member_repo),
            Some(&role_repo),
            Some(&perm_repo),
        )
        .unwrap();
    }

    // check member activity updated
    let mm = member_repo.get("m1").unwrap();
    assert_eq!(mm.last_session_id.unwrap(), "sess-1610000000");
    assert_eq!(mm.last_seen.unwrap(), 1610000000);
}
