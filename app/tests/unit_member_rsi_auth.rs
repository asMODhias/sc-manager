use sc_manager_app::handlers::member_handler::MemberHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_core::repositories::RepositoryError;

#[test]
fn add_with_rsi_success() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);

    let cmd = sc_manager_app::commands::AddMemberCommand::new("m1", Some("alice".to_string()), None);
    let res = handler.add_with_rsi(cmd, |_| Ok(true));
    assert!(res.is_ok());
}

#[test]
fn add_with_rsi_not_found() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);

    let cmd = sc_manager_app::commands::AddMemberCommand::new("m2", Some("unknown".to_string()), None);
    let res = handler.add_with_rsi(cmd, |_| Ok(false));
    assert_eq!(res.unwrap_err(), RepositoryError::NotFound);
}

#[test]
fn add_with_rsi_verifier_error() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);

    let cmd = sc_manager_app::commands::AddMemberCommand::new("m3", Some("any".to_string()), None);
    let res = handler.add_with_rsi(cmd, |_| Err("service error".to_string()));
    assert_eq!(res.unwrap_err(), RepositoryError::Internal);
}