use sc_manager_adapters::fleetyards::FleetYardClient;
use sc_manager_adapters::fleetyards::SimpleFleetYardClient;
use sc_manager_app::in_memory_ship_repo::InMemoryShipRepo;
use sc_manager_core::repositories::ShipRepository;

#[test]
fn fetch_and_register_ships_into_repo() {
    let mut client = SimpleFleetYardClient::new();
    client.add_ships_for_org(
        "orgZ",
        vec![
            sc_manager_core::domain::Ship::new("s1", "Aurora"),
            sc_manager_core::domain::Ship::new("s2", "Avenger"),
        ],
    );

    let mut repo = InMemoryShipRepo::new();

    // Fetch
    let ships = client.fetch_ships_for_org("orgZ").unwrap();
    // Register into repo, assign owner_org to fetched ships
    for mut s in ships.into_iter() {
        s.owner_org = Some("orgZ".to_string());
        repo.register(s).unwrap();
    }

    let list = repo.list_by_owner_org("orgZ").unwrap();
    assert_eq!(list.len(), 2);
}
