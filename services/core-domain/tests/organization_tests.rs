use sc_manager_core::Organization;
use sc_manager_core::domain::Division;

#[test]
fn add_division_adds_once() {
    let mut org = Organization::new("o1", "Org");
    let d = Division::new("d1", "Div1", None);
    org.add_division(d.clone());
    assert_eq!(org.divisions.len(), 1);
    org.add_division(d);
    assert_eq!(org.divisions.len(), 1, "adding duplicate division should not create duplicates");
}

#[test]
fn remove_division_removes_by_id() {
    let mut org = Organization::new("o2", "Org2");
    let d1 = Division::new("d1", "Div1", None);
    let d2 = Division::new("d2", "Div2", None);
    org.add_division(d1);
    org.add_division(d2);
    assert_eq!(org.divisions.len(), 2);
    org.remove_division("d1");
    assert_eq!(org.divisions.len(), 1);
    assert_eq!(org.divisions[0].id, "d2");
}
