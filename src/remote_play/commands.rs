use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    callbacks::process_remote_play_steam_events,
    messages::{
        SteamworksRemotePlayCommand, SteamworksRemotePlayError, SteamworksRemotePlayOperation,
        SteamworksRemotePlayResult,
    },
    state::SteamworksRemotePlayState,
    types::{SteamworksRemotePlaySessionInfo, SteamworksRemotePlaySessionSnapshot},
};

pub(super) fn process_remote_play_commands(
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
