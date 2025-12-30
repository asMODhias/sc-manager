use sc_manager_core::events::EventEnvelope;

#[test]
fn event_envelope_orgcreated() {
    let e = EventEnvelope::OrgCreated { id: "o1".into() };
    match e {
        EventEnvelope::OrgCreated { id } => assert_eq!(id, "o1"),
        _ => panic!("unexpected variant"),
    }
}
