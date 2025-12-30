use sc_manager_app::commands::{
    AssignPermissionToRoleCommand, CreatePermissionCommand, CreateRoleCommand,
};
use sc_manager_app::handlers::role_handler::RoleHandler;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_core::repositories::*;

#[test]
fn integration_role_permission_flow() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut perm_repo = InMemoryPermissionRepo::new();
    let mut handler = RoleHandler::new(&mut role_repo, &mut perm_repo);

    handler
        .create(CreateRoleCommand::new("rlx", "Ops"))
        .unwrap();
    handler
        .create_permission(CreatePermissionCommand::new("plx", "Fly"))
        .unwrap();
    handler
        .assign_permission(AssignPermissionToRoleCommand::new("rlx", "plx"))
        .unwrap();

    let list = role_repo.get("rlx").unwrap().list_permissions();
    assert_eq!(list, vec!["plx".to_string()]);
}
