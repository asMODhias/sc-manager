use sc_manager_core::domain::Organization;
use sc_manager_core::repositories::{OrganizationRepository, RepositoryError};

pub struct CreateOrganizationHandler<'a, R: OrganizationRepository + 'a> {
    pub repo: &'a mut R,
}

impl<'a, R: OrganizationRepository> CreateOrganizationHandler<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn handle_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        cmd: crate::commands::CreateOrganizationCommand,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        // check permission: org.create (global)
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "org.create",
            None,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        let org = Organization::new(cmd.id, cmd.name);
        self.repo.create(org)
    }

    /// Backward-compatible handle method without auth checks (used in some tests or internal flows)
    pub fn handle(
        &mut self,
        cmd: crate::commands::CreateOrganizationCommand,
    ) -> Result<(), RepositoryError> {
        let org = Organization::new(cmd.id, cmd.name);
        self.repo.create(org)
    }
}
