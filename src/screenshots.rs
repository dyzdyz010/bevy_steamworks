//! High-level Bevy ECS integration for Steam screenshots.
//!
//! This module builds on top of the upstream [`steamworks::screenshots::Screenshots`] API.
//! It submits screenshot operations through Bevy messages while keeping final
//! Steam callback confirmation in [`crate::SteamworksEvent`].

use std::path::PathBuf;

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Bevy plugin for high-level Steam screenshot commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksScreenshotsCommand`] and [`SteamworksScreenshotsResult`] messages
/// and runs its command processor in [`bevy_app::First`] after Steam callbacks.
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

/// Runtime state for [`SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksScreenshotsState {
    last_error: Option<SteamworksScreenshotsError>,
    screenshots_hooked: Option<bool>,
    added_screenshots: Vec<steamworks::screenshots::ScreenshotHandle>,
}

impl SteamworksScreenshotsState {
    /// Returns the most recent synchronous error observed by the screenshots plugin.
    pub fn last_error(&self) -> Option<&SteamworksScreenshotsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent screenshot hook state observed or submitted.
    pub fn screenshots_hooked(&self) -> Option<bool> {
        self.screenshots_hooked
    }

    /// Returns screenshot handles successfully submitted through this command layer.
    ///
    /// Final save confirmation arrives later through
    /// [`crate::SteamworksEvent::ScreenshotReady`].
    pub fn added_screenshots(&self) -> &[steamworks::screenshots::ScreenshotHandle] {
        &self.added_screenshots
    }

    fn record_error(&mut self, error: SteamworksScreenshotsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksScreenshotsOperation) {
        match operation {
            SteamworksScreenshotsOperation::ScreenshotsHookSet { hook }
            | SteamworksScreenshotsOperation::ScreenshotsHookedRead { hooked: hook } => {
                self.screenshots_hooked = Some(*hook);
            }
            SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted { handle, .. }
                if !self.added_screenshots.contains(handle) =>
            {
                self.added_screenshots.push(*handle);
            }
            _ => {}
        }
    }
}

/// A high-level command for Steam screenshot workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsCommand {
    /// Set whether this app handles Steam screenshot requests itself.
    ///
    /// When enabled, Steam emits [`crate::SteamworksEvent::ScreenshotRequested`]
    /// and the game is expected to capture and submit a screenshot.
    HookScreenshots {
        /// Whether screenshots should be hooked by the app.
        hook: bool,
    },
    /// Read whether this app is currently hooking Steam screenshots.
    IsScreenshotsHooked,
    /// Trigger a Steam screenshot.
    ///
    /// Depending on hook state, Steam may emit
    /// [`crate::SteamworksEvent::ScreenshotRequested`] and later
    /// [`crate::SteamworksEvent::ScreenshotReady`].
    TriggerScreenshot,
    /// Add an existing screenshot image file to the user's Steam screenshot library.
    ///
    /// This submits the request and returns a handle immediately if Steam accepts
    /// it. Final save confirmation arrives later through
    /// [`crate::SteamworksEvent::ScreenshotReady`].
    ///
    /// The upstream wrapper canonicalizes the provided paths before submitting
    /// them to Steam, so use local paths and keep this command low-frequency.
    AddScreenshotToLibrary {
        /// Screenshot image file path.
        filename: PathBuf,
        /// Optional thumbnail image file path.
        thumbnail_filename: Option<PathBuf>,
        /// Screenshot width in pixels.
        width: i32,
        /// Screenshot height in pixels.
        height: i32,
    },
}

impl SteamworksScreenshotsCommand {
    /// Creates a [`SteamworksScreenshotsCommand::HookScreenshots`] command.
    pub fn hook_screenshots(hook: bool) -> Self {
        Self::HookScreenshots { hook }
    }

    /// Creates a [`SteamworksScreenshotsCommand::AddScreenshotToLibrary`] command.
    pub fn add_screenshot_to_library(
        filename: impl Into<PathBuf>,
        thumbnail_filename: Option<impl Into<PathBuf>>,
        width: i32,
        height: i32,
    ) -> Self {
        Self::AddScreenshotToLibrary {
            filename: filename.into(),
            thumbnail_filename: thumbnail_filename.map(Into::into),
            width,
            height,
        }
    }
}

