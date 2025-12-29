use sc_manager_core::domain::{Permission, Role};

#[test]
fn add_permission_adds_once() {
    let mut r = Role::new("role1", "TestRole");
    r.add_permission("perm.read");
    r.add_permission("perm.read");

    let perms = r.list_permissions();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0], "perm.read");
}

#[test]
fn remove_permission_removes_matching_entry() {
    let mut r = Role::new("role2", "RemovableRole");
    r.add_permission("perm.a");
    r.add_permission("perm.b");

    r.remove_permission("perm.a");
    let perms = r.list_permissions();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0], "perm.b");
}

#[test]
fn list_permissions_returns_cloned_list() {
    let mut r = Role::new("role3", "CloneRole");
    r.add_permission("perm.one");

    let mut listed = r.list_permissions();
    listed.push("perm.modified".to_string());

    // original shouldn't be affected
    let orig = r.list_permissions();
    assert_eq!(orig.len(), 1);
    assert_eq!(orig[0], "perm.one");
}

#[test]
fn permission_new_creates_expected_struct() {
    let p = Permission::new("perm.x", "Permission X");
    assert_eq!(p.id, "perm.x");
    assert_eq!(p.name, "Permission X");
}
