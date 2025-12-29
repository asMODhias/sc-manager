use sc_manager_app::handlers::organization_handler::CreateOrganizationHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_core::repositories::{
    MemberRepository, OrganizationRepository, RepositoryError, RoleRepository,
};

// Simple dummy permission repo for tests
struct DummyPermissionRepo;
impl sc_manager_core::repositories::PermissionRepository for DummyPermissionRepo {
    fn create(
        &mut self,
        _permission: sc_manager_core::domain::Permission,
    ) -> Result<(), RepositoryError> {
        Ok(())
    }
    fn get(&self, _id: &str) -> Result<sc_manager_core::domain::Permission, RepositoryError> {
        Err(RepositoryError::NotFound)
    }
    fn list_all(&self) -> Result<Vec<sc_manager_core::domain::Permission>, RepositoryError> {
        Ok(vec![])
    }
}

// Simple in-memory org repo for tests
struct InMemoryOrganizationRepo {
    store: std::collections::HashMap<String, sc_manager_core::domain::Organization>,
}
impl InMemoryOrganizationRepo {
    fn new() -> Self {
        Self {
            store: std::collections::HashMap::new(),
        }
    }
}
impl sc_manager_core::repositories::OrganizationRepository for InMemoryOrganizationRepo {
    fn create(
        &mut self,
        org: sc_manager_core::domain::Organization,
    ) -> Result<(), RepositoryError> {
        if self.store.contains_key(&org.id) {
            Err(RepositoryError::AlreadyExists)
        } else {
            self.store.insert(org.id.clone(), org);
            Ok(())
        }
    }
    fn get(&self, id: &str) -> Result<sc_manager_core::domain::Organization, RepositoryError> {
        self.store.get(id).cloned().ok_or(RepositoryError::NotFound)
    }
    fn update(
        &mut self,
        org: sc_manager_core::domain::Organization,
    ) -> Result<(), RepositoryError> {
        if self.store.contains_key(&org.id) {
            self.store.insert(org.id.clone(), org);
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
    fn delete(&mut self, id: &str) -> Result<(), RepositoryError> {
        if self.store.remove(id).is_some() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}

#[test]
fn role_based_resource_permission_allows_org_creation() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    // create role with permission
    let mut r = sc_manager_core::domain::Role::new("r1", "OrgAdmin");
    r.add_permission("org.create");
    role_repo.create(r).unwrap();

    // create member and assign role globally
    member_repo
        .add(sc_manager_core::domain::Member::new("alice"))
        .unwrap();
    let mut m = member_repo.get("alice").unwrap();
    m.assign_role("r1", None);
    member_repo.update(m).unwrap();

    let mut org_repo = InMemoryOrganizationRepo::new();
    let mut handler = CreateOrganizationHandler::new(&mut org_repo);

    let cmd = sc_manager_app::commands::CreateOrganizationCommand::new("orgX", "Org X");
    let res =
        handler.handle_with_auth("alice", cmd, &member_repo, &role_repo, &DummyPermissionRepo);
    assert!(res.is_ok());
}

#[test]
fn role_without_resource_scope_denied_for_scoped_action() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();

    // create role with permission
    let mut r = sc_manager_core::domain::Role::new("r2", "ScopedAdmin");
    r.add_permission("org.update");
    role_repo.create(r).unwrap();

    // create member and assign role but scoped to orgY
    member_repo
        .add(sc_manager_core::domain::Member::new("bob"))
        .unwrap();
    let mut m = member_repo.get("bob").unwrap();
    m.assign_role("r2", Some("orgY".to_string()));
    member_repo.update(m).unwrap();

    let mut org_repo = InMemoryOrganizationRepo::new();
    org_repo
        .create(sc_manager_core::domain::Organization::new("orgX", "Org X"))
        .unwrap();
    let mut handler = CreateOrganizationHandler::new(&mut org_repo);

    // attempt to update orgX via a create (simulate unauthorized action)
    let cmd = sc_manager_app::commands::CreateOrganizationCommand::new("orgX", "Org X");
    let res = handler.handle_with_auth("bob", cmd, &member_repo, &role_repo, &DummyPermissionRepo);
    assert_eq!(res.unwrap_err(), RepositoryError::Unauthorized);
}
