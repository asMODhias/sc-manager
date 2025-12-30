use sc_manager_core::domain::Fleet;
use sc_manager_core::repositories::{FleetRepository, RepositoryError, ShipRepository};

pub struct FleetHandler<'a, F: FleetRepository + 'a, S: ShipRepository + 'a> {
    pub fleet_repo: &'a mut F,
    pub ship_repo: &'a mut S,
}

impl<'a, F: FleetRepository, S: ShipRepository> FleetHandler<'a, F, S> {
    pub fn new(fleet_repo: &'a mut F, ship_repo: &'a mut S) -> Self {
        Self {
            fleet_repo,
            ship_repo,
        }
    }

    pub fn create(
        &mut self,
        cmd: crate::commands::CreateFleetCommand,
    ) -> Result<(), RepositoryError> {
        let f = Fleet::new(cmd.id, cmd.name);
        self.fleet_repo.create(f)
    }

    pub fn create_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        cmd: crate::commands::CreateFleetCommand,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "fleet.create",
            None,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.create(cmd)
    }

    pub fn add_ship_to_fleet(
        &mut self,
        fleet_id: &str,
        ship_id: &str,
    ) -> Result<(), RepositoryError> {
        let mut fleet = self.fleet_repo.get(fleet_id)?;
        let ship = self.ship_repo.get(ship_id)?;
        fleet.add_ship(ship.clone());
        self.fleet_repo.update(fleet)
    }

    pub fn add_ship_to_fleet_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        fleet_id: &str,
        ship_id: &str,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "fleet.update",
            Some(fleet_id),
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.add_ship_to_fleet(fleet_id, ship_id)
    }

    pub fn remove_ship_from_fleet(
        &mut self,
        fleet_id: &str,
        ship_id: &str,
    ) -> Result<(), RepositoryError> {
        let mut fleet = self.fleet_repo.get(fleet_id)?;
        fleet.remove_ship(ship_id);
        self.fleet_repo.update(fleet)
    }

    pub fn remove_ship_from_fleet_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        fleet_id: &str,
        ship_id: &str,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "fleet.update",
            Some(fleet_id),
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.remove_ship_from_fleet(fleet_id, ship_id)
    }
}
