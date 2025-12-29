use sc_manager_core::domain::{Fleet, Ship};

#[test]
fn add_ship_adds_and_prevents_duplicates() {
    let mut fleet = Fleet::new("fleet-1", "Alpha");

    let s1 = Ship::new("ship-1", "Aurora");
    fleet.add_ship(s1.clone());
    assert_eq!(fleet.ships.len(), 1);

    // add duplicate id - should not increase length
    fleet.add_ship(s1);
    assert_eq!(fleet.ships.len(), 1);
}

#[test]
fn remove_ship_removes_by_id() {
    let mut fleet = Fleet::new("fleet-2", "Beta");

    let s1 = Ship::new("ship-a", "Constellation");
    let s2 = Ship::new("ship-b", "Idris");
    fleet.add_ship(s1.clone());
    fleet.add_ship(s2.clone());
    assert_eq!(fleet.ships.len(), 2);

    fleet.remove_ship("ship-a");
    assert_eq!(fleet.ships.len(), 1);
    assert_eq!(fleet.ships[0], s2);
}

#[test]
fn list_ships_returns_cloned_list() {
    let mut fleet = Fleet::new("fleet-3", "Gamma");
    let s1 = Ship::new("ship-x", "Hammerhead");
    fleet.add_ship(s1.clone());

    let list = fleet.list_ships();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0], s1);

    // mutate original fleet - cloned list should remain unchanged
    fleet.remove_ship("ship-x");
    assert_eq!(fleet.ships.len(), 0);
    assert_eq!(list.len(), 1);
}
