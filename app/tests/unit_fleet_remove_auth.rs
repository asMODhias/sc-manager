use sc_manager_app::handlers::fleet_handler::FleetHandler;
use sc_manager_app::in_memory_fleet_repo::InMemoryFleetRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_ship_repo::InMemoryShipRepo;
use sc_manager_core::repositories::{
    FleetRepository, MemberRepository, RoleRepository, ShipRepository,
};

#[test]
fn fleet_remove_ship_allowed_when_scoped() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r1", "FleetAdmin");
    r.add_permission("fleet.update");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("frank"))
        .unwrap();
    let mut m = member_repo.get("frank").unwrap();
    m.assign_role("r1", Some("f1".to_string()));
    member_repo.update(m).unwrap();

    let mut fleet_repo = InMemoryFleetRepo::new();
    fleet_repo
        .create(sc_manager_core::domain::Fleet::new("f1", "Fleet 1"))
        .unwrap();

    let mut ship_repo = InMemoryShipRepo::new();
    ship_repo
        .register(sc_manager_core::domain::Ship::new("s1", "Ship S1"))
        .unwrap();

    let mut handler = FleetHandler::new(&mut fleet_repo, &mut ship_repo);
    // add ship first
    handler.add_ship_to_fleet("f1", "s1").unwrap();
    // now remove with auth
    let res = handler.remove_ship_from_fleet_with_auth(
        "frank",
        "f1",
        "s1",
        &member_repo,
        &role_repo,
        &perm_repo,
    );
    assert!(res.is_ok());
}

#[test]
fn fleet_remove_ship_denied_when_not_scoped() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r2", "FleetScoped");
    r.add_permission("fleet.update");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("tom"))
        .unwrap();
    let mut m = member_repo.get("tom").unwrap();
    m.assign_role("r2", Some("other_fleet".to_string()));
    member_repo.update(m).unwrap();

    let mut fleet_repo = InMemoryFleetRepo::new();
    fleet_repo
        .create(sc_manager_core::domain::Fleet::new("f2", "Fleet 2"))
        .unwrap();

    let mut ship_repo = InMemoryShipRepo::new();
    ship_repo
        .register(sc_manager_core::domain::Ship::new("s2", "Ship S2"))
        .unwrap();

    let mut handler = FleetHandler::new(&mut fleet_repo, &mut ship_repo);
    handler.add_ship_to_fleet("f2", "s2").unwrap();

    let res = handler.remove_ship_from_fleet_with_auth(
        "tom",
        "f2",
        "s2",
        &member_repo,
        &role_repo,
        &perm_repo,
    );
    assert_eq!(
        res.unwrap_err(),
        sc_manager_core::repositories::RepositoryError::Unauthorized
    );
}
