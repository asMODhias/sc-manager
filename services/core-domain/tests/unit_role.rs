use sc_manager_core::domain::role::Role;

#[test]
fn new_role_has_permissions_empty() {
    let r = Role::new("r1", "Recon");
    assert_eq!(r.id, "r1");
    assert_eq!(r.name, "Recon");
    assert!(r.permissions.is_empty());
}

#[test]
fn add_and_remove_permissions_service() {
    let mut r = Role::new("r2", "Ops");
    r.add_permission("perm-a");
    r.add_permission("perm-a");
    assert_eq!(r.list_permissions(), vec!["perm-a".to_string()]);
    r.remove_permission("perm-a");
    assert!(r.list_permissions().is_empty());
}