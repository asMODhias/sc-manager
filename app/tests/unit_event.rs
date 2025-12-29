use sc_manager_app::commands::CreateEventCommand;
use sc_manager_app::handlers::event_handler::EventHandler;
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_core::repositories::*;

#[test]
fn create_event_appends() {
    let mut repo = InMemoryEventRepo::new();
    let mut handler = EventHandler::new(&mut repo);
    let cmd = CreateEventCommand::new("e1", "Session Start", 1234567890, Some("orgA".to_string()));
    handler.create(cmd).unwrap();
    let all = repo.list_all().unwrap();
    assert_eq!(all.len(), 1);
    assert!(all[0].title.contains("Session Start"));
}
