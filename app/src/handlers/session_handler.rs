use sc_manager_core::domain::Session;
use sc_manager_core::repositories::{RepositoryError, SessionRepository};

pub struct SessionHandler<'a, S: SessionRepository + 'a> {
    pub repo: &'a mut S,
}

impl<'a, S: SessionRepository> SessionHandler<'a, S> {
    pub fn new(repo: &'a mut S) -> Self {
        Self { repo }
    }

    pub fn start(
        &mut self,
        id: impl Into<String>,
        ts: i64,
        org_id: Option<String>,
        participant: Option<String>,
    ) -> Result<(), RepositoryError> {
        let s = Session::new(id, ts, org_id, participant);
        self.repo.create(s)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn start_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        id: impl Into<String>,
        ts: i64,
        org_id: Option<String>,
        participant: Option<String>,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        let resource = org_id.as_deref();
        let allowed = crate::services::policy_service::PolicyService::can_start_session(
            actor,
            resource,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.start(id, ts, org_id, participant)
    }

    pub fn end(&mut self, id: &str, ts: i64) -> Result<(), RepositoryError> {
        let mut s = self.repo.get(id)?;
        s.end(ts);
        self.repo.update(s)
    }

    pub fn end_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        id: &str,
        ts: i64,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        // determine resource scope from session
        let s = self.repo.get(id)?;
        let resource = s.org_id.as_deref();
        let allowed = crate::services::policy_service::PolicyService::can_end_session(
            actor,
            resource,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.end(id, ts)
    }

    pub fn add_event_to_active_session(
        &mut self,
        event_id: impl Into<String>,
    ) -> Result<(), RepositoryError> {
        // find last active session (no end_ts)
        let sessions = self.repo.list_all()?;
        if let Some(mut s) = sessions.into_iter().rev().find(|s| s.is_active()) {
            s.add_event(event_id);
            self.repo.update(s)
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    pub fn add_event_to_active_session_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        event_id: impl Into<String>,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        // check permission for event.create (resource-less for now)
        let allowed = crate::services::policy_service::PolicyService::can_create_event(
            actor,
            None,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        self.add_event_to_active_session(event_id)
    }
}
