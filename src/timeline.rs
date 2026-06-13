//! High-level Bevy ECS integration for Steam Timeline.
//!
//! This module builds on top of the upstream [`steamworks::Timeline`] API. It
//! exposes Bevy messages for timeline state and event submissions, while
//! validating inputs that upstream converts into C strings.

use std::time::Duration;

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

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

/// Runtime state for [`SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksTimelineState {
    last_error: Option<SteamworksTimelineError>,
    game_mode: Option<SteamworksTimelineGameMode>,
    state_description: Option<SteamworksTimelineStateDescription>,
    last_event: Option<SteamworksTimelineEventInfo>,
    event_count: u64,
}

impl SteamworksTimelineState {
    /// Returns the most recent synchronous error observed by the Timeline plugin.
    pub fn last_error(&self) -> Option<&SteamworksTimelineError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent Timeline game mode submitted through the plugin.
    pub fn game_mode(&self) -> Option<SteamworksTimelineGameMode> {
        self.game_mode
    }

    /// Returns the current Timeline state description tracked by the plugin.
    pub fn state_description(&self) -> Option<&SteamworksTimelineStateDescription> {
        self.state_description.as_ref()
    }

    /// Returns the most recent Timeline event submitted through the plugin.
    pub fn last_event(&self) -> Option<&SteamworksTimelineEventInfo> {
        self.last_event.as_ref()
    }

    /// Returns the number of Timeline events submitted through the plugin.
    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    fn record_error(&mut self, error: SteamworksTimelineError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksTimelineOperation) {
        match operation {
            SteamworksTimelineOperation::GameModeSet { mode } => {
                self.game_mode = Some(*mode);
            }
            SteamworksTimelineOperation::StateDescriptionSet {
                description,
                duration,
            } => {
                self.state_description = Some(SteamworksTimelineStateDescription {
                    description: description.clone(),
                    duration: *duration,
                });
            }
            SteamworksTimelineOperation::StateDescriptionCleared { .. } => {
                self.state_description = None;
            }
            SteamworksTimelineOperation::TimelineEventAdded { event } => {
                self.last_event = Some(event.clone());
                self.event_count = self.event_count.saturating_add(1);
            }
        }
    }
}

/// Steam Timeline game mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksTimelineGameMode {
    /// The player is fully loaded into the game and playing.
    Playing,
    /// The player is in a multiplayer lobby.
    Staging,
    /// The player is in the game's main menu or a pause menu.
    Menus,
    /// The player is waiting for a loading screen.
    LoadingScreen,
}

impl SteamworksTimelineGameMode {
    fn to_steam(self) -> steamworks::TimelineGameMode {
        match self {
            Self::Playing => steamworks::TimelineGameMode::Playing,
            Self::Staging => steamworks::TimelineGameMode::Staging,
            Self::Menus => steamworks::TimelineGameMode::Menus,
            Self::LoadingScreen => steamworks::TimelineGameMode::LoadingScreen,
        }
    }
}

/// Steam Timeline event clip priority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksTimelineEventClipPriority {
    /// This event is not appropriate as a clip.
    None,
    /// The user may want to make a clip around this event.
    Standard,
    /// The player is likely to want a clip around this event.
    Featured,
}

impl SteamworksTimelineEventClipPriority {
    fn to_steam(self) -> steamworks::TimelineEventClipPriority {
        match self {
            Self::None => steamworks::TimelineEventClipPriority::None,
            Self::Standard => steamworks::TimelineEventClipPriority::Standard,
            Self::Featured => steamworks::TimelineEventClipPriority::Featured,
        }
    }
}

/// Timeline state description tracked by this command layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksTimelineStateDescription {
    /// Timeline tooltip text.
    pub description: String,
    /// Duration over which Steam should apply the change.
    pub duration: Duration,
}

/// Timeline event submitted through this command layer.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksTimelineEventInfo {
    /// Icon identifier configured for the game in Steamworks.
    pub icon: String,
    /// Event title.
    pub title: String,
    /// Event description.
    pub description: String,
    /// Event priority. Higher priority events are shown more prominently by Steam.
    pub priority: u32,
    /// Start offset in seconds relative to now.
    pub start_offset_seconds: f32,
    /// Event duration.
    pub duration: Duration,
    /// Clip priority submitted to Steam.
    pub clip_priority: SteamworksTimelineEventClipPriority,
}

