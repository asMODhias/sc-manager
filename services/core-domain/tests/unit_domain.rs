use services_core_domain::domain::{member::Member, role::Role};

#[test]
fn role_add_remove_permissions() {
    let mut r = Role::new("r1", "admin");
    assert_eq!(r.list_permissions().len(), 0);

    r.add_permission("perm:a");
    assert_eq!(r.list_permissions(), vec!["perm:a".to_string()]);

    // adding the same permission again should not duplicate
    r.add_permission("perm:a");
    assert_eq!(r.list_permissions(), vec!["perm:a".to_string()]);

    r.add_permission("perm:b");
    assert!(r.list_permissions().contains(&"perm:b".to_string()));

    r.remove_permission("perm:a");
    assert_eq!(r.list_permissions(), vec!["perm:b".to_string()]);
}

#[test]
fn member_assign_revoke_org_and_roles() {
    let mut m = Member::new("m1");
    assert!(m.org_id.is_none());

    m.assign_to_org("org:42");
    assert_eq!(m.org_id.as_deref(), Some("org:42"));

    m.unassign_org();
    assert!(m.org_id.is_none());

    // roles
    m.assign_role("role:alpha", None);
    assert_eq!(m.roles.len(), 1);

    // duplicate assign should not create another
    m.assign_role("role:alpha", None);
    assert_eq!(m.roles.len(), 1);

    // assign with resource_id
    m.assign_role("role:beta", Some("res1".to_string()));
    assert_eq!(m.roles.len(), 2);

    // revoke role
    m.revoke_role("role:alpha", None);
    assert!(m.roles.iter().all(|r| r.role_id != "role:alpha"));

    // revoke with resource
    m.revoke_role("role:beta", Some("res1"));
    assert!(m.roles.iter().all(|r| r.role_id != "role:beta"));
}
