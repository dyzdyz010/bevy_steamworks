//! High-level Bevy ECS integration for Steam screenshots.
//!
//! This module builds on top of the upstream [`steamworks::screenshots::Screenshots`] API.
//! It submits screenshot operations through Bevy messages while mirroring final
//! Steam callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksScreenshotsResult`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksScreenshotsState;
pub use types::*;

/// Bevy plugin for high-level Steam screenshot commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksScreenshotsCommand`] and [`SteamworksScreenshotsResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
/// It also mirrors screenshot requested/ready callbacks into screenshot results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksScreenshotsPlugin;

impl SteamworksScreenshotsPlugin {
    /// Creates a screenshots plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksScreenshotsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksScreenshotsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksScreenshotsCommand>()
            .add_message::<SteamworksScreenshotsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessScreenshotsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_screenshots_commands.in_set(SteamworksSystem::ProcessScreenshotsCommands),
            );
    }
}
fn process_screenshots_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksScreenshotsState>,
    mut commands: ResMut<Messages<SteamworksScreenshotsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksScreenshotsResult>,
) {
    process_screenshots_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksScreenshotsError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks screenshots command failed"
            );
            results.write(SteamworksScreenshotsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_screenshots_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks screenshots command"
                );
                results.write(SteamworksScreenshotsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks screenshots command failed"
                );
                results.write(SteamworksScreenshotsResult::Err { command, error });
            }
        }
    }
}

fn process_screenshots_steam_events(
    state: &mut SteamworksScreenshotsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksScreenshotsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::ScreenshotRequested(_) => {
                SteamworksScreenshotsOperation::ScreenshotRequested {
                    count: state.screenshot_requested_count().saturating_add(1),
                }
            }
            SteamworksEvent::ScreenshotReady(event) => {
                SteamworksScreenshotsOperation::ScreenshotReady {
                    ready: SteamworksScreenshotReady {
                        local_handle: event.local_handle.clone().map_err(Into::into),
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks screenshots callback"
        );
        results.write(SteamworksScreenshotsResult::Ok(operation));
    }
}

fn handle_screenshots_command(
    client: &SteamworksClient,
    command: &SteamworksScreenshotsCommand,
) -> Result<SteamworksScreenshotsOperation, SteamworksScreenshotsError> {
    validate_command(command)?;

    let screenshots = client.screenshots();
    match command {
        SteamworksScreenshotsCommand::HookScreenshots { hook } => {
            screenshots.hook_screenshots(*hook);
            Ok(SteamworksScreenshotsOperation::ScreenshotsHookSet { hook: *hook })
        }
        SteamworksScreenshotsCommand::IsScreenshotsHooked => {
            Ok(SteamworksScreenshotsOperation::ScreenshotsHookedRead {
                hooked: screenshots.is_screenshots_hooked(),
            })
        }
        SteamworksScreenshotsCommand::TriggerScreenshot => {
            screenshots.trigger_screenshot();
            Ok(SteamworksScreenshotsOperation::ScreenshotTriggered)
        }
        SteamworksScreenshotsCommand::AddScreenshotToLibrary {
            filename,
            thumbnail_filename,
            width,
            height,
        } => screenshots
            .add_screenshot_to_library(filename, thumbnail_filename.as_deref(), *width, *height)
            .map(
                |handle| SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
                    handle,
                    filename: filename.clone(),
                    thumbnail_filename: thumbnail_filename.clone(),
                    width: *width,
                    height: *height,
                },
            )
            .map_err(SteamworksScreenshotsError::library_add_failed),
    }
}

fn validate_command(
    command: &SteamworksScreenshotsCommand,
) -> Result<(), SteamworksScreenshotsError> {
    if let SteamworksScreenshotsCommand::AddScreenshotToLibrary { width, height, .. } = command {
        if *width <= 0 || *height <= 0 {
            return Err(SteamworksScreenshotsError::InvalidDimensions {
                width: *width,
                height: *height,
            });
        }
    }

    Ok(())
}
