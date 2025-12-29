// Re-export signing utilities from `sc_manager_core` so application uses canonical definitions
pub use sc_manager_core::events::{DomainEventPayload as DomainEvent, SignedEvent, sign_event, verify_signature, generate_test_keypair};