/// A high-level command for Steam Timeline workflows.
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksTimelineCommand {
    /// Set the current Timeline game mode.
    SetGameMode {
        /// Game mode to submit to Steam.
        mode: SteamworksTimelineGameMode,
    },
    /// Set a Timeline state description tooltip.
    SetStateDescription {
        /// Tooltip text.
        description: String,
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Clear the current Timeline state description tooltip.
    ClearStateDescription {
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Add an event marker to the Steam Timeline.
    AddEvent {
        /// Event details.
        event: SteamworksTimelineEventInfo,
    },
}

impl SteamworksTimelineCommand {
    /// Creates a [`SteamworksTimelineCommand::SetGameMode`] command.
    pub fn set_game_mode(mode: SteamworksTimelineGameMode) -> Self {
        Self::SetGameMode { mode }
    }

    /// Creates a [`SteamworksTimelineCommand::SetStateDescription`] command.
    pub fn set_state_description(description: impl Into<String>, duration: Duration) -> Self {
        Self::SetStateDescription {
            description: description.into(),
            duration,
        }
    }

    /// Creates a [`SteamworksTimelineCommand::ClearStateDescription`] command.
    pub fn clear_state_description(duration: Duration) -> Self {
        Self::ClearStateDescription { duration }
    }

    /// Creates a [`SteamworksTimelineCommand::AddEvent`] command.
    pub fn add_event(event: SteamworksTimelineEventInfo) -> Self {
        Self::AddEvent { event }
    }
}

/// A Steam Timeline operation accepted by this command layer.
///
/// The operation has been submitted to the upstream `steamworks` wrapper. Steam
/// may still no-op the request when the runtime Timeline interface is
/// unavailable.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksTimelineOperation {
    /// Timeline game mode was submitted.
    GameModeSet {
        /// Game mode submitted to Steam.
        mode: SteamworksTimelineGameMode,
    },
    /// Timeline state description was submitted.
    StateDescriptionSet {
        /// Tooltip text.
        description: String,
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Timeline state description clear was submitted.
    StateDescriptionCleared {
        /// Duration over which Steam should apply the change.
        duration: Duration,
    },
    /// Timeline event was submitted.
    TimelineEventAdded {
        /// Event details submitted to Steam.
        event: SteamworksTimelineEventInfo,
    },
}

/// Result message emitted by [`SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksTimelineResult {
    /// The command was accepted and submitted to the upstream wrapper.
    ///
    /// This does not guarantee that Steam applied the request; the upstream
    /// Timeline wrapper no-ops when the runtime interface is unavailable.
    Ok(SteamworksTimelineOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksTimelineCommand,
        /// Failure reason.
        error: SteamworksTimelineError,
    },
}

/// Synchronous errors from [`SteamworksTimelinePlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksTimelineError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks Timeline command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A floating-point value is not finite.
    #[error("Steamworks Timeline command field {field} must be finite")]
    InvalidFloat {
        /// Field whose value was invalid.
        field: &'static str,
    },
}

impl SteamworksTimelineError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn invalid_float(field: &'static str) -> Self {
        Self::InvalidFloat { field }
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

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn timeline_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksTimelinePlugin::new());

        assert!(app.world().contains_resource::<SteamworksTimelineState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksTimelineCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksTimelineResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksTimelinePlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksTimelineCommand>>()
            .write(SteamworksTimelineCommand::set_game_mode(
                SteamworksTimelineGameMode::Playing,
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksTimelineResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksTimelineResult::Err {
                command: SteamworksTimelineCommand::set_game_mode(
                    SteamworksTimelineGameMode::Playing
                ),
                error: SteamworksTimelineError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksTimelineState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksTimelineError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_interior_nul() {
        let command =
            SteamworksTimelineCommand::set_state_description("boss\0phase", Duration::from_secs(1));

        assert_eq!(
            validate_command(&command),
            Err(SteamworksTimelineError::InvalidString {
                field: "description",
            })
        );

        let command = SteamworksTimelineCommand::add_event(SteamworksTimelineEventInfo {
            icon: "skull".to_owned(),
            title: "wipe\0bad".to_owned(),
            description: "party defeated".to_owned(),
            priority: 10,
            start_offset_seconds: 0.0,
            duration: Duration::ZERO,
            clip_priority: SteamworksTimelineEventClipPriority::Featured,
        });

        assert_eq!(
            validate_command(&command),
            Err(SteamworksTimelineError::InvalidString { field: "title" })
        );
    }

    #[test]
    fn validation_rejects_non_finite_values() {
        let command = SteamworksTimelineCommand::add_event(SteamworksTimelineEventInfo {
            icon: "star".to_owned(),
            title: "win".to_owned(),
            description: "match won".to_owned(),
            priority: 1,
            start_offset_seconds: f32::NAN,
            duration: Duration::from_secs(1),
            clip_priority: SteamworksTimelineEventClipPriority::Standard,
        });

        assert_eq!(
            validate_command(&command),
            Err(SteamworksTimelineError::InvalidFloat {
                field: "start_offset_seconds",
            })
        );
    }

    #[test]
    fn state_tracks_last_event_and_count_without_unbounded_history() {
        let mut state = SteamworksTimelineState::default();
        let first = SteamworksTimelineEventInfo {
            icon: "first".to_owned(),
            title: "first title".to_owned(),
            description: "first description".to_owned(),
            priority: 1,
            start_offset_seconds: 0.0,
            duration: Duration::ZERO,
            clip_priority: SteamworksTimelineEventClipPriority::Standard,
        };
        let second = SteamworksTimelineEventInfo {
            icon: "second".to_owned(),
            title: "second title".to_owned(),
            description: "second description".to_owned(),
            priority: 2,
            start_offset_seconds: -1.0,
            duration: Duration::from_secs(2),
            clip_priority: SteamworksTimelineEventClipPriority::Featured,
        };

        state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded { event: first });
        state.record_operation(&SteamworksTimelineOperation::TimelineEventAdded {
            event: second.clone(),
        });

        assert_eq!(state.event_count(), 2);
        assert_eq!(state.last_event(), Some(&second));
    }
}
