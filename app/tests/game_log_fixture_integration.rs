use sc_manager_adapters::game_log::{GameLogParser, SimpleGameLogParser};
use sc_manager_app::commands::CreateEventCommand;
use sc_manager_app::handlers::event_handler::EventHandler;
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_core::events::EventEnvelope;
use sc_manager_core::repositories::EventRepository;

#[test]
fn parse_fixture_and_append_events() {
    let data = include_str!("fixtures/game_logs/session_and_kills.log");
    let parser = SimpleGameLogParser;
    let mut repo = InMemoryEventRepo::new();
    let mut handler = EventHandler::new(&mut repo);

    for line in data.lines() {
        if let Some(EventEnvelope::GameEvent {
            id,
            event_type,
            timestamp,
            details,
        }) = parser.parse_line(line)
        {
            let title = if let Some(ref d) = details {
                format!("{:?} {}", event_type, d)
            } else {
                format!("{:?}", event_type)
            };
            let cmd = CreateEventCommand::new(id.clone(), title.clone(), timestamp, None);
            handler.create(cmd).unwrap();
        }
    }

    let all = repo.list_all().unwrap();
    // we expect 5 events from the fixture
    assert_eq!(all.len(), 5);
    // Check that at least one event includes a quoted weapon name
    assert!(all
        .iter()
        .any(|e| e.title.contains("Laser Cannon") || e.title.contains("Rail Gun")));
}
