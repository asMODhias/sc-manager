mod test_utils;
use sc_manager_app::in_memory_event_repo::InMemoryEventRepo;
use sc_manager_app::in_memory_member_repo::InMemoryMemberRepo;
use sc_manager_app::in_memory_permission_repo::InMemoryPermissionRepo;
use sc_manager_app::in_memory_role_repo::InMemoryRoleRepo;
use sc_manager_app::in_memory_session_repo::InMemorySessionRepo;
use sc_manager_app::services::session_service::SessionService;
use sc_manager_core::domain::game_event::GameEventType;
use sc_manager_core::events::EventEnvelope;
use sc_manager_core::repositories::RepositoryError;
use sc_manager_core::repositories::{MemberRepository, RoleRepository};

#[test]
fn event_creation_denied_when_member_lacks_permission() {
    let mut role_repo = InMemoryRoleRepo::new();
    let mut member_repo = InMemoryMemberRepo::new();
    let perm_repo = InMemoryPermissionRepo::new();

    // role without the event.create permission
    let r = sc_manager_core::domain::Role::new("r-no", "NoEvents");
    role_repo.create(r).unwrap();

    member_repo
        .add(sc_manager_core::domain::Member::new("nope"))
        .unwrap();
    let mut m = member_repo.get("nope").unwrap();
    m.assign_role("r-no", None);
    member_repo.update(m).unwrap();

    let env = EventEnvelope::GameEvent {
        id: "e1".into(),
        event_type: GameEventType::Kill,
        timestamp: 1610001000,
        details: Some("member=\"nope\"".into()),
    };

    let mut session_repo = InMemorySessionRepo::new();
    let mut event_repo = InMemoryEventRepo::new();

    let res = SessionService::process_envelope(
        env,
        &mut session_repo,
        &mut event_repo,
        Some(&mut member_repo),
        Some(&role_repo),
        Some(&perm_repo),
    );
    assert_eq!(res.unwrap_err(), RepositoryError::Unauthorized);
}
