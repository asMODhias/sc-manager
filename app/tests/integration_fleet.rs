use sc_manager_app::in_memory_ship_repo::InMemoryShipRepo;
use sc_manager_core::repositories::*;

#[test]
fn integration_register_ship_and_list_by_org() {
    let mut ship_repo = InMemoryShipRepo::new();
    ship_repo
        .register(sc_manager_core::domain::Ship::new("si1", "Aurora"))
        .unwrap();
    ship_repo
        .register(sc_manager_core::domain::Ship::new("si2", "Avenger"))
        .unwrap();
    ship_repo
        .update(sc_manager_core::domain::Ship {
            id: "si1".to_string(),
            model: "Aurora".to_string(),
            owner_org: Some("orgZ".to_string()),
        })
        .unwrap();
    ship_repo
        .update(sc_manager_core::domain::Ship {
            id: "si2".to_string(),
            model: "Avenger".to_string(),
            owner_org: Some("orgZ".to_string()),
        })
        .unwrap();

    let list = ship_repo.list_by_owner_org("orgZ").unwrap();
    assert_eq!(list.len(), 2);
}
