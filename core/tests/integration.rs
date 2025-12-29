use sc_manager_core::Organization;

#[test]
fn integration_create_and_rename_org() {
    let mut org = Organization::new("int1", "Integration Org");
    assert_eq!(org.id, "int1");
    org.rename("Integration Org Renamed");
    assert_eq!(org.name, "Integration Org Renamed");
}
