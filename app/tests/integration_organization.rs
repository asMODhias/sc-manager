use sc_manager_app::commands::create_organization::CreateOrganizationCommand;
use sc_manager_app::handlers::organization_handler::CreateOrganizationHandler;
use sc_manager_app::in_memory_repo::InMemoryOrganizationRepo;
use sc_manager_core::repositories::*;

#[test]
fn integration_create_and_get() {
    let mut repo = InMemoryOrganizationRepo::new();
    let mut handler = CreateOrganizationHandler::new(&mut repo);

    let cmd = CreateOrganizationCommand::new("int_org", "Integration Org");
    handler.handle(cmd).expect("create should work");

    let org = repo.get("int_org").expect("should find org");
    assert_eq!(org.name, "Integration Org");
}
