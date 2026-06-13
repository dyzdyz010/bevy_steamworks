//! High-level Bevy ECS integration for Steam Remote Play.
//!
//! This module builds on top of the upstream [`steamworks::RemotePlay`] API.
//! Session connect/disconnect callbacks are mirrored from
//! [`crate::SteamworksEvent`] into [`SteamworksRemotePlayResult`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Bevy plugin for high-level Steam Remote Play commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksRemotePlayCommand`] and [`SteamworksRemotePlayResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
/// It also mirrors Remote Play session connect/disconnect callbacks into Remote
/// Play results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksRemotePlayPlugin;

impl SteamworksRemotePlayPlugin {
    /// Creates a Remote Play plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksRemotePlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksRemotePlayState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksRemotePlayCommand>()
            .add_message::<SteamworksRemotePlayResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessRemotePlayCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_remote_play_commands.in_set(SteamworksSystem::ProcessRemotePlayCommands),
            );
    }
}

/// Runtime state for [`SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksRemotePlayState {
    last_error: Option<SteamworksRemotePlayError>,
    sessions: Vec<SteamworksRemotePlaySessionSnapshot>,
    known_sessions: Vec<SteamworksRemotePlaySessionInfo>,
    observed_connected_sessions: Vec<steamworks::RemotePlaySessionId>,
    last_submitted_invite: Option<SteamworksRemotePlayInvite>,
    submitted_invite_count: u64,
}

impl SteamworksRemotePlayState {
    /// Returns the most recent synchronous error observed by the Remote Play plugin.
    pub fn last_error(&self) -> Option<&SteamworksRemotePlayError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent active Remote Play session list read through the plugin.
    ///
    /// Upstream `steamworks` does not expose session IDs from bulk session listing;
    /// use [`SteamworksRemotePlayCommand::GetSession`] with IDs from
    /// [`SteamworksRemotePlayOperation::SessionConnected`] or
    /// [`crate::SteamworksEvent::RemotePlayConnected`] when a stable ID is needed.
    pub fn sessions(&self) -> &[SteamworksRemotePlaySessionSnapshot] {
        &self.sessions
    }

    /// Returns session snapshots read through ID-based commands.
    pub fn known_sessions(&self) -> &[SteamworksRemotePlaySessionInfo] {
        &self.known_sessions
    }

    /// Returns one ID-based session snapshot read through the plugin.
    pub fn known_session(
        &self,
        session: steamworks::RemotePlaySessionId,
    ) -> Option<&SteamworksRemotePlaySessionInfo> {
        self.known_sessions
            .iter()
            .find(|known| known.session == session)
    }

    /// Returns session IDs observed as connected and not yet disconnected.
    ///
    /// This list is callback-driven and only reflects sessions observed while
    /// this plugin has been running. Use [`SteamworksRemotePlayCommand::ListSessions`]
    /// for a fresh bulk snapshot from Steam.
    pub fn observed_connected_sessions(&self) -> &[steamworks::RemotePlaySessionId] {
        &self.observed_connected_sessions
    }

    /// Returns whether a session has been observed as connected and not yet disconnected.
    pub fn is_session_observed_connected(&self, session: steamworks::RemotePlaySessionId) -> bool {
        self.observed_connected_sessions.contains(&session)
    }

    /// Returns the most recent Remote Play Together invite submitted through this command layer.
    pub fn last_submitted_invite(&self) -> Option<SteamworksRemotePlayInvite> {
        self.last_submitted_invite
    }

    /// Returns how many Remote Play Together invites this plugin successfully submitted.
    pub fn submitted_invite_count(&self) -> u64 {
        self.submitted_invite_count
    }

    fn record_error(&mut self, error: SteamworksRemotePlayError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksRemotePlayOperation) {
        match operation {
            SteamworksRemotePlayOperation::SessionsListed { sessions } => {
                self.sessions.clone_from(sessions);
            }
            SteamworksRemotePlayOperation::SessionRead { session } => {
                if let Some(existing) = self
                    .known_sessions
                    .iter_mut()
                    .find(|known| known.session == session.session)
                {
                    *existing = session.clone();
                } else {
                    self.known_sessions.push(session.clone());
                }
            }
            SteamworksRemotePlayOperation::InviteSubmitted { session, friend } => {
                self.last_submitted_invite = Some(SteamworksRemotePlayInvite {
                    session: *session,
                    friend: *friend,
                });
                self.submitted_invite_count = self.submitted_invite_count.saturating_add(1);
            }
            SteamworksRemotePlayOperation::SessionConnected { session } => {
                if !self.observed_connected_sessions.contains(session) {
                    self.observed_connected_sessions.push(*session);
                }
            }
            SteamworksRemotePlayOperation::SessionDisconnected { session } => {
                self.observed_connected_sessions
                    .retain(|known| known != session);
                self.known_sessions
                    .retain(|known| known.session != *session);
            }
        }
    }
}

