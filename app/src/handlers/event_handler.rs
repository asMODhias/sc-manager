use sc_manager_core::domain::Event;
use sc_manager_core::repositories::{EventRepository, RepositoryError};

pub struct EventHandler<'a, R: EventRepository + 'a> {
    pub repo: &'a mut R,
}

impl<'a, R: EventRepository> EventHandler<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn create(
        &mut self,
        cmd: crate::commands::CreateEventCommand,
    ) -> Result<(), RepositoryError> {
        let mut e = Event::new(cmd.id, cmd.title, cmd.timestamp);
        // optional org association stored in Event if provided
        if let Some(org) = cmd.org_id {
            // For now, store in title as simple placeholder or extend Event later
            e.title = format!("{} (org={})", e.title, org);
        }
        self.repo.append(e)
    }

    pub fn create_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        cmd: crate::commands::CreateEventCommand,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        let resource = cmd.org_id.as_deref();
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "event.create",
            resource,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.create(cmd)
    }
}
