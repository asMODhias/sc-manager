use sc_manager_app::commands::CreateFleetCommand;
use sc_manager_app::commands::RegisterShipCommand;
use sc_manager_app::handlers::fleet_handler::FleetHandler;
use sc_manager_app::handlers::ship_handler::ShipHandler;
use sc_manager_app::in_memory_fleet_repo::InMemoryFleetRepo;
use sc_manager_app::in_memory_ship_repo::InMemoryShipRepo;
use sc_manager_core::repositories::*;

#[test]
fn create_fleet_and_add_ship() {
    let mut fleet_repo = InMemoryFleetRepo::new();
    let mut ship_repo = InMemoryShipRepo::new();

    // register ship first (drop ship_handler before creating fleet_handler to avoid overlapping &mut borrows)
    {
        let mut ship_handler = ShipHandler::new(&mut ship_repo);
        ship_handler
            .register(RegisterShipCommand::new(
                "s1",
                "Constellation",
                Some("org1".to_string()),
            ))
            .unwrap();
    }

    let mut fleet_handler = FleetHandler::new(&mut fleet_repo, &mut ship_repo);
    fleet_handler
        .create(CreateFleetCommand::new("f1", "Alpha Fleet"))
        .unwrap();
    fleet_handler.add_ship_to_fleet("f1", "s1").unwrap();

    let fleet = fleet_repo.get("f1").unwrap();
    assert_eq!(fleet.ships.len(), 1);
    assert_eq!(fleet.ships[0].id, "s1");
}

#[test]
fn adding_nonexistent_ship_fails() {
    let mut fleet_repo = InMemoryFleetRepo::new();
    let mut ship_repo = InMemoryShipRepo::new();
    let mut fleet_handler = FleetHandler::new(&mut fleet_repo, &mut ship_repo);

    fleet_handler
        .create(CreateFleetCommand::new("f2", "Beta Fleet"))
        .unwrap();

    let res = fleet_handler.add_ship_to_fleet("f2", "nope");
    assert!(res.is_err());
}