/// A successfully submitted Steam screenshot operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksScreenshotsOperation {
    /// Screenshot hook state was set.
    ScreenshotsHookSet {
        /// Hook state submitted to Steam.
        hook: bool,
    },
    /// Screenshot hook state was read.
    ScreenshotsHookedRead {
        /// Whether screenshots are hooked by the app.
        hooked: bool,
    },
    /// A Steam screenshot was triggered.
    ScreenshotTriggered,
    /// A screenshot library add request was accepted by Steam.
    ///
    /// Final save confirmation arrives later through
    /// [`crate::SteamworksEvent::ScreenshotReady`].
    ScreenshotLibraryAddSubmitted {
        /// Steam screenshot handle.
        handle: steamworks::screenshots::ScreenshotHandle,
        /// Screenshot image file path submitted.
        filename: PathBuf,
        /// Optional thumbnail image file path submitted.
        thumbnail_filename: Option<PathBuf>,
        /// Screenshot width in pixels.
        width: i32,
        /// Screenshot height in pixels.
        height: i32,
    },
}

/// Result message emitted by [`SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksScreenshotsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksScreenshotsCommand,
        /// Failure reason.
        error: SteamworksScreenshotsError,
    },
}

/// Synchronous errors from [`SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksScreenshotsError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// Screenshot dimensions must be positive.
    #[error("Steamworks screenshot dimensions must be positive, got {width}x{height}")]
    InvalidDimensions {
        /// Requested width.
        width: i32,
        /// Requested height.
        height: i32,
    },
    /// The upstream Steamworks API rejected the screenshot library add.
    #[error("Steamworks screenshot library add failed: {source}")]
    LibraryAddFailed {
        /// Failure reported by the upstream Steamworks wrapper.
        #[source]
        source: SteamworksScreenshotLibraryError,
    },
}

impl SteamworksScreenshotsError {
    fn library_add_failed(source: steamworks::screenshots::ScreenshotLibraryAddError) -> Self {
        Self::LibraryAddFailed {
            source: source.into(),
        }
    }
}

/// Cloneable, comparable mirror of upstream
/// [`steamworks::screenshots::ScreenshotLibraryAddError`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum SteamworksScreenshotLibraryError {
    /// Steam failed to save the screenshot file for an unspecified reason.
    #[error("the screenshot file could not be saved")]
    SavingFailed,
    /// One of the provided paths was invalid or could not be canonicalized.
    #[error("invalid screenshot path")]
    InvalidPath,
}

impl From<steamworks::screenshots::ScreenshotLibraryAddError> for SteamworksScreenshotLibraryError {
    fn from(error: steamworks::screenshots::ScreenshotLibraryAddError) -> Self {
        match error {
            steamworks::screenshots::ScreenshotLibraryAddError::SavingFailed => Self::SavingFailed,
            steamworks::screenshots::ScreenshotLibraryAddError::InvalidPath => Self::InvalidPath,
        }
    }
}

fn process_screenshots_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksScreenshotsState>,
    mut commands: ResMut<Messages<SteamworksScreenshotsCommand>>,
    mut results: MessageWriter<SteamworksScreenshotsResult>,
) {
    let Some(client) = client else {
        let error = SteamworksScreenshotsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
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

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn screenshots_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksScreenshotsPlugin::new());

        assert!(app
            .world()
            .contains_resource::<SteamworksScreenshotsState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksScreenshotsCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksScreenshotsResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksScreenshotsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksScreenshotsCommand>>()
            .write(SteamworksScreenshotsCommand::IsScreenshotsHooked);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksScreenshotsResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksScreenshotsResult::Err {
                command: SteamworksScreenshotsCommand::IsScreenshotsHooked,
                error: SteamworksScreenshotsError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksScreenshotsState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksScreenshotsError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_invalid_dimensions() {
        let command = SteamworksScreenshotsCommand::add_screenshot_to_library(
            "shot.png",
            None::<&str>,
            0,
            720,
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksScreenshotsError::InvalidDimensions {
                width: 0,
                height: 720,
            })
        );
    }

    #[test]
    fn screenshot_library_errors_are_cloneable_and_comparable() {
        assert_eq!(
            SteamworksScreenshotLibraryError::from(
                steamworks::screenshots::ScreenshotLibraryAddError::SavingFailed
            ),
            SteamworksScreenshotLibraryError::SavingFailed
        );
        assert_eq!(
            SteamworksScreenshotLibraryError::from(
                steamworks::screenshots::ScreenshotLibraryAddError::InvalidPath
            ),
            SteamworksScreenshotLibraryError::InvalidPath
        );
    }
}