/// Snapshot of one Steam Remote Play session returned by bulk listing.
///
/// The upstream `steamworks` API does not expose the session ID from
/// [`steamworks::RemotePlay::sessions`]. Session IDs are available from
/// [`SteamworksRemotePlayOperation::SessionConnected`] or
/// [`crate::SteamworksEvent::RemotePlayConnected`] and can then be queried with
/// [`SteamworksRemotePlayCommand::GetSession`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemotePlaySessionSnapshot {
    /// Steam user associated with this session.
    pub user: steamworks::SteamId,
    /// Client device name, or `None` if the session has expired.
    pub client_name: Option<String>,
    /// Client device form factor, or `None` if unknown or expired.
    pub client_form_factor: Option<steamworks::SteamDeviceFormFactor>,
    /// Client resolution, or `None` if the session has expired.
    pub client_resolution: Option<(u32, u32)>,
}

/// Snapshot of one Steam Remote Play session with a known session ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksRemotePlaySessionInfo {
    /// Remote Play session ID.
    pub session: steamworks::RemotePlaySessionId,
    /// Steam user associated with this session.
    pub user: steamworks::SteamId,
    /// Client device name, or `None` if the session has expired.
    pub client_name: Option<String>,
    /// Client device form factor, or `None` if unknown or expired.
    pub client_form_factor: Option<steamworks::SteamDeviceFormFactor>,
    /// Client resolution, or `None` if the session has expired.
    pub client_resolution: Option<(u32, u32)>,
}

/// Remote Play Together invite accepted by Steam.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksRemotePlayInvite {
    /// Session context supplied by the caller.
    pub session: steamworks::RemotePlaySessionId,
    /// Friend Steam ID invited.
    pub friend: steamworks::SteamId,
}

/// A high-level command for Steam Remote Play workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemotePlayCommand {
    /// Read all active Remote Play sessions.
    ListSessions,
    /// Read one Remote Play session snapshot.
    GetSession {
        /// Session to inspect.
        session: steamworks::RemotePlaySessionId,
    },
    /// Invite a friend to join using Remote Play Together.
    ///
    /// The current upstream Rust wrapper exposes invites through
    /// [`steamworks::RemotePlaySession`]. The session ID is retained in the
    /// result as caller-provided context, but Steam's invite acceptance is not
    /// proof that the invite was session-specific.
    Invite {
        /// Session context used to access the upstream invite helper.
        session: steamworks::RemotePlaySessionId,
        /// Friend Steam ID to invite.
        friend: steamworks::SteamId,
    },
}

impl SteamworksRemotePlayCommand {
    /// Creates a [`SteamworksRemotePlayCommand::GetSession`] command.
    pub fn get_session(session: steamworks::RemotePlaySessionId) -> Self {
        Self::GetSession { session }
    }

    /// Creates a [`SteamworksRemotePlayCommand::Invite`] command.
    pub fn invite(session: steamworks::RemotePlaySessionId, friend: steamworks::SteamId) -> Self {
        Self::Invite { session, friend }
    }
}

/// A successfully submitted Steam Remote Play operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksRemotePlayOperation {
    /// Active Remote Play sessions were listed.
    SessionsListed {
        /// Session snapshots.
        sessions: Vec<SteamworksRemotePlaySessionSnapshot>,
    },
    /// One Remote Play session was read.
    SessionRead {
        /// Session snapshot.
        session: SteamworksRemotePlaySessionInfo,
    },
    /// A Remote Play Together invite was submitted.
    InviteSubmitted {
        /// Session context supplied by the caller.
        session: steamworks::RemotePlaySessionId,
        /// Friend Steam ID invited.
        friend: steamworks::SteamId,
    },
    /// A Remote Play session connected callback was observed.
    SessionConnected {
        /// Session that connected.
        session: steamworks::RemotePlaySessionId,
    },
    /// A Remote Play session disconnected callback was observed.
    SessionDisconnected {
        /// Session that disconnected.
        session: steamworks::RemotePlaySessionId,
    },
}

