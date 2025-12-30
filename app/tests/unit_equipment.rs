use sc_manager_app::commands::RegisterEquipmentCommand;
use sc_manager_app::handlers::equipment_handler::EquipmentHandler;
use sc_manager_app::in_memory_equipment_repo::InMemoryEquipmentRepo;
use sc_manager_core::repositories::*;

#[test]
fn register_equipment_and_get() {
    let mut repo = InMemoryEquipmentRepo::new();
    let mut handler = EquipmentHandler::new(&mut repo);
    handler
        .register(RegisterEquipmentCommand::new("eq1", "Laser Cannon", true))
        .unwrap();
    let got = repo.get("eq1").unwrap();
    assert_eq!(got.name, "Laser Cannon");
    assert!(got.read_only);
}
