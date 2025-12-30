//! Erkul adapter (outbound)
//!
//! Provides an interface for fetching equipment lists from an Erkul-like service.

use std::collections::HashMap;

use sc_manager_core::domain::Equipment;

pub trait ErkulClient {
    /// Fetch equipment catalog or equipment for a specific org or ship.
    /// Returns Ok(Vec<Equipment>) on success or Err(String) on client error.
    fn fetch_equipment(
        &self,
        org_id: Option<&str>,
        ship_id: Option<&str>,
    ) -> Result<Vec<Equipment>, String>;
}

pub struct SimpleErkulClient {
    store_by_org: HashMap<String, Vec<Equipment>>,
    store_by_ship: HashMap<String, Vec<Equipment>>,
    simulate_error: bool,
}

impl SimpleErkulClient {
    pub fn new() -> Self {
        Self {
            store_by_org: HashMap::new(),
            store_by_ship: HashMap::new(),
            simulate_error: false,
        }
    }

    pub fn with_error(mut self, err: bool) -> Self {
        self.simulate_error = err;
        self
    }

    pub fn add_equipment_for_org(&mut self, org_id: impl Into<String>, items: Vec<Equipment>) {
        self.store_by_org.insert(org_id.into(), items);
    }

    pub fn add_equipment_for_ship(&mut self, ship_id: impl Into<String>, items: Vec<Equipment>) {
        self.store_by_ship.insert(ship_id.into(), items);
    }
}

impl Default for SimpleErkulClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ErkulClient for SimpleErkulClient {
    fn fetch_equipment(
        &self,
        org_id: Option<&str>,
        ship_id: Option<&str>,
    ) -> Result<Vec<Equipment>, String> {
        if self.simulate_error {
            return Err("simulated erkul service error".to_string());
        }
        if let Some(sid) = ship_id {
            Ok(self
                .store_by_ship
                .get(sid)
                .cloned()
                .unwrap_or_default())
        } else if let Some(org) = org_id {
            Ok(self
                .store_by_org
                .get(org)
                .cloned()
                .unwrap_or_default())
        } else {
            // return all equipment across orgs and ships
            let mut all: Vec<Equipment> = self
                .store_by_org
                .values()
                .flat_map(|v| v.clone().into_iter())
                .collect();
            all.extend(
                self.store_by_ship
                    .values()
                    .flat_map(|v| v.clone().into_iter()),
            );
            Ok(all)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_manager_core::domain::Equipment;

    #[test]
    fn fetch_empty_returns_ok() {
        let c = SimpleErkulClient::new();
        let r = c.fetch_equipment(Some("orgX"), None).expect("fetch_equipment returned Ok");
        assert!(r.is_empty());
    }

    #[test]
    fn fetch_existing_equipment_by_org() {
        let mut c = SimpleErkulClient::new();
        c.add_equipment_for_org(
            "orgA",
            vec![
                Equipment::new("eq1", "Laser", true),
                Equipment::new("eq2", "Shield", false),
            ],
        );
        let r = c.fetch_equipment(Some("orgA"), None).expect("fetch_equipment returned Ok");
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].id, "eq1");
    }

    #[test]
    fn fetch_existing_equipment_by_ship() {
        let mut c = SimpleErkulClient::new();
        c.add_equipment_for_ship("si1", vec![Equipment::new("eq3", "Engine", true)]);
        let r = c.fetch_equipment(None, Some("si1")).expect("fetch_equipment returned Ok");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "eq3");
    }

    #[test]
    fn fetch_all_catalog() {
        let mut c = SimpleErkulClient::new();
        c.add_equipment_for_org("orgA", vec![Equipment::new("eq1", "Laser", true)]);
        c.add_equipment_for_ship("si1", vec![Equipment::new("eq2", "Shield", false)]);
        let r = c.fetch_equipment(None, None).expect("fetch_equipment returned Ok");
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn simulate_error() {
        let c = SimpleErkulClient::new().with_error(true);
        let r = c.fetch_equipment(Some("orgZ"), None);
        assert!(r.is_err());
    }
}
