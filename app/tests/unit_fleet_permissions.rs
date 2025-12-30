use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::handlers::fleet_handler::FleetHandler;
use sc_manager_app::commands::CreateFleetCommand;
use sc_manager_core::domain::Role;
use sc_manager_core::repositories::{MemberRepository, RoleRepository, ShipRepository, RepositoryError};

#[test]
fn fleet_create_and_update_permissions() {
    let mut fleet_repo = sc_manager_app::in_memory_fleet_repo::InMemoryFleetRepo::new();
    let mut ship_repo = sc_manager_app::in_memory_ship_repo::InMemoryShipRepo::new();
    // register test ship before handing an immutable borrow of ship_repo to handler
    ship_repo.register(sc_manager_core::domain::Ship::new("s100", "TestShip")).unwrap();
    let mut member_repo = InMemoryMemberRepo::new();
    let mut role_repo = InMemoryRoleRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    member_repo.add(sc_manager_core::domain::Member::new("m1")).unwrap();

    let mut h = FleetHandler::new(&mut fleet_repo, &mut ship_repo);
    let cmd1 = CreateFleetCommand::new("f1", "Fleet One");

    // no permission yet
    let res = h.create_with_auth("m1", cmd1, &member_repo, &role_repo, &perm_repo);
    assert_eq!(res.unwrap_err(), RepositoryError::Unauthorized);

    // allow by creating role with fleet.create
    let mut r = Role::new("r_create", "Fleet Creator");
    r.add_permission("fleet.create");
    role_repo.create(r).unwrap();
    let mut m = member_repo.get("m1").unwrap();
    m.assign_role("r_create", None);
    member_repo.update(m).unwrap();

    let cmd2 = CreateFleetCommand::new("f1", "Fleet One");
    let res = h.create_with_auth("m1", cmd2, &member_repo, &role_repo, &perm_repo);
    assert!(res.is_ok());

    // prepare add ship permission on specific fleet
    let mut r2 = Role::new("r_update", "Fleet Updater");
    r2.add_permission("fleet.update");
    role_repo.create(r2).unwrap();

    // assign role scoped to 'f1'
    let mut m2 = member_repo.get("m1").unwrap();
    m2.assign_role("r_update", Some("f1".to_string()));
    member_repo.update(m2).unwrap();

    // adding ship should be allowed with scoped permission
    let res = h.add_ship_to_fleet_with_auth("m1", "f1", "s100", &member_repo, &role_repo, &perm_repo);
    assert!(res.is_ok());
}
