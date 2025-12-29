//! FleetYards adapter (outbound)
//!
//!
//! Provides a small trait to fetch ship lists for an organization and a simple in-memory client used for testing.

use std::collections::HashMap;

use sc_manager_core::domain::Ship;

pub trait FleetYardClient {
    /// Fetch the ship list for the given org id.
    /// Returns Ok(Vec<Ship>) on success or Err(String) on client error.
    fn fetch_ships_for_org(&self, org_id: &str) -> Result<Vec<Ship>, String>;
}

pub struct SimpleFleetYardClient {
    store: HashMap<String, Vec<Ship>>,
    simulate_error: bool,
}

impl SimpleFleetYardClient {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            simulate_error: false,
        }
    }

    pub fn with_error(mut self, err: bool) -> Self {
        self.simulate_error = err;
        self
    }

    pub fn add_ships_for_org(&mut self, org_id: impl Into<String>, ships: Vec<Ship>) {
        self.store.insert(org_id.into(), ships);
    }
}

impl Default for SimpleFleetYardClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetYardClient for SimpleFleetYardClient {
    fn fetch_ships_for_org(&self, org_id: &str) -> Result<Vec<Ship>, String> {
        if self.simulate_error {
            return Err("simulated fleet service error".to_string());
        }
        Ok(self.store.get(org_id).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_manager_core::domain::Ship;

    #[test]
    fn fetch_empty_returns_ok() {
        let c = SimpleFleetYardClient::new();
        let r = c.fetch_ships_for_org("orgX").expect("fetch_ships_for_org failed in test");
        assert!(r.is_empty());
    }

    #[test]
    fn fetch_existing_ships() {
        let mut c = SimpleFleetYardClient::new();
        c.add_ships_for_org(
            "orgZ",
            vec![Ship::new("s1", "Aurora"), Ship::new("s2", "Avenger")],
        );
        let r = c.fetch_ships_for_org("orgZ").expect("fetch_ships_for_org failed in test");
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].id, "s1");
    }

    #[test]
    fn simulate_error() {
        let c = SimpleFleetYardClient::new().with_error(true);
        let r = c.fetch_ships_for_org("orgZ");
        assert!(r.is_err());
    }
}
