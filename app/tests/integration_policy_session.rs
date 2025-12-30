use sc_manager_adapters::game_log::GameLogParser;
use sc_manager_adapters::game_log::SimpleGameLogParser;
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_app::services::session_service::SessionService;
use sc_manager_core::repositories::{EventRepository, MemberRepository, RoleRepository};

#[test]
fn event_with_permission_processed_updates_member_and_creates_event() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r1", "Full");
    r.add_permission("session.start");
    r.add_permission("event.create");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("eve"))
        .unwrap();
    let mut m = member_repo.get("eve").unwrap();
    m.assign_role("r1", None);
    member_repo.update(m).unwrap();

    let parser = SimpleGameLogParser;
    let mut session_repo = InMemorySessionRepo::new();
    let mut event_repo = InMemoryEventRepo::new();

    let s_line = "SessionStart 1610000000 member=\"eve\"";
    let k_line = "Kill 1610000100 member=\"eve\"";

    if let Some(env) = parser.parse_line(s_line) {
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
    if let Some(env) = parser.parse_line(k_line) {
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

    // event should be created
    let events = event_repo.list_all().unwrap();
    assert_eq!(events.len(), 1);

    // member should have updated last_seen and last_session_id
    let mm = member_repo.get("eve").unwrap();
    assert_eq!(mm.last_seen, Some(1610000100));
    assert_eq!(mm.last_session_id, Some("sess-1610000000".to_string()));
}

#[test]
fn event_denied_when_missing_event_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r2", "SessionOnly");
    r.add_permission("session.start");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("nope"))
        .unwrap();
    let mut m = member_repo.get("nope").unwrap();
    m.assign_role("r2", None);
    member_repo.update(m).unwrap();

    let parser = SimpleGameLogParser;
    let mut session_repo = InMemorySessionRepo::new();
    let mut event_repo = InMemoryEventRepo::new();

    let s_line = "SessionStart 1610000000 member=\"nope\"";
    let k_line = "Kill 1610000100 member=\"nope\"";

    if let Some(env) = parser.parse_line(s_line) {
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

    if let Some(env) = parser.parse_line(k_line) {
        let res = SessionService::process_envelope(
            env,
            &mut session_repo,
            &mut event_repo,
            Some(&mut member_repo),
            Some(&role_repo),
            Some(&perm_repo),
        );
        assert_eq!(
            res.unwrap_err(),
            sc_manager_core::repositories::RepositoryError::Unauthorized
        );
    }

    // no events appended
    let events = event_repo.list_all().unwrap();
    assert_eq!(events.len(), 0);
}
