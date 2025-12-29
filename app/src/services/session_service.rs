use crate::handlers::event_handler::EventHandler;
use crate::handlers::session_handler::SessionHandler;
use sc_manager_core::domain::game_event::GameEventType;
use sc_manager_core::events::EventEnvelope;
use sc_manager_core::repositories::{EventRepository, RepositoryError, SessionRepository};

/// Processes game log envelopes and wires them into Event + Session repositories.
pub struct SessionService;

impl SessionService {
    pub fn process_envelope<
        S: SessionRepository,
        E: EventRepository,
        M: sc_manager_core::repositories::MemberRepository,
        R: sc_manager_core::repositories::RoleRepository,
        P: sc_manager_core::repositories::PermissionRepository,
    >(
        env: EventEnvelope,
        session_repo: &mut S,
        event_repo: &mut E,
        mut member_repo: Option<&mut M>,
        role_repo: Option<&R>,
        permission_repo: Option<&P>,
    ) -> Result<(), RepositoryError> {
        match env {
            EventEnvelope::GameEvent {
                id,
                event_type,
                timestamp,
                details,
            } => {
                // If SessionStart -> create session
                match event_type {
                    GameEventType::SessionStart => {
                        // extract optional member from details (member=ID)
                        let mut member_id_opt: Option<String> = None;
                        if let Some(ref d) = details {
                            for tok in d.split_whitespace() {
                                if let Some(rest) = tok.strip_prefix("member=") {
                                    member_id_opt = Some(rest.trim_matches('"').to_string());
                                }
                            }
                        }
                        // If actor present and repos provided, check permission to start session
                        if let Some(ref actor) = member_id_opt {
                            if let (Some(mrepo_mut), Some(rrepo), Some(prepo)) =
                                (member_repo.as_mut(), role_repo, permission_repo)
                            {
                                let mrepo_ref: &M = mrepo_mut;
                                let allowed = crate::services::policy_service::PolicyService::can_start_session(actor, None, mrepo_ref, rrepo, prepo)?;
                                if !allowed {
                                    return Err(RepositoryError::Unauthorized);
                                }
                            }
                        }

                        let mut handler = SessionHandler::new(session_repo);
                        let sess_id = format!("sess-{}", timestamp);
                        handler.start(sess_id.clone(), timestamp, None, member_id_opt.clone())?;

                        // update member if provided
                        if let (Some(mrepo_mut), Some(mid)) = (member_repo.as_mut(), member_id_opt)
                        {
                            if let Ok(mut mm) = mrepo_mut.get(&mid) {
                                mm.last_session_id = Some(sess_id.clone());
                                mm.last_seen = Some(timestamp);
                                mrepo_mut.update(mm)?;
                            }
                        }

                        Ok(())
                    }
                    GameEventType::SessionEnd => {
                        // end last active session
                        // find active session id first (avoid overlapping borrows)
                        let sessions = session_repo.list_all()?;
                        if let Some(s) = sessions.into_iter().rev().find(|s| s.is_active()) {
                            let mut handler = SessionHandler::new(session_repo);
                            handler.end(&s.id, timestamp)
                        } else {
                            Err(RepositoryError::NotFound)
                        }
                    }
                    _ => {
                        // extract optional member from details (member=ID)
                        let mut member_id_opt: Option<String> = None;
                        if let Some(ref d) = details {
                            for tok in d.split_whitespace() {
                                if let Some(rest) = tok.strip_prefix("member=") {
                                    member_id_opt = Some(rest.trim_matches('"').to_string());
                                }
                            }
                        }

                        // If actor present and role+permission repos provided, check permission
                        if let Some(ref actor) = member_id_opt {
                            if let (Some(mrepo_mut), Some(rrepo), Some(prepo)) =
                                (member_repo.as_mut(), role_repo, permission_repo)
                            {
                                // create an immutable borrow for check
                                let mrepo_ref: &M = mrepo_mut;
                                let allowed = crate::services::policy_service::PolicyService::check_permission(actor, "event.create", None, mrepo_ref, rrepo, prepo)?;
                                if !allowed {
                                    return Err(RepositoryError::Unauthorized);
                                }
                            }
                        }

                        // create event and attach to active session
                        let mut evt_handler = EventHandler::new(event_repo);
                        let title = if let Some(d) = details {
                            format!("{:?} {}", event_type, d)
                        } else {
                            format!("{:?}", event_type)
                        };
                        evt_handler.create(crate::commands::CreateEventCommand::new(
                            id.clone(),
                            title,
                            timestamp,
                            None,
                        ))?;

                        // update member last_seen if member repo present
                        if let Some(ref actor) = member_id_opt {
                            if let Some(mrepo_mut) = member_repo.as_mut() {
                                if let Ok(mut mm) = mrepo_mut.get(actor) {
                                    mm.last_seen = Some(timestamp);
                                    let _ = mrepo_mut.update(mm);
                                }
                            }
                        }

                        // attach to session if active
                        let mut sess_handler = SessionHandler::new(session_repo);
                        sess_handler.add_event_to_active_session(id)
                    }
                }
            }
            _ => Ok(()),
        }
    }
}
