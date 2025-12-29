use sc_manager_core::domain::session::Session;

#[test]
fn session_add_event_and_end() {
    let mut s = Session::new("s1", 0, None, Some("p1".into()));
    assert!(s.is_active());
    s.add_event("e1");
    assert_eq!(s.events, vec!["e1".to_string()]);
    s.end(10);
    assert!(!s.is_active());
}