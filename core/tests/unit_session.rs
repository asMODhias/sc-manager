use sc_manager_core::domain::session::Session;

#[test]
fn session_lifecycle() {
    let mut s = Session::new("sess-1", 1_700_000_000, None, Some("user-1".into()));
    assert!(s.is_active());
    s.add_event("ev-1");
    assert_eq!(s.events, vec!["ev-1".to_string()]);

    s.end(1_700_000_100);
    assert!(!s.is_active());
    assert_eq!(s.end_ts, Some(1_700_000_100));
}