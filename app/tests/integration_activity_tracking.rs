use sc_manager_adapters::game_log::{GameLogParser, SimpleGameLogParser};
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_app::services::session_service::SessionService;
use sc_manager_core::repositories::MemberRepository;

#[test]
fn activity_tracking_sets_member_last_seen_and_session() {
    let data = "SessionStart 1610000000 member=m1\nKill 1610000100 killer=m1 victim=m2\nSessionEnd 1610003600";
    let parser = SimpleGameLogParser;
    let mut session_repo = InMemorySessionRepo::new();
    let mut event_repo = InMemoryEventRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    // precreate member
    member_repo
        .add(sc_manager_core::domain::Member::new("m1"))
        .unwrap();

    for line in data.lines() {
        if let Some(env) = parser.parse_line(line) {
            SessionService::process_envelope(
                env,
                &mut session_repo,
                &mut event_repo,
                Some(&mut member_repo),
                None::<&sc_manager_app::in_memory_role_repo::InMemoryRoleRepo>,
                None::<&sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo>,
            )
            .unwrap();
        }
    }

    let m = member_repo.get("m1").unwrap();
    assert_eq!(m.last_session_id.unwrap(), "sess-1610000000");
    assert_eq!(m.last_seen.unwrap(), 1610000000);
}
