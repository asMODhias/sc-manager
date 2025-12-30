mod test_utils;
use sc_manager_app::handlers::fleet_handler::FleetHandler;
use sc_manager_app::in_memory_fleet_repo::InMemoryFleetRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_ship_repo::InMemoryShipRepo;
use sc_manager_core::repositories::{
    FleetRepository, MemberRepository, RepositoryError, RoleRepository,
};
use test_utils::DummyPermissionRepo;

#[test]
fn fleet_create_allowed_for_global_role() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let mut r = sc_manager_core::domain::Role::new("r1", "FleetAdmin");
    r.add_permission("fleet.create");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("eve"))
        .unwrap();
    let mut m = member_repo.get("eve").unwrap();
    m.assign_role("r1", None);
    member_repo.update(m).unwrap();

    let mut fleet_repo = InMemoryFleetRepo::new();
    let mut ship_repo = InMemoryShipRepo::new();
    let mut handler = FleetHandler::new(&mut fleet_repo, &mut ship_repo);

    let cmd = sc_manager_app::commands::CreateFleetCommand::new("f1", "Alpha");
    let res = handler.create_with_auth("eve", cmd, &member_repo, &role_repo, &DummyPermissionRepo);
    assert!(res.is_ok());
}

#[test]
fn fleet_update_denied_when_not_scoped() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r2", "FleetScoped");
    r.add_permission("fleet.update");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("frank"))
        .unwrap();
    let mut m = member_repo.get("frank").unwrap();
    m.assign_role("r2", Some("fleetY".to_string()));
    member_repo.update(m).unwrap();

    let mut fleet_repo = InMemoryFleetRepo::new();
    let mut ship_repo = InMemoryShipRepo::new();
    // create fleet f1
    fleet_repo
        .create(sc_manager_core::domain::Fleet::new("f1", "Fleet 1"))
        .unwrap();
    let mut handler = FleetHandler::new(&mut fleet_repo, &mut ship_repo);

    let res = handler.add_ship_to_fleet_with_auth(
        "frank",
        "f1",
        "s1",
        &member_repo,
        &role_repo,
        &DummyPermissionRepo,
    );
    assert_eq!(res.unwrap_err(), RepositoryError::Unauthorized);
}
