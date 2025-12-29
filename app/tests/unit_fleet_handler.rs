use sc_manager_app::handlers::fleet_handler::FleetHandler;
use sc_manager_app::commands::CreateFleetCommand;
use sc_manager_core::domain::{Fleet, Ship};
use sc_manager_core::repositories::{FleetRepository, RepositoryError, ShipRepository};

use std::collections::HashMap;

// simple in-memory FleetRepo
struct InMemFleetRepo {
    map: HashMap<String, Fleet>,
}
impl InMemFleetRepo {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }
}
impl FleetRepository for InMemFleetRepo {
    fn create(&mut self, fleet: Fleet) -> Result<(), RepositoryError> {
        if self.map.contains_key(&fleet.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        self.map.insert(fleet.id.clone(), fleet);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Fleet, RepositoryError> {
        self.map.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn update(&mut self, fleet: Fleet) -> Result<(), RepositoryError> {
        if !self.map.contains_key(&fleet.id) {
            return Err(RepositoryError::NotFound);
        }
        self.map.insert(fleet.id.clone(), fleet);
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), RepositoryError> {
        self.map.remove(id).map(|_| ()).ok_or(RepositoryError::NotFound)
    }
}

// simple in-memory ShipRepo
#[derive(Clone)]
struct InMemShipRepo {
    map: HashMap<String, Ship>,
}
impl InMemShipRepo {
    fn new() -> Self { Self { map: HashMap::new() } }
}
impl ShipRepository for InMemShipRepo {
    fn register(&mut self, ship: Ship) -> Result<(), RepositoryError> {
        if self.map.contains_key(&ship.id) { return Err(RepositoryError::AlreadyExists); }
        self.map.insert(ship.id.clone(), ship);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Ship, RepositoryError> {
        self.map.get(id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn update(&mut self, ship: Ship) -> Result<(), RepositoryError> {
        if !self.map.contains_key(&ship.id) { return Err(RepositoryError::NotFound); }
        self.map.insert(ship.id.clone(), ship);
        Ok(())
    }

    fn remove(&mut self, id: &str) -> Result<(), RepositoryError> {
        self.map.remove(id).map(|_| ()).ok_or(RepositoryError::NotFound)
    }

    fn list_by_owner_org(&self, org_id: &str) -> Result<Vec<Ship>, RepositoryError> {
        Ok(self.map.values().cloned().filter(|s| s.owner_org.as_deref() == Some(org_id)).collect())
    }
}

#[test]
fn create_fleet_works() {
    let mut fleet_repo = InMemFleetRepo::new();
    let mut ship_repo = InMemShipRepo::new();
    let mut h = FleetHandler::new(&mut fleet_repo, &mut ship_repo);

    let cmd = CreateFleetCommand { id: "f1".to_string(), name: "First Fleet".to_string() };
    assert!(h.create(cmd).is_ok());
    // verify stored
    let f = fleet_repo.get("f1").unwrap();
    assert_eq!(f.name, "First Fleet");
}

#[test]
fn add_and_remove_ship_in_fleet() {
    let mut fleet_repo = InMemFleetRepo::new();
    let mut ship_repo = InMemShipRepo::new();

    // seed fleet and ship
    let fleet = Fleet::new("ff", "X");
    fleet_repo.create(fleet).unwrap();
    let ship = Ship::new("s1", "Model");
    ship_repo.register(ship.clone()).unwrap();

    let mut h = FleetHandler::new(&mut fleet_repo, &mut ship_repo);
    assert!(h.add_ship_to_fleet("ff", "s1").is_ok());

    // query through handler-held repo to avoid borrow conflicts
    let f = h.fleet_repo.get("ff").unwrap();
    assert!(f.list_ships().iter().any(|s| s.id == "s1"));

    assert!(h.remove_ship_from_fleet("ff", "s1").is_ok());
    let f2 = h.fleet_repo.get("ff").unwrap();
    assert!(!f2.list_ships().iter().any(|s| s.id == "s1"));
}
