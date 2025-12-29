use sc_manager_core::domain::Equipment;
use sc_manager_core::repositories::{EquipmentRepository, RepositoryError};

pub struct EquipmentHandler<'a, R: EquipmentRepository + 'a> {
    pub repo: &'a mut R,
}

impl<'a, R: EquipmentRepository> EquipmentHandler<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn register(
        &mut self,
        cmd: crate::commands::RegisterEquipmentCommand,
    ) -> Result<(), RepositoryError> {
        let eq = Equipment::new(cmd.id, cmd.name, cmd.read_only);
        self.repo.register(eq)
    }

    pub fn register_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        cmd: crate::commands::RegisterEquipmentCommand,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        // equipment may be global register permission
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "equipment.register",
            None,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.register(cmd)
    }
}
