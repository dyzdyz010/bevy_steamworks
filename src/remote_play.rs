//! High-level Bevy ECS integration for Steam Remote Play.
//!
//! This module builds on top of the upstream [`steamworks::RemotePlay`] API.
//! Session connect/disconnect callbacks still arrive through
//! [`crate::SteamworksEvent`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Bevy plugin for high-level Steam Remote Play commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksRemotePlayCommand`] and [`SteamworksRemotePlayResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
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
    /// [`crate::SteamworksEvent::RemotePlayConnected`] when a stable ID is needed.
    pub fn sessions(&self) -> &[SteamworksRemotePlaySessionSnapshot] {
        &self.sessions
    }

    /// Returns session snapshots read through ID-based commands.
    pub fn known_sessions(&self) -> &[SteamworksRemotePlaySessionInfo] {
        &self.known_sessions
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
            _ => {}
        }
    }
}

/// Snapshot of one Steam Remote Play session returned by bulk listing.
///
/// The upstream `steamworks` API does not expose the session ID from
/// [`steamworks::RemotePlay::sessions`]. Session IDs are available from
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
}

/// Result message emitted by [`SteamworksRemotePlayPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksRemotePlayResult {
    /// The command was submitted to Steamworks or a value was read.
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
    mut results: MessageWriter<SteamworksRemotePlayResult>,
) {
    let Some(client) = client else {
        let error = SteamworksRemotePlayError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
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
}
