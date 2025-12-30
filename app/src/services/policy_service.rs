use sc_manager_core::repositories::{
    MemberRepository, PermissionRepository, RepositoryError, RoleRepository,
};

/// PolicyService performs resource-level permission checks.
pub struct PolicyService;

impl PolicyService {
    /// Checks whether `member_id` has `permission_id` on optional `resource_id`.
    /// Role assignments on member can be global (resource_id None) or resource-scoped (resource_id Some).
    pub fn check_permission<M: MemberRepository, R: RoleRepository, P: PermissionRepository>(
        member_id: &str,
        permission_id: &str,
        resource_id: Option<&str>,
        member_repo: &M,
        role_repo: &R,
        _permission_repo: &P,
    ) -> Result<bool, RepositoryError> {
        let member = member_repo
            .get(member_id)
            .map_err(|_| RepositoryError::NotFound)?;
        // For each role assignment on the member, load role and check permissions
        for ra in member.roles.iter() {
            // role may not exist
            if let Ok(role) = role_repo.get(&ra.role_id) {
                if role.permissions.iter().any(|p| p == permission_id) {
                    // resource check: either assignment is global or matches requested resource
                    let ra_res = ra.resource_id.as_deref();
                    println!("DEBUG PolicyService: member={} role={} ra_res={:?} requested_res={:?}", member_id, &ra.role_id, ra_res, resource_id);
                    if ra.resource_id.is_none()
                        || resource_id.is_none()
                        || ra.resource_id.as_deref() == resource_id
                    {
                        println!("DEBUG PolicyService: allowed by role={}", &ra.role_id);
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    /// Convenience helper: can the member create events on the optional resource?
    pub fn can_create_event<M: MemberRepository, R: RoleRepository, P: PermissionRepository>(
        member_id: &str,
        resource_id: Option<&str>,
        member_repo: &M,
        role_repo: &R,
        permission_repo: &P,
    ) -> Result<bool, RepositoryError> {
        Self::check_permission(
            member_id,
            "event.create",
            resource_id,
            member_repo,
            role_repo,
            permission_repo,
        )
    }

    /// Convenience helper: can the member start sessions for the optional resource?
    pub fn can_start_session<M: MemberRepository, R: RoleRepository, P: PermissionRepository>(
        member_id: &str,
        resource_id: Option<&str>,
        member_repo: &M,
        role_repo: &R,
        permission_repo: &P,
    ) -> Result<bool, RepositoryError> {
        Self::check_permission(
            member_id,
            "session.start",
            resource_id,
            member_repo,
            role_repo,
            permission_repo,
        )
    }

    /// Convenience helper: can the member end sessions for the optional resource?
    pub fn can_end_session<M: MemberRepository, R: RoleRepository, P: PermissionRepository>(
        member_id: &str,
        resource_id: Option<&str>,
        member_repo: &M,
        role_repo: &R,
        permission_repo: &P,
    ) -> Result<bool, RepositoryError> {
        Self::check_permission(
            member_id,
            "session.end",
            resource_id,
            member_repo,
            role_repo,
            permission_repo,
        )
    }
}
