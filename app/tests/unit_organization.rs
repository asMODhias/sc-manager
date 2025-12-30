use sc_manager_app::commands::create_organization::CreateOrganizationCommand;
use sc_manager_app::handlers::organization_handler::CreateOrganizationHandler;
use sc_manager_app::in_memory_repo::InMemoryOrganizationRepo;
use sc_manager_core::repositories::RepositoryError;
use sc_manager_core::repositories::*;

#[test]
fn create_organization_via_handler() {
    let mut repo = InMemoryOrganizationRepo::new();
    let mut handler = CreateOrganizationHandler::new(&mut repo);
    let cmd = CreateOrganizationCommand::new("org1", "Test Org");
    let res = handler.handle(cmd);
    assert!(res.is_ok());
    let got = repo.get("org1").unwrap();
    assert_eq!(got.id, "org1");
    assert_eq!(got.name, "Test Org");
}

#[test]
fn creating_existing_returns_error() {
    let mut repo = InMemoryOrganizationRepo::new();
    let mut handler = CreateOrganizationHandler::new(&mut repo);
    let cmd = CreateOrganizationCommand::new("org2", "Org 2");
    assert!(handler.handle(cmd).is_ok());
    // second create should return AlreadyExists error
    let cmd2 = CreateOrganizationCommand::new("org2", "Org 2 repeat");
    let res = handler.handle(cmd2);
    assert_eq!(res.unwrap_err(), RepositoryError::AlreadyExists);
}