/// Result message emitted by [`SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemotePlayResult {
    /// The command or observed callback was processed successfully.
    Ok(SteamworksRemotePlayOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksRemotePlayCommand,
        /// Failure reason.
        error: SteamworksRemotePlayError,
    },
}

/// Synchronous errors from [`SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksRemotePlayError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// The upstream Steamworks API rejected the invite.
    #[error("Steamworks Remote Play Together invite failed")]
    InviteFailed,
}

fn process_remote_play_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksRemotePlayState>,
    mut commands: ResMut<Messages<SteamworksRemotePlayCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksRemotePlayResult>,
) {
    process_remote_play_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksRemotePlayError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Remote Play command failed"
            );
            results.write(SteamworksRemotePlayResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_remote_play_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks Remote Play command"
                );
                results.write(SteamworksRemotePlayResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks Remote Play command failed"
                );
                results.write(SteamworksRemotePlayResult::Err { command, error });
            }
        }
    }
}

fn process_remote_play_steam_events(
    state: &mut SteamworksRemotePlayState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksRemotePlayResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::RemotePlayConnected(event) => {
                SteamworksRemotePlayOperation::SessionConnected {
                    session: event.session,
                }
            }
            SteamworksEvent::RemotePlayDisconnected(event) => {
                SteamworksRemotePlayOperation::SessionDisconnected {
                    session: event.session,
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks Remote Play callback"
        );
        results.write(SteamworksRemotePlayResult::Ok(operation));
    }
}

fn handle_remote_play_command(
    client: &SteamworksClient,
    command: &SteamworksRemotePlayCommand,
) -> Result<SteamworksRemotePlayOperation, SteamworksRemotePlayError> {
    let remote_play = client.remote_play();
    match command {
        SteamworksRemotePlayCommand::ListSessions => {
            let sessions = remote_play
                .sessions()
                .iter()
                .map(snapshot_session)
                .collect();
            Ok(SteamworksRemotePlayOperation::SessionsListed { sessions })
        }
        SteamworksRemotePlayCommand::GetSession { session } => {
            let session_ref = remote_play.session(*session);
            Ok(SteamworksRemotePlayOperation::SessionRead {
                session: snapshot_known_session(*session, &session_ref),
            })
        }
        SteamworksRemotePlayCommand::Invite { session, friend } => {
            let session_ref = remote_play.session(*session);
            if session_ref.invite(*friend) {
                Ok(SteamworksRemotePlayOperation::InviteSubmitted {
                    session: *session,
                    friend: *friend,
                })
            } else {
                Err(SteamworksRemotePlayError::InviteFailed)
            }
        }
    }
}

fn snapshot_session(
    session: &steamworks::RemotePlaySession,
) -> SteamworksRemotePlaySessionSnapshot {
    SteamworksRemotePlaySessionSnapshot {
        user: session.user(),
        client_name: session.client_name(),
        client_form_factor: session.client_form_factor(),
        client_resolution: session.client_resolution(),
    }
}

