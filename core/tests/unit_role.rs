use sc_manager_core::domain::role::Role;

#[test]
fn new_role_has_no_permissions() {
    let r = Role::new("r1", "Recon");
    assert_eq!(r.id, "r1");
    assert_eq!(r.name, "Recon");
    assert!(r.permissions.is_empty());
}

#[test]
fn add_and_remove_permissions() {
    let mut r = Role::new("r2", "Ops");
    r.add_permission("perm-a");
    r.add_permission("perm-a"); // idempotent
    assert_eq!(r.list_permissions(), vec!["perm-a".to_string()]);

    r.add_permission("perm-b");
    assert_eq!(r.list_permissions(), vec!["perm-a".to_string(), "perm-b".to_string()]);

    r.remove_permission("perm-a");
    assert_eq!(r.list_permissions(), vec!["perm-b".to_string()]);

    r.remove_permission("perm-b");
    assert!(r.list_permissions().is_empty());
}