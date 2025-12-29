use sc_manager_app::commands::CreateEventCommand;
use sc_manager_app::handlers::event_handler::EventHandler;
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_core::repositories::*;

#[test]
fn integration_list_by_org() {
    let mut repo = InMemoryEventRepo::new();
    let mut handler = EventHandler::new(&mut repo);
    handler
        .create(CreateEventCommand::new(
            "e2",
            "Alpha",
            1,
            Some("orgX".to_string()),
        ))
        .unwrap();
    handler
        .create(CreateEventCommand::new(
            "e3",
            "Beta",
            2,
            Some("orgX".to_string()),
        ))
        .unwrap();
    let list = repo.list_by_org("orgX").unwrap();
    assert_eq!(list.len(), 2);
}
