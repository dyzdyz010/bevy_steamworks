//! High-level Bevy ECS integration for Steam Timeline.
//!
//! This module builds on top of the upstream [`steamworks::Timeline`] API. It
//! exposes Bevy messages for timeline state and event submissions, while
//! validating inputs that upstream converts into C strings.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksSystem};

mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksTimelineState;
pub use types::*;

/// Bevy plugin for high-level Steam Timeline commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksTimelineCommand`] and [`SteamworksTimelineResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksTimelinePlugin;

impl SteamworksTimelinePlugin {
    /// Creates a Timeline plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksTimelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksTimelineState>()
            .add_message::<SteamworksTimelineCommand>()
            .add_message::<SteamworksTimelineResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessTimelineCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_timeline_commands.in_set(SteamworksSystem::ProcessTimelineCommands),
            );
    }
}

fn process_timeline_commands(
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

fn validate_command(command: &SteamworksTimelineCommand) -> Result<(), SteamworksTimelineError> {
    match command {
        SteamworksTimelineCommand::SetStateDescription { description, .. } => {
            validate_steam_string("description", description)?;
            Ok(())
        }
        SteamworksTimelineCommand::ClearStateDescription { .. } => Ok(()),
        SteamworksTimelineCommand::AddEvent { event } => {
            validate_steam_string("icon", &event.icon)?;
            validate_steam_string("title", &event.title)?;
            validate_steam_string("description", &event.description)?;
            validate_finite_f32("start_offset_seconds", event.start_offset_seconds)
        }
        SteamworksTimelineCommand::SetGameMode { .. } => Ok(()),
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksTimelineError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksTimelineError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_finite_f32(field: &'static str, value: f32) -> Result<(), SteamworksTimelineError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SteamworksTimelineError::invalid_float(field))
    }
}
