use sc_manager_app::in_memory_fleet_repo::InMemoryFleetRepo;
use sc_manager_app::in_memory_ship_repo::InMemoryShipRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_core::domain::{Fleet, Ship, Session};
use sc_manager_core::repositories::{RepositoryError, FleetRepository, ShipRepository, SessionRepository};

#[test]
fn fleet_repo_crud() {
    let mut repo = InMemoryFleetRepo::new();
    let f = Fleet::new("f1", "F1");
    assert!(repo.create(f.clone()).is_ok());
    let got = repo.get("f1").unwrap();
    assert_eq!(got.name, "F1");

    let mut upd = got.clone();
    upd.name = "F1-upd".into();
    assert!(repo.update(upd.clone()).is_ok());
    assert_eq!(repo.get("f1").unwrap().name, "F1-upd");

    assert!(repo.delete("f1").is_ok());
    assert!(matches!(repo.get("f1"), Err(RepositoryError::NotFound)));
}

#[test]
fn ship_repo_list_by_owner() {
    let mut repo = InMemoryShipRepo::new();
    let mut s1 = Ship::new("s1", "A"); s1.owner_org = Some("org1".into());
    let s2 = Ship::new("s2", "B");
    repo.register(s1.clone()).unwrap();
    repo.register(s2.clone()).unwrap();

    let list = repo.list_by_owner_org("org1").unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "s1");
}

#[test]
fn session_repo_create_list() {
    let mut repo = InMemorySessionRepo::new();
    let s = Session::new("sess1", 0, Some("org1".into()), Some("m1".into()));
    assert!(repo.create(s.clone()).is_ok());
    let all = repo.list_all().unwrap();
    assert!(all.iter().any(|x| x.id == "sess1"));
}
