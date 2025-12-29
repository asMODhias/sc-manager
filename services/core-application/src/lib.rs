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

pub mod signing;

#[cfg(test)]
mod tests {
    use super::signing::*;

    #[test]
    fn sign_and_verify_roundtrip() {
        // Use deterministic generation in CI if needed.
        let kp = generate_test_keypair();
        let ev = DomainEvent { id: "e1".into(), kind: "Test".into(), payload: serde_json::json!({"x":1}) };
        let s = sign_event(&kp, &ev);
        assert!(verify_signature(&s), "signature should verify for signed event");
    }
}
