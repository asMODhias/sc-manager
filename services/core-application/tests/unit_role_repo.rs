use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_core::domain::Role;
use sc_manager_core::repositories::{RepositoryError, RoleRepository};

#[test]
fn role_repo_crud() {
    let mut repo = InMemoryRoleRepo::new();
    let r = Role::new("role1", "Admin");
    assert!(repo.create(r.clone()).is_ok());

    let got = repo.get("role1").unwrap();
    assert_eq!(got.name, "Admin");

    let mut upd = got.clone();
    upd.name = "Admin2".into();
    assert!(repo.update(upd.clone()).is_ok());
    assert_eq!(repo.get("role1").unwrap().name, "Admin2");

    assert!(repo.delete("role1").is_ok());
    assert!(matches!(repo.get("role1"), Err(RepositoryError::NotFound)));
}
