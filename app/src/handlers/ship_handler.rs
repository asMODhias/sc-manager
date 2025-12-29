use sc_manager_core::domain::Ship;
use sc_manager_core::repositories::{RepositoryError, ShipRepository};

pub struct ShipHandler<'a, S: ShipRepository + 'a> {
    pub repo: &'a mut S,
}

impl<'a, S: ShipRepository> ShipHandler<'a, S> {
    pub fn new(repo: &'a mut S) -> Self {
        Self { repo }
    }

    pub fn register(
        &mut self,
        cmd: crate::commands::RegisterShipCommand,
    ) -> Result<(), RepositoryError> {
        let ship = Ship::new(cmd.id, cmd.model);
        let mut s = ship;
        s.owner_org = cmd.owner_org;
        self.repo.register(s)
    }

    pub fn register_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        cmd: crate::commands::RegisterShipCommand,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        // resource-level: if owner_org provided, check permission on that org, else global
        let resource = cmd.owner_org.as_deref();
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "ship.register",
            resource,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.register(cmd)
    }

    pub fn remove(&mut self, ship_id: &str) -> Result<(), RepositoryError> {
        self.repo.remove(ship_id)
    }

    pub fn remove_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        ship_id: &str,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "ship.remove",
            Some(ship_id),
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.remove(ship_id)
    }
}
