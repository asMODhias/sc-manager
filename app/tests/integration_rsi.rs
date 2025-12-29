use sc_manager_app::commands::AddMemberCommand;
use sc_manager_app::handlers::member_handler::MemberHandler;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_core::repositories::MemberRepository;
use sc_manager_core::repositories::RepositoryError;

use sc_manager_adapters::rsi::RsiClient;
use sc_manager_adapters::rsi::SimpleRsiClient;

#[test]
fn add_member_with_verified_rsi() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);

    let client = SimpleRsiClient::new(vec!["good_handle"]);
    let cmd = AddMemberCommand::new("m1", Some("good_handle".to_string()), None);

    let res = handler.add_with_rsi(cmd, |h| client.verify_handle(h));
    assert!(res.is_ok());
    let m = repo.get("m1").unwrap();
    assert_eq!(m.rsi_handle.unwrap(), "good_handle");
}

#[test]
fn add_member_with_invalid_rsi_returns_notfound() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);

    let client = SimpleRsiClient::new(vec!["good_handle"]);
    let cmd = AddMemberCommand::new("m2", Some("bad_handle".to_string()), None);

    let res = handler.add_with_rsi(cmd, |h| client.verify_handle(h));
    assert_eq!(res.unwrap_err(), RepositoryError::NotFound);
}

#[test]
fn add_member_with_rsi_client_error_returns_internal() {
    let mut repo = InMemoryMemberRepo::new();
    let mut handler = MemberHandler::new(&mut repo);

    let client = SimpleRsiClient::new(Vec::<&str>::new()).with_error(true);
    let cmd = AddMemberCommand::new("m3", Some("any".to_string()), None);

    let res = handler.add_with_rsi(cmd, |h| client.verify_handle(h));
    assert_eq!(res.unwrap_err(), RepositoryError::Internal);
}
