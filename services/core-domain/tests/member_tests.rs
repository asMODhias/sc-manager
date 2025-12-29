use sc_manager_core::domain::Member;

#[test]
fn assign_role_adds_and_does_not_duplicate() {
    let mut m = Member::new("m1");
    m.assign_role("admin", None);
    assert_eq!(m.roles.len(), 1);
    m.assign_role("admin", None);
    assert_eq!(m.roles.len(), 1, "assigning the same role twice must not duplicate");
}

#[test]
fn revoke_role_removes_matching_assignment() {
    let mut m = Member::new("m2");
    m.assign_role("admin", None);
    m.assign_role("admin", Some("res1".to_string()));
    assert_eq!(m.roles.len(), 2);

    m.revoke_role("admin", None);
    assert_eq!(m.roles.len(), 1);
    assert_eq!(m.roles[0].resource_id.as_deref(), Some("res1"));
}

#[test]
fn assign_and_unassign_org_sets_org_id() {
    let mut m = Member::new("m3");
    m.assign_to_org("orgX");
    assert_eq!(m.org_id.as_deref(), Some("orgX"));
    m.unassign_org();
    assert!(m.org_id.is_none());
}
