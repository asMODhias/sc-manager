use sc_manager_core::domain::{Organization, Member, Division};
use sc_manager_core::repositories::{OrganizationRepository, MemberRepository, RepositoryError};

struct InMemOrgRepo { store: std::collections::HashMap<String, Organization> }
impl InMemOrgRepo { fn new() -> Self { Self { store: std::collections::HashMap::new() } } }

impl OrganizationRepository for InMemOrgRepo {
    fn create(&mut self, org: Organization) -> Result<(), RepositoryError> {
        if self.store.contains_key(&org.id) { return Err(RepositoryError::AlreadyExists); }
        self.store.insert(org.id.clone(), org); Ok(())
    }
    fn get(&self, id: &str) -> Result<Organization, RepositoryError> { self.store.get(id).cloned().ok_or(RepositoryError::NotFound) }
    fn update(&mut self, org: Organization) -> Result<(), RepositoryError> { if !self.store.contains_key(&org.id) { return Err(RepositoryError::NotFound); } self.store.insert(org.id.clone(), org); Ok(()) }
    fn delete(&mut self, id: &str) -> Result<(), RepositoryError> { if self.store.remove(id).is_some() { Ok(()) } else { Err(RepositoryError::NotFound) } }
}

struct InMemMemberRepo { store: std::collections::HashMap<String, Member> }
impl InMemMemberRepo { fn new() -> Self { Self { store: std::collections::HashMap::new() } } }

impl MemberRepository for InMemMemberRepo {
    fn add(&mut self, member: Member) -> Result<(), RepositoryError> { if self.store.contains_key(&member.id) { return Err(RepositoryError::AlreadyExists) } self.store.insert(member.id.clone(), member); Ok(()) }
    fn get(&self, id: &str) -> Result<Member, RepositoryError> { self.store.get(id).cloned().ok_or(RepositoryError::NotFound) }
    fn update(&mut self, member: Member) -> Result<(), RepositoryError> { if !self.store.contains_key(&member.id) { return Err(RepositoryError::NotFound) } self.store.insert(member.id.clone(), member); Ok(()) }
    fn remove(&mut self, id: &str) -> Result<(), RepositoryError> { if self.store.remove(id).is_some() { Ok(()) } else { Err(RepositoryError::NotFound) } }
    fn list_by_org(&self, org_id: &str) -> Result<Vec<Member>, RepositoryError> { Ok(self.store.values().filter(|m| m.org_id.as_deref() == Some(org_id)).cloned().collect()) }
}

#[test]
fn cp1_org_crud() {
    let mut repo = InMemOrgRepo::new();
    let mut org = Organization::new("org1","Test Org");
    let d1 = Division::new("div-1","Alpha",None);
    org.add_division(d1.clone());
    repo.create(org.clone()).expect("create");
    let got = repo.get("org1").expect("get");
    assert_eq!(got.name, "Test Org");
    assert_eq!(got.list_divisions(), vec![d1]);
    let mut org2 = got.clone();
    org2.rename("New Name");
    repo.update(org2.clone()).expect("update");
    let g2 = repo.get("org1").unwrap();
    assert_eq!(g2.name, "New Name");
    repo.delete("org1").expect("delete");
    assert!(repo.get("org1").is_err());
}

#[test]
fn cp2_member_crud_and_assign_roles() {
    let mut mrepo = InMemMemberRepo::new();
    let mut member = Member::new("m1");
    member.assign_to_org("org1");
    member.assign_role("commander", None);
    mrepo.add(member.clone()).expect("add");
    let got = mrepo.get("m1").unwrap();
    assert_eq!(got.org_id, Some("org1".to_string()));
    assert!(got.roles.iter().any(|r| r.role_id == "commander"));
    let mut updated = got.clone();
    updated.assign_role("pilot", Some("ship-1".to_string()));
    mrepo.update(updated.clone()).expect("update");
    let got2 = mrepo.get("m1").unwrap();
    assert!(got2.roles.iter().any(|r| r.role_id == "pilot"));
    assert_eq!(mrepo.list_by_org("org1").unwrap().len(), 1);
    mrepo.remove("m1").expect("remove");
    assert!(mrepo.get("m1").is_err());
}
