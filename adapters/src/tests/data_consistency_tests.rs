use crate::fleetyards::SimpleFleetYardClient;
use crate::erkul::SimpleErkulClient;

#[test]
fn fleetyards_returns_normalized_ships() {
    let mut client = SimpleFleetYardClient::new();
    client.add_ships_for_org("orgZ", vec![sc_manager_core::domain::Ship::new("s1", "Aurora")]);
    let ships = client.fetch_ships_for_org("orgZ").unwrap();
    assert!(!ships.is_empty());
    let s = &ships[0];
    assert!(!s.id.is_empty());
    assert!(!s.model.is_empty());
}

#[test]
fn erkul_returns_normalized_equipment() {
    let mut client = SimpleErkulClient::new();
    client.add_equipment_for_org("orgA", vec![sc_manager_core::domain::Equipment::new("eq1", "Laser", true)]);
    let items = client.fetch_equipment(Some("orgA"), None).unwrap();
    assert!(!items.is_empty());
    let it = &items[0];
    assert!(!it.id.is_empty());
    assert!(!it.name.is_empty());
}
