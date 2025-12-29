use crate::domain::Ship;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fleet {
    pub id: String,
    pub name: String,
    pub ships: Vec<Ship>,
}

impl Fleet {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            ships: vec![],
        }
    }

    pub fn add_ship(&mut self, ship: Ship) {
        if !self.ships.iter().any(|s| s.id == ship.id) {
            self.ships.push(ship);
        }
    }

    pub fn remove_ship(&mut self, ship_id: &str) {
        self.ships.retain(|s| s.id != ship_id);
    }

    pub fn list_ships(&self) -> Vec<Ship> {
        self.ships.clone()
    }
}
