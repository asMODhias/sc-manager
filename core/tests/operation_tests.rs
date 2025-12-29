use sc_manager_core::domain::Operation;

#[test]
fn create_operation_starts_in_planning() {
    let op = Operation::new("op1", "Recon", "explore", 1700000000);
    assert_eq!(op.id, "op1");
    assert_eq!(op.name, "Recon");
    assert_eq!(op.status, sc_manager_core::domain::OperationStatus::Planning);
    assert_eq!(op.phases.len(), 0);
}

#[test]
fn add_phase_and_complete_updates_status() {
    let mut op = Operation::new("op2", "Raid", "pvp", 1700000000);
    op.add_phase("phase1", "Approach");
    op.add_phase("phase2", "Engage");
    assert_eq!(op.phases.len(), 2);

    op.start();
    assert_eq!(op.status, sc_manager_core::domain::OperationStatus::Active);

    op.complete_phase("phase1");
    assert!(op.phases.iter().any(|p| p.id == "phase1" && p.completed_at.is_some()));

    op.complete_phase("phase2");
    assert_eq!(op.status, sc_manager_core::domain::OperationStatus::Completed);
}
