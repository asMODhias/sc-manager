use sc_manager_eventbus_nats::EventBus;
use sc_manager_core::events::{DomainEventPayload, KeyPair};
use serde_json::json;

/// CreateOperation command payload
pub struct CreateOperation {
    pub id: String,
    pub name: String,
    pub mission_type: String,
    pub org_id: String,
}

/// Handle CreateOperation by publishing a signed DomainEvent
pub fn handle_create_operation<E: EventBus>(
    bus: &E,
    kp: &KeyPair,
    cmd: &CreateOperation,
) -> Result<(), String> {
    let ev = DomainEventPayload {
        id: cmd.id.clone(),
        kind: "OperationCreated".into(),
        payload: json!({ "operation_id": cmd.id, "name": cmd.name, "org_id": cmd.org_id }),
    };

    // Topic per spec
    super::services::publisher::sign_and_publish(bus, "domain.operations", kp, &ev)
}

/// StartOperation command
pub struct StartOperation {
    pub operation_id: String,
    pub org_id: String,
}

pub fn handle_start_operation<E: EventBus>(
    bus: &E,
    kp: &KeyPair,
    cmd: &StartOperation,
) -> Result<(), String> {
    let ev = DomainEventPayload {
        id: format!("start-{}", cmd.operation_id),
        kind: "OperationStarted".into(),
        payload: json!({ "operation_id": cmd.operation_id, "org_id": cmd.org_id }),
    };

    super::services::publisher::sign_and_publish(bus, "domain.operations", kp, &ev)
}

/// CompletePhase command
pub struct CompletePhase {
    pub operation_id: String,
    pub phase_id: String,
    pub org_id: String,
}

pub fn handle_complete_phase<E: EventBus>(
    bus: &E,
    kp: &KeyPair,
    cmd: &CompletePhase,
) -> Result<(), String> {
    let ev = DomainEventPayload {
        id: format!("phase-{}-{}", cmd.operation_id, cmd.phase_id),
        kind: "PhaseCompleted".into(),
        payload: json!({ "operation_id": cmd.operation_id, "phase_id": cmd.phase_id, "org_id": cmd.org_id }),
    };

    super::services::publisher::sign_and_publish(bus, "domain.operations", kp, &ev)
}