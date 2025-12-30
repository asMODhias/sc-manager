use sc_manager_core::domain::member::Member;

#[test]
fn new_member_defaults() {
    let m = Member::new("m1");
    assert_eq!(m.id, "m1");
    assert!(!m.online);
    assert!(m.org_id.is_none());
    assert!(m.roles.is_empty());
}

#[test]
fn assign_and_revoke_role() {
    let mut m = Member::new("m2");
    m.assign_role("role-1", Some("res-1".to_string()));
    m.assign_role("role-1", Some("res-1".to_string())); // duplicate no-op
    assert_eq!(m.roles.len(), 1);

    m.revoke_role("role-1", Some("res-1"));
    assert!(m.roles.is_empty());
}

#[test]
fn assign_and_unassign_org() {
    let mut m = Member::new("m3");
    m.assign_to_org("org-99");
    assert_eq!(m.org_id.as_deref(), Some("org-99"));
    m.unassign_org();
    assert!(m.org_id.is_none());
}