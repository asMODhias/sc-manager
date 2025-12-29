//! Application layer (commands / queries / handlers)

pub mod commands;
pub mod handlers;
pub mod queries;

pub mod in_memory_equipment_repo;
pub mod in_memory_event_repo;
pub mod in_memory_fleet_repo;
pub mod in_memory_member_repo;
pub mod in_memory_permission_repo;
pub mod in_memory_repo;
pub mod in_memory_role_repo;
pub mod in_memory_session_repo;
pub mod in_memory_ship_repo;

pub mod services;

// Example: commands live in `commands` module
