use sc_manager_core::domain::{member::Member, role::Role, ship::Ship};

#[test]
fn role_permissions_workflow() {
    let mut r = Role::new("r1", "admin");
    assert_eq!(r.list_permissions().len(), 0);

    r.add_permission("p1");
    r.add_permission("p2");
    assert!(r.list_permissions().contains(&"p1".to_string()));
    assert!(r.list_permissions().contains(&"p2".to_string()));

    // duplicate
    r.add_permission("p1");
    assert_eq!(r.list_permissions().iter().filter(|p| *p == "p1").count(), 1);

    r.remove_permission("p1");
    assert!(!r.list_permissions().contains(&"p1".to_string()));
}

#[test]
fn member_role_and_org() {
    let mut m = Member::new("m1");
    assert!(m.org_id.is_none());

    m.assign_to_org("org42");
    assert_eq!(m.org_id.as_deref(), Some("org42"));

    m.unassign_org();
    assert!(m.org_id.is_none());

    m.assign_role("roleA", None);
    assert_eq!(m.roles.len(), 1);

    // no duplicate
    m.assign_role("roleA", None);
    assert_eq!(m.roles.len(), 1);

    m.assign_role("roleB", Some("res1".to_string()));
    assert_eq!(m.roles.len(), 2);

    m.revoke_role("roleA", None);
    assert!(m.roles.iter().all(|r| r.role_id != "roleA"));

    m.revoke_role("roleB", Some("res1"));
    assert!(m.roles.iter().all(|r| r.role_id != "roleB"));
}

#[test]
fn ship_creation() {
    let s = Ship::new("ship1", "Aegis");
    assert_eq!(s.id, "ship1");
    assert_eq!(s.model, "Aegis");
    assert!(s.owner_org.is_none());
}