fn snapshot_known_session(
    id: steamworks::RemotePlaySessionId,
    session: &steamworks::RemotePlaySession,
) -> SteamworksRemotePlaySessionInfo {
    let snapshot = snapshot_session(session);
    SteamworksRemotePlaySessionInfo {
        session: id,
        user: snapshot.user,
        client_name: snapshot.client_name,
        client_form_factor: snapshot.client_form_factor,
        client_resolution: snapshot.client_resolution,
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn remote_play_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksRemotePlayPlugin::new());

        assert!(app.world().contains_resource::<SteamworksRemotePlayState>());
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksRemotePlayCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksRemotePlayResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksRemotePlayPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksRemotePlayCommand>>()
            .write(SteamworksRemotePlayCommand::ListSessions);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksRemotePlayResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksRemotePlayResult::Err {
                command: SteamworksRemotePlayCommand::ListSessions,
                error: SteamworksRemotePlayError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksRemotePlayState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksRemotePlayError::ClientUnavailable)
        );
    }

    #[test]
    fn constructors_preserve_session_context() {
        let session = steamworks::RemotePlaySessionId::from_raw(7);
        let friend = steamworks::SteamId::from_raw(42);

        assert_eq!(
            SteamworksRemotePlayCommand::get_session(session),
            SteamworksRemotePlayCommand::GetSession { session }
        );
        assert_eq!(
            SteamworksRemotePlayCommand::invite(session, friend),
            SteamworksRemotePlayCommand::Invite { session, friend }
        );
    }

    #[test]
    fn state_records_remote_play_operations() {
        let mut state = SteamworksRemotePlayState::default();
        let first_session = steamworks::RemotePlaySessionId::from_raw(1);
        let second_session = steamworks::RemotePlaySessionId::from_raw(2);
        let friend = steamworks::SteamId::from_raw(42);
        let first_info = SteamworksRemotePlaySessionInfo {
            session: first_session,
            user: steamworks::SteamId::from_raw(100),
            client_name: Some("Deck".to_owned()),
            client_form_factor: None,
            client_resolution: Some((1280, 800)),
        };
        let updated_info = SteamworksRemotePlaySessionInfo {
            client_name: Some("Living Room".to_owned()),
            client_resolution: Some((1920, 1080)),
            ..first_info.clone()
        };
        let listed_session = SteamworksRemotePlaySessionSnapshot {
            user: steamworks::SteamId::from_raw(200),
            client_name: Some("Laptop".to_owned()),
            client_form_factor: None,
            client_resolution: None,
        };

        state.record_operation(&SteamworksRemotePlayOperation::SessionsListed {
            sessions: vec![listed_session.clone()],
        });
        state.record_operation(&SteamworksRemotePlayOperation::SessionRead {
            session: first_info,
        });
        state.record_operation(&SteamworksRemotePlayOperation::SessionRead {
            session: updated_info.clone(),
        });
        state.record_operation(&SteamworksRemotePlayOperation::SessionConnected {
            session: first_session,
        });
        state.record_operation(&SteamworksRemotePlayOperation::SessionConnected {
            session: first_session,
        });
        state.record_operation(&SteamworksRemotePlayOperation::SessionConnected {
            session: second_session,
        });
        state.record_operation(&SteamworksRemotePlayOperation::InviteSubmitted {
            session: first_session,
            friend,
        });
        state.record_operation(&SteamworksRemotePlayOperation::InviteSubmitted {
            session: second_session,
            friend,
        });

        assert_eq!(state.sessions(), &[listed_session]);
        assert_eq!(state.known_sessions(), std::slice::from_ref(&updated_info));
        assert_eq!(state.known_session(first_session), Some(&updated_info));
        assert!(state.known_session(second_session).is_none());
        assert_eq!(
            state.observed_connected_sessions(),
            &[first_session, second_session]
        );
        assert!(state.is_session_observed_connected(first_session));
        assert_eq!(
            state.last_submitted_invite(),
            Some(SteamworksRemotePlayInvite {
                session: second_session,
                friend,
            })
        );
        assert_eq!(state.submitted_invite_count(), 2);

        state.record_operation(&SteamworksRemotePlayOperation::SessionDisconnected {
            session: first_session,
        });

        assert_eq!(state.observed_connected_sessions(), &[second_session]);
        assert!(!state.is_session_observed_connected(first_session));
        assert!(state.known_session(first_session).is_none());
    }

    #[test]
    fn remote_play_callbacks_are_bridged_without_client() {
        let mut app = App::new();
        let first = steamworks::RemotePlaySessionId::from_raw(1);
        let second = steamworks::RemotePlaySessionId::from_raw(2);

        app.add_plugins(SteamworksRemotePlayPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::RemotePlayConnected(
                steamworks::RemotePlayConnected { session: first },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::RemotePlayConnected(
                steamworks::RemotePlayConnected { session: second },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::RemotePlayDisconnected(
                steamworks::RemotePlayDisconnected { session: first },
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksRemotePlayResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        assert_eq!(
            drained,
            vec![
                SteamworksRemotePlayResult::Ok(SteamworksRemotePlayOperation::SessionConnected {
                    session: first
                },),
                SteamworksRemotePlayResult::Ok(SteamworksRemotePlayOperation::SessionConnected {
                    session: second
                },),
                SteamworksRemotePlayResult::Ok(
                    SteamworksRemotePlayOperation::SessionDisconnected { session: first },
                ),
            ]
        );

        let state = app.world().resource::<SteamworksRemotePlayState>();
        assert_eq!(state.observed_connected_sessions(), &[second]);
        assert_eq!(state.last_error(), None);
    }
}
