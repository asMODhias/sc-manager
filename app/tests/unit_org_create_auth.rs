use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::handlers::organization_handler::CreateOrganizationHandler;
use sc_manager_app::commands::CreateOrganizationCommand;
use sc_manager_core::domain::Role;
use sc_manager_core::repositories::{MemberRepository, RoleRepository, RepositoryError};

#[test]
fn org_create_requires_permission() {
    let mut org_repo = sc_manager_app::in_memory_repo::InMemoryOrganizationRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let mut role_repo = InMemoryRoleRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    // create member m1 without roles
    member_repo.add(sc_manager_core::domain::Member::new("m1")).unwrap();

    let mut h = CreateOrganizationHandler::new(&mut org_repo);

    let cmd1 = CreateOrganizationCommand::new("orgX", "Org X");
    let res = h.handle_with_auth("m1", cmd1, &member_repo, &role_repo, &perm_repo);
    assert_eq!(res.unwrap_err(), RepositoryError::Unauthorized);

    // create role with permission and assign to member
    let mut r = Role::new("r1", "Org Creator");
    r.add_permission("org.create");
    role_repo.create(r).unwrap();

    // update member to have role assignment
    let mut m = member_repo.get("m1").unwrap();
    m.assign_role("r1", None);
    member_repo.update(m).unwrap();

    let cmd2 = CreateOrganizationCommand::new("orgX", "Org X");
    let res = h.handle_with_auth("m1", cmd2, &member_repo, &role_repo, &perm_repo);
    assert!(res.is_ok());
}
