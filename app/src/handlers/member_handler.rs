use sc_manager_core::domain::Member;
use sc_manager_core::repositories::{MemberRepository, RepositoryError};

pub struct MemberHandler<'a, R: MemberRepository + 'a> {
    pub repo: &'a mut R,
}

impl<'a, R: MemberRepository> MemberHandler<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn add(&mut self, cmd: crate::commands::AddMemberCommand) -> Result<(), RepositoryError> {
        let mut m = Member::new(cmd.id);
        m.rsi_handle = cmd.rsi_handle;
        if let Some(org) = cmd.org_id {
            m.assign_to_org(org);
        }
        self.repo.add(m)
    }

    /// Add a member but verify the RSI handle first using a provided verifier function.
    /// The verifier is a closure that returns `Result<bool, String>`: Ok(true) = valid, Ok(false) = not found, Err(_) = client error.
    pub fn add_with_rsi<F>(
        &mut self,
        cmd: crate::commands::AddMemberCommand,
        mut verifier: F,
    ) -> Result<(), RepositoryError>
    where
        F: FnMut(&str) -> Result<bool, String>,
    {
        if let Some(ref h) = cmd.rsi_handle {
            match verifier(h) {
                Ok(true) => (),
                Ok(false) => return Err(RepositoryError::NotFound),
                Err(_) => return Err(RepositoryError::Internal),
            }
        }
        self.add(cmd)
    }

    /// Set member online/offline state and update last_seen timestamp
    pub fn set_online(
        &mut self,
        member_id: &str,
        online: bool,
        ts: i64,
    ) -> Result<(), RepositoryError> {
        let mut m = self.repo.get(member_id)?;
        m.online = online;
        m.last_seen = Some(ts);
        self.repo.update(m)
    }

    /// Set last session id for a member and update last_seen
    pub fn set_last_session(
        &mut self,
        member_id: &str,
        session_id: impl Into<String>,
        ts: i64,
    ) -> Result<(), RepositoryError> {
        let mut m = self.repo.get(member_id)?;
        m.last_session_id = Some(session_id.into());
        m.last_seen = Some(ts);
        m.online = true;
        self.repo.update(m)
    }

    /// Assign a role to a member with authorization check.
    pub fn assign_role_with_auth<
        M: sc_manager_core::repositories::MemberRepository,
        Rr: sc_manager_core::repositories::RoleRepository,
        Pp: sc_manager_core::repositories::PermissionRepository,
    >(
        &mut self,
        actor: &str,
        member_id: &str,
        role_id: &str,
        scope: Option<String>,
        member_repo: &M,
        role_repo: &Rr,
        perm_repo: &Pp,
    ) -> Result<(), RepositoryError> {
        // permission: member.assign_role, scoped by org if provided
        let resource = scope.as_deref();
        let allowed = crate::services::policy_service::PolicyService::check_permission(
            actor,
            "member.assign_role",
            resource,
            member_repo,
            role_repo,
            perm_repo,
        )?;
        println!("DEBUG assign_role_with_auth allowed={}", allowed);
        if !allowed {
            return Err(RepositoryError::Unauthorized);
        }
        let mut m = self.repo.get(member_id)?;
        m.assign_role(role_id, scope);
        self.repo.update(m)
    }

    pub fn update(
        &mut self,
        cmd: crate::commands::UpdateMemberCommand,
    ) -> Result<(), RepositoryError> {
        let mut existing = self.repo.get(&cmd.id)?;
        if let Some(h) = cmd.rsi_handle {
            existing.rsi_handle = Some(h);
        }
        if let Some(online) = cmd.online {
            existing.online = online;
        }
        if let Some(org_opt) = cmd.org_id {
            existing.org_id = org_opt;
        }
        self.repo.update(existing)
    }

    pub fn remove(
        &mut self,
        cmd: crate::commands::RemoveMemberCommand,
    ) -> Result<(), RepositoryError> {
        self.repo.remove(&cmd.id)
    }
}
