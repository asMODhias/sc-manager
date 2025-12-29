use sc_manager_adapters::game_log::GameLogParser;
use sc_manager_adapters::game_log::SimpleGameLogParser;
use sc_manager_app::commands::CreateEventCommand;
use sc_manager_app::handlers::event_handler::EventHandler;
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_core::events::EventEnvelope;
use sc_manager_core::repositories::*;

#[test]
fn parser_appends_event_to_repo() {
    let parser = SimpleGameLogParser;
    let mut repo = InMemoryEventRepo::new();
    let mut handler = EventHandler::new(&mut repo);

    let line = "SessionStart 1610000000";
    if let Some(EventEnvelope::GameEvent {
        id,
        event_type,
        timestamp,
        details,
    }) = parser.parse_line(line)
    {
        // Map to domain Event for append; include parsed details when present
        let title = if let Some(ref d) = details {
            format!("{:?} {}", event_type, d)
        } else {
            format!("{:?}", event_type)
        };
        let cmd = CreateEventCommand::new(id.clone(), title.clone(), timestamp, None);
        handler.create(cmd).unwrap();
    } else {
        panic!("Parser did not return a GameEvent");
    }

    let all = repo.list_all().unwrap();
    assert_eq!(all.len(), 1);
    assert!(all[0].title.contains("SessionStart") || all[0].title.contains("SessionStart"));
}
