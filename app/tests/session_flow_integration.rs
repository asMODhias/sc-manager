use sc_manager_adapters::game_log::{GameLogParser, SimpleGameLogParser};
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_app::services::session_service::SessionService;
use sc_manager_core::repositories::SessionRepository;

#[test]
fn fixture_parsing_creates_sessions_and_links_events() {
    let data = include_str!("fixtures/game_logs/session_and_kills.log");
    let parser = SimpleGameLogParser;
    let mut session_repo = InMemorySessionRepo::new();
    let mut event_repo = InMemoryEventRepo::new();

    for line in data.lines() {
        if let Some(env) = parser.parse_line(line) {
            SessionService::process_envelope(
                env,
                &mut session_repo,
                &mut event_repo,
                None::<&mut sc_manager_app::in_memory_member_repo::InMemoryMemberRepo>,
                None::<&sc_manager_app::in_memory_role_repo::InMemoryRoleRepo>,
                None::<&sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo>,
            )
            .unwrap();
        }
    }

    let sessions = session_repo.list_all().unwrap();
    assert_eq!(sessions.len(), 1);
    let s = &sessions[0];
    assert_eq!(s.start_ts, 1610000000);
    assert_eq!(s.end_ts.unwrap(), 1610003600);
    // there were 3 events between start and end
    assert_eq!(s.events.len(), 3);
}
