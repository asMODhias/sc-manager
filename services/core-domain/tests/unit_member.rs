use sc_manager_core::domain::member::Member;

#[test]
fn member_org_and_roles() {
    let mut m = Member::new("mm1");
    m.assign_to_org("org-x");
    assert_eq!(m.org_id.as_deref(), Some("org-x"));

    m.assign_role("role-a", None);
    assert!(m.roles.iter().any(|r| r.role_id == "role-a"));

    m.revoke_role("role-a", None);
    assert!(!m.roles.iter().any(|r| r.role_id == "role-a"));
}