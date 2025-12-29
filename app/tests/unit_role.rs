use sc_manager_app::commands::{
    AssignPermissionToRoleCommand, CreatePermissionCommand, CreateRoleCommand,
};
use sc_manager_app::handlers::role_handler::RoleHandler;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_core::repositories::*;

#[test]
fn create_role_and_assign_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut perm_repo = InMemoryPermissionRepo::new();
    let mut handler = RoleHandler::new(&mut role_repo, &mut perm_repo);

    handler
        .create(CreateRoleCommand::new("r1", "Admin"))
        .unwrap();
    handler
        .create_permission(CreatePermissionCommand::new("p1", "Manage Orga"))
        .unwrap();

    handler
        .assign_permission(AssignPermissionToRoleCommand::new("r1", "p1"))
        .unwrap();

    let r = role_repo.get("r1").unwrap();
    assert_eq!(r.permissions.len(), 1);
    assert_eq!(r.permissions[0], "p1");
}

#[test]
fn assigning_unknown_permission_returns_error() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut perm_repo = InMemoryPermissionRepo::new();
    let mut handler = RoleHandler::new(&mut role_repo, &mut perm_repo);

    handler
        .create(CreateRoleCommand::new("r2", "Member"))
        .unwrap();

    let res = handler.assign_permission(AssignPermissionToRoleCommand::new("r2", "missing"));
    assert!(res.is_err());
}
