use sc_manager_core::domain::{Permission, Role};
use sc_manager_core::repositories::{PermissionRepository, RepositoryError, RoleRepository};

pub struct RoleHandler<'a, R: RoleRepository + 'a, P: PermissionRepository + 'a> {
    pub role_repo: &'a mut R,
    pub perm_repo: &'a mut P,
}

impl<'a, R: RoleRepository, P: PermissionRepository> RoleHandler<'a, R, P> {
    pub fn new(role_repo: &'a mut R, perm_repo: &'a mut P) -> Self {
        Self {
            role_repo,
            perm_repo,
        }
    }

    pub fn create(
        &mut self,
        cmd: crate::commands::CreateRoleCommand,
    ) -> Result<(), RepositoryError> {
        let r = Role::new(cmd.id, cmd.name);
        self.role_repo.create(r)
    }

    pub fn create_permission(
        &mut self,
        cmd: crate::commands::CreatePermissionCommand,
    ) -> Result<(), RepositoryError> {
        let p = Permission::new(cmd.id, cmd.name);
        self.perm_repo.create(p)
    }

    pub fn assign_permission(
        &mut self,
        cmd: crate::commands::AssignPermissionToRoleCommand,
    ) -> Result<(), RepositoryError> {
        // ensure permission exists
        let _p = self.perm_repo.get(&cmd.permission_id)?;
        let mut role = self.role_repo.get(&cmd.role_id)?;
        role.add_permission(cmd.permission_id.clone());
        self.role_repo.update(role)
    }
}
