mod test_utils;
use sc_manager_app::handlers::equipment_handler::EquipmentHandler;
use sc_manager_app::in_memory_equipment_repo::InMemoryEquipmentRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_core::repositories::{MemberRepository, RoleRepository};
use test_utils::DummyPermissionRepo;

#[test]
fn equipment_register_requires_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    let mut r = sc_manager_core::domain::Role::new("r1", "EqReg");
    r.add_permission("equipment.register");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("ivy"))
        .unwrap();
    let mut m = member_repo.get("ivy").unwrap();
    m.assign_role("r1", None);
    member_repo.update(m).unwrap();

    let mut repo = InMemoryEquipmentRepo::new();
    let mut handler = EquipmentHandler::new(&mut repo);

    let cmd = sc_manager_app::commands::RegisterEquipmentCommand::new("eq1", "Laser", true);
    let res =
        handler.register_with_auth("ivy", cmd, &member_repo, &role_repo, &DummyPermissionRepo);
    assert!(res.is_ok());
}

#[test]
fn equipment_register_denied_when_missing() {
    let role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    member_repo
        .add(sc_manager_core::domain::Member::new("jack"))
        .unwrap();

    let mut repo = InMemoryEquipmentRepo::new();
    let mut handler = EquipmentHandler::new(&mut repo);

    let cmd = sc_manager_app::commands::RegisterEquipmentCommand::new("eq2", "Shield", false);
    let res =
        handler.register_with_auth("jack", cmd, &member_repo, &role_repo, &DummyPermissionRepo);
    assert_eq!(
        res.unwrap_err(),
        sc_manager_core::repositories::RepositoryError::Unauthorized
    );
}
