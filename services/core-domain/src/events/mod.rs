//! Domain events module

pub trait DomainEvent: Send + Sync {
    fn event_name(&self) -> &'static str;
}

// Example generic event envelope
#[derive(Debug, Clone)]
pub enum EventEnvelope {
    OrgCreated {
        id: String,
    },
    MemberAdded {
        member_id: String,
    },
    // Game related events coming from adapters like game.log
    GameEvent {
        id: String,
        event_type: crate::domain::game_event::GameEventType,
        timestamp: i64,
        details: Option<String>,
    },
}

pub mod signing;
pub use signing::{DomainEventPayload, SignedEvent, sign_event, verify_signature, generate_test_keypair, KeyPair};
