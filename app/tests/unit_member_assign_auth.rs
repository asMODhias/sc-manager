use sc_manager_app::handlers::member_handler::MemberHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_core::domain::Role;
use sc_manager_core::repositories::{MemberRepository, RoleRepository, PermissionRepository, RepositoryError};

#[test]
fn assign_role_requires_permission_and_respects_scope() {
    // use separate repos for handler and policy checks to avoid borrow conflicts
    let mut handler_repo = InMemoryMemberRepo::new();
    let mut policy_member_repo = InMemoryMemberRepo::new();
    let mut role_repo = InMemoryRoleRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    // setup members in both repos
    handler_repo.add(sc_manager_core::domain::Member::new("actor")).unwrap();
    handler_repo.add(sc_manager_core::domain::Member::new("target")).unwrap();
    policy_member_repo.add(sc_manager_core::domain::Member::new("actor")).unwrap();
    policy_member_repo.add(sc_manager_core::domain::Member::new("target")).unwrap();

    let mut handler = MemberHandler::new(&mut handler_repo);

    // ensure actor has no roles initially
    assert_eq!(policy_member_repo.get("actor").unwrap().roles.len(), 0);
    // verify PolicyService directly
    assert_eq!(sc_manager_app::services::policy_service::PolicyService::check_permission("actor", "member.assign_role", None, &policy_member_repo, &role_repo, &perm_repo).unwrap(), false);
    // without permission -> unauthorized
    let res = handler.assign_role_with_auth("actor", "target", "r1", None, &policy_member_repo, &role_repo, &perm_repo);
    println!("DEBUG initial assign res={:?}", res);
    assert_eq!(res.unwrap_err(), RepositoryError::Unauthorized);

    // create a role with member.assign_role and assign it to actor (in policy repo)
    let mut r = Role::new("r_assign", "Role Assigner");
    r.add_permission("member.assign_role");
    role_repo.create(r).unwrap();
    let mut actor_m = policy_member_repo.get("actor").unwrap();
    actor_m.assign_role("r_assign", None);
    policy_member_repo.update(actor_m).unwrap();

    // now should be allowed
    let res = handler.assign_role_with_auth("actor", "target", "r1", None, &policy_member_repo, &role_repo, &perm_repo);
    assert!(res.is_ok());

    // scoped permission: replace global assignment with a scoped role for actor
    let mut r2 = Role::new("r_scoped", "Scoped Assigner");
    r2.add_permission("member.assign_role");
    role_repo.create(r2).unwrap();
    let mut actor_m = policy_member_repo.get("actor").unwrap();
    // remove global roles and assign scoped role to actor
    actor_m.roles.clear();
    actor_m.assign_role("r_scoped", Some("orgX".to_string()));
    policy_member_repo.update(actor_m).unwrap();

    // attempt to assign role with resource orgY -> should be unauthorized
    let res = handler.assign_role_with_auth("actor", "target", "r2", Some("orgY".to_string()), &policy_member_repo, &role_repo, &perm_repo);
    println!("DEBUG scoped assign res(orgY)={:?}", res);
    assert_eq!(res.unwrap_err(), RepositoryError::Unauthorized);

    // assign with correct scope -> allowed
    let res = handler.assign_role_with_auth("actor", "target", "r2", Some("orgX".to_string()), &policy_member_repo, &role_repo, &perm_repo);
    assert!(res.is_ok());
}