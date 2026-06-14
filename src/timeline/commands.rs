use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::SteamworksClient;

use super::{
    messages::{
        SteamworksTimelineCommand, SteamworksTimelineError, SteamworksTimelineOperation,
        SteamworksTimelineResult,
    },
    state::SteamworksTimelineState,
    validation::validate_command,
};

pub(super) fn process_timeline_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksTimelineState>,
    mut commands: ResMut<Messages<SteamworksTimelineCommand>>,
    mut results: MessageWriter<SteamworksTimelineResult>,
) {
    let Some(client) = client else {
        let error = SteamworksTimelineError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Timeline command failed"
            );
            results.write(SteamworksTimelineResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_timeline_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks Timeline command"
                );
                results.write(SteamworksTimelineResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks Timeline command failed"
                );
                results.write(SteamworksTimelineResult::Err { command, error });
            }
        }
    }
}

fn handle_timeline_command(
    client: &SteamworksClient,
    command: &SteamworksTimelineCommand,
) -> Result<SteamworksTimelineOperation, SteamworksTimelineError> {
    validate_command(command)?;

    let timeline = client.timeline();
    match command {
        SteamworksTimelineCommand::SetGameMode { mode } => {
            timeline.set_timeline_game_mode(mode.to_steam());
            Ok(SteamworksTimelineOperation::GameModeSet { mode: *mode })
        }
        SteamworksTimelineCommand::SetStateDescription {
            description,
            duration,
        } => {
            timeline.set_timeline_state_description(description, *duration);
            Ok(SteamworksTimelineOperation::StateDescriptionSet {
                description: description.clone(),
                duration: *duration,
            })
        }
        SteamworksTimelineCommand::ClearStateDescription { duration } => {
            timeline.clear_timeline_state_description(*duration);
            Ok(SteamworksTimelineOperation::StateDescriptionCleared {
                duration: *duration,
            })
        }
        SteamworksTimelineCommand::AddEvent { event } => {
            timeline.add_timeline_event(
                &event.icon,
                &event.title,
                &event.description,
                event.priority,
                event.start_offset_seconds,
                event.duration,
                event.clip_priority.to_steam(),
            );
            Ok(SteamworksTimelineOperation::TimelineEventAdded {
                event: event.clone(),
            })
        }
    }
}
