use crate::domain::{Member, Organization};

/// Simple repository error type for core-level repository operations.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RepositoryError {
    #[error("not found")]
    NotFound,
    #[error("already exists")]
    AlreadyExists,
    #[error("unauthorized")]
    Unauthorized,
    #[error("internal")]
    Internal,
}

/// Repository trait for Organization CRUD operations.
pub trait OrganizationRepository {
    fn create(&mut self, org: Organization) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<Organization, RepositoryError>;
    fn update(&mut self, org: Organization) -> Result<(), RepositoryError>;
    fn delete(&mut self, id: &str) -> Result<(), RepositoryError>;
}

/// Repository trait for Member CRUD operations.
pub trait MemberRepository {
    fn add(&mut self, member: Member) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<Member, RepositoryError>;
    fn update(&mut self, member: Member) -> Result<(), RepositoryError>;
    fn remove(&mut self, id: &str) -> Result<(), RepositoryError>;
    fn list_by_org(&self, org_id: &str) -> Result<Vec<Member>, RepositoryError>;
}

/// Repository trait for Fleet CRUD operations.
pub trait FleetRepository {
    fn create(&mut self, fleet: crate::domain::Fleet) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<crate::domain::Fleet, RepositoryError>;
    fn update(&mut self, fleet: crate::domain::Fleet) -> Result<(), RepositoryError>;
    fn delete(&mut self, id: &str) -> Result<(), RepositoryError>;
}

/// Repository trait for Ship operations (registration + lookup).
pub trait ShipRepository {
    fn register(&mut self, ship: crate::domain::Ship) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<crate::domain::Ship, RepositoryError>;
    fn update(&mut self, ship: crate::domain::Ship) -> Result<(), RepositoryError>;
    fn remove(&mut self, id: &str) -> Result<(), RepositoryError>;
    fn list_by_owner_org(&self, org_id: &str) -> Result<Vec<crate::domain::Ship>, RepositoryError>;
}

/// Repository trait for Event storage (append-only simple interface)
pub trait EventRepository {
    fn append(&mut self, event: crate::domain::Event) -> Result<(), RepositoryError>;
    fn list_all(&self) -> Result<Vec<crate::domain::Event>, RepositoryError>;
    fn list_by_org(&self, org_id: &str) -> Result<Vec<crate::domain::Event>, RepositoryError>;
}

/// Repository trait for Equipment (read-only source or registration)
pub trait EquipmentRepository {
    fn register(&mut self, equipment: crate::domain::Equipment) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<crate::domain::Equipment, RepositoryError>;
    fn list_all(&self) -> Result<Vec<crate::domain::Equipment>, RepositoryError>;
}

#[allow(dead_code)]
/// Repository trait for Roles and Permissions
pub trait RoleRepository {
    fn create(&mut self, role: crate::domain::Role) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<crate::domain::Role, RepositoryError>;
    fn update(&mut self, role: crate::domain::Role) -> Result<(), RepositoryError>;
    fn delete(&mut self, id: &str) -> Result<(), RepositoryError>;
}

pub trait PermissionRepository {
    fn create(&mut self, permission: crate::domain::Permission) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<crate::domain::Permission, RepositoryError>;
    fn list_all(&self) -> Result<Vec<crate::domain::Permission>, RepositoryError>;
}

/// Repository trait for Session management
pub trait SessionRepository {
    fn create(&mut self, sess: crate::domain::Session) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<crate::domain::Session, RepositoryError>;
    fn update(&mut self, sess: crate::domain::Session) -> Result<(), RepositoryError>;
    fn list_all(&self) -> Result<Vec<crate::domain::Session>, RepositoryError>;
    fn list_by_org(&self, org_id: &str) -> Result<Vec<crate::domain::Session>, RepositoryError>;
}
