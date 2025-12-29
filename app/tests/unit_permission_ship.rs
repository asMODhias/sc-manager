mod test_utils;
use sc_manager_app::handlers::ship_handler::ShipHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_ship_repo::InMemoryShipRepo;
use sc_manager_core::repositories::{MemberRepository, RoleRepository, ShipRepository};
use test_utils::DummyPermissionRepo;

#[test]
#[allow(unused_mut)]
fn ship_register_allowed_on_org_scope() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r1", "ShipRegistrar");
    r.add_permission("ship.register");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("greg"))
        .unwrap();
    let mut m = member_repo.get("greg").unwrap();
    m.assign_role("r1", Some("orgA".to_string()));
    member_repo.update(m).unwrap();

    let mut ship_repo = InMemoryShipRepo::new();
    let mut handler = ShipHandler::new(&mut ship_repo);

    let cmd = sc_manager_app::commands::RegisterShipCommand::new(
        "s1",
        "Aurora",
        Some("orgA".to_string()),
    );
    let res =
        handler.register_with_auth("greg", cmd, &member_repo, &role_repo, &DummyPermissionRepo);
    assert!(res.is_ok());
}

#[test]
fn ship_remove_denied_when_no_permission() {
    let role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    member_repo
        .add(sc_manager_core::domain::Member::new("hank"))
        .unwrap();

    let mut ship_repo = InMemoryShipRepo::new();
    ship_repo
        .register(sc_manager_core::domain::Ship::new("s1", "Ship S1"))
        .unwrap();
    let mut handler = ShipHandler::new(&mut ship_repo);

    let res =
        handler.remove_with_auth("hank", "s1", &member_repo, &role_repo, &DummyPermissionRepo);
    assert_eq!(
        res.unwrap_err(),
        sc_manager_core::repositories::RepositoryError::Unauthorized
    );
}
