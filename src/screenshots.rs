//! High-level Bevy ECS integration for Steam screenshots.
//!
//! This module builds on top of the upstream [`steamworks::screenshots::Screenshots`] API.
//! It submits screenshot operations through Bevy messages while mirroring final
//! Steam callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksScreenshotsResult`].

use std::path::PathBuf;

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

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

/// Runtime state for [`SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksScreenshotsState {
    last_error: Option<SteamworksScreenshotsError>,
    screenshots_hooked: Option<bool>,
    added_screenshots: Vec<steamworks::screenshots::ScreenshotHandle>,
    submitted_screenshots: Vec<SteamworksSubmittedScreenshot>,
    screenshot_trigger_count: u64,
    screenshot_requested_count: u64,
    last_screenshot_ready: Option<SteamworksScreenshotReady>,
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
    /// Final save confirmation arrives later through both
    /// [`crate::SteamworksEvent::ScreenshotReady`] and
    /// [`SteamworksScreenshotsOperation::ScreenshotReady`].
    pub fn added_screenshots(&self) -> &[steamworks::screenshots::ScreenshotHandle] {
        &self.added_screenshots
    }

    /// Returns screenshot library submissions accepted by Steam through this command layer.
    pub fn submitted_screenshots(&self) -> &[SteamworksSubmittedScreenshot] {
        &self.submitted_screenshots
    }

    /// Returns one submitted screenshot snapshot by Steam screenshot handle.
    pub fn submitted_screenshot(
        &self,
        handle: steamworks::screenshots::ScreenshotHandle,
    ) -> Option<&SteamworksSubmittedScreenshot> {
        self.submitted_screenshots
            .iter()
            .find(|submission| submission.handle == handle)
    }

    /// Returns the most recent screenshot library submission accepted by Steam.
    pub fn last_submitted_screenshot(&self) -> Option<&SteamworksSubmittedScreenshot> {
        self.submitted_screenshots.last()
    }

    /// Returns how many trigger screenshot commands this plugin successfully submitted.
    pub fn screenshot_trigger_count(&self) -> u64 {
        self.screenshot_trigger_count
    }

    /// Returns how many screenshot requested callbacks this plugin observed.
    pub fn screenshot_requested_count(&self) -> u64 {
        self.screenshot_requested_count
    }

    /// Returns the most recent screenshot ready callback snapshot.
    pub fn last_screenshot_ready(&self) -> Option<&SteamworksScreenshotReady> {
        self.last_screenshot_ready.as_ref()
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
            SteamworksScreenshotsOperation::ScreenshotTriggered => {
                self.screenshot_trigger_count = self.screenshot_trigger_count.saturating_add(1);
            }
            SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
                handle,
                filename,
                thumbnail_filename,
                width,
                height,
            } => {
                if !self.added_screenshots.contains(handle) {
                    self.added_screenshots.push(*handle);
                }
                upsert_submitted_screenshot(
                    &mut self.submitted_screenshots,
                    SteamworksSubmittedScreenshot {
                        handle: *handle,
                        filename: filename.clone(),
                        thumbnail_filename: thumbnail_filename.clone(),
                        width: *width,
                        height: *height,
                    },
                );
            }
            SteamworksScreenshotsOperation::ScreenshotRequested { count } => {
                self.screenshot_requested_count = *count;
            }
            SteamworksScreenshotsOperation::ScreenshotReady { ready } => {
                self.last_screenshot_ready = Some(ready.clone());
            }
        }
    }
}

/// Screenshot library submission accepted by Steam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksSubmittedScreenshot {
    /// Steam screenshot handle.
    pub handle: steamworks::screenshots::ScreenshotHandle,
    /// Screenshot image file path submitted.
    pub filename: PathBuf,
    /// Optional thumbnail image file path submitted.
    pub thumbnail_filename: Option<PathBuf>,
    /// Screenshot width in pixels.
    pub width: i32,
    /// Screenshot height in pixels.
    pub height: i32,
}

/// Screenshot ready callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksScreenshotReady {
    /// Screenshot handle, or the error reported by Steam.
    pub local_handle:
        Result<steamworks::screenshots::ScreenshotHandle, SteamworksScreenshotReadyError>,
}

/// A high-level command for Steam screenshot workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsCommand {
    /// Set whether this app handles Steam screenshot requests itself.
    ///
    /// When enabled, Steam emits [`crate::SteamworksEvent::ScreenshotRequested`]
    /// and [`SteamworksScreenshotsOperation::ScreenshotRequested`], and the game
    /// is expected to capture and submit a screenshot.
    HookScreenshots {
        /// Whether screenshots should be hooked by the app.
        hook: bool,
    },
    /// Read whether this app is currently hooking Steam screenshots.
    IsScreenshotsHooked,
    /// Trigger a Steam screenshot.
    ///
    /// Depending on hook state, Steam may emit
    /// [`crate::SteamworksEvent::ScreenshotRequested`] /
    /// [`SteamworksScreenshotsOperation::ScreenshotRequested`] and later
    /// [`crate::SteamworksEvent::ScreenshotReady`] /
    /// [`SteamworksScreenshotsOperation::ScreenshotReady`].
    TriggerScreenshot,
    /// Add an existing screenshot image file to the user's Steam screenshot library.
    ///
    /// This submits the request and returns a handle immediately if Steam accepts
    /// it. Final save confirmation arrives later through both
    /// [`crate::SteamworksEvent::ScreenshotReady`] and
    /// [`SteamworksScreenshotsOperation::ScreenshotReady`].
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
    /// Final save confirmation arrives later through both
    /// [`crate::SteamworksEvent::ScreenshotReady`] and
    /// [`SteamworksScreenshotsOperation::ScreenshotReady`].
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
    /// Steam requested a screenshot from this app.
    ScreenshotRequested {
        /// Total number of screenshot request callbacks observed by this plugin.
        count: u64,
    },
    /// Steam reported a screenshot ready result.
    ScreenshotReady {
        /// Callback snapshot.
        ready: SteamworksScreenshotReady,
    },
}

/// Result message emitted by [`SteamworksScreenshotsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksScreenshotsResult {
    /// The command or observed callback was processed successfully.
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

/// Cloneable, comparable mirror of upstream
/// [`steamworks::screenshots::ScreenshotReadyError`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum SteamworksScreenshotReadyError {
    /// The screenshot could not be loaded or parsed.
    #[error("the screenshot could not be loaded or parsed")]
    Fail,
    /// The screenshot could not be saved to disk.
    #[error("the screenshot could not be saved to disk")]
    IoFailure,
}

impl From<steamworks::screenshots::ScreenshotReadyError> for SteamworksScreenshotReadyError {
    fn from(error: steamworks::screenshots::ScreenshotReadyError) -> Self {
        match error {
            steamworks::screenshots::ScreenshotReadyError::Fail => Self::Fail,
            steamworks::screenshots::ScreenshotReadyError::IoFailure => Self::IoFailure,
        }
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
                    count: state.screenshot_requested_count.saturating_add(1),
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

fn upsert_submitted_screenshot(
    submissions: &mut Vec<SteamworksSubmittedScreenshot>,
    submission: SteamworksSubmittedScreenshot,
) {
    if let Some(index) = submissions
        .iter()
        .position(|existing| existing.handle == submission.handle)
    {
        submissions.remove(index);
    }

    submissions.push(submission);
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
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
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

    #[test]
    fn screenshot_ready_errors_are_cloneable_and_comparable() {
        assert_eq!(
            SteamworksScreenshotReadyError::from(
                steamworks::screenshots::ScreenshotReadyError::Fail
            ),
            SteamworksScreenshotReadyError::Fail
        );
        assert_eq!(
            SteamworksScreenshotReadyError::from(
                steamworks::screenshots::ScreenshotReadyError::IoFailure
            ),
            SteamworksScreenshotReadyError::IoFailure
        );
    }

    #[test]
    fn state_records_screenshot_operations() {
        let mut state = SteamworksScreenshotsState::default();
        let first_submission = SteamworksSubmittedScreenshot {
            handle: 11,
            filename: PathBuf::from("first.png"),
            thumbnail_filename: Some(PathBuf::from("first_thumb.png")),
            width: 1920,
            height: 1080,
        };
        let updated_submission = SteamworksSubmittedScreenshot {
            handle: 11,
            filename: PathBuf::from("updated.png"),
            thumbnail_filename: None,
            width: 1280,
            height: 720,
        };
        let second_submission = SteamworksSubmittedScreenshot {
            handle: 22,
            filename: PathBuf::from("second.png"),
            thumbnail_filename: None,
            width: 800,
            height: 600,
        };

        state.record_operation(&SteamworksScreenshotsOperation::ScreenshotsHookSet { hook: true });
        state.record_operation(&SteamworksScreenshotsOperation::ScreenshotsHookedRead {
            hooked: false,
        });
        state.record_operation(&SteamworksScreenshotsOperation::ScreenshotTriggered);
        state.record_operation(&SteamworksScreenshotsOperation::ScreenshotTriggered);
        state.record_operation(
            &SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
                handle: first_submission.handle,
                filename: first_submission.filename.clone(),
                thumbnail_filename: first_submission.thumbnail_filename.clone(),
                width: first_submission.width,
                height: first_submission.height,
            },
        );
        state.record_operation(
            &SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
                handle: second_submission.handle,
                filename: second_submission.filename.clone(),
                thumbnail_filename: second_submission.thumbnail_filename.clone(),
                width: second_submission.width,
                height: second_submission.height,
            },
        );
        state.record_operation(
            &SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
                handle: updated_submission.handle,
                filename: updated_submission.filename.clone(),
                thumbnail_filename: updated_submission.thumbnail_filename.clone(),
                width: updated_submission.width,
                height: updated_submission.height,
            },
        );
        state.record_operation(&SteamworksScreenshotsOperation::ScreenshotRequested { count: 3 });
        state.record_operation(&SteamworksScreenshotsOperation::ScreenshotReady {
            ready: SteamworksScreenshotReady {
                local_handle: Ok(updated_submission.handle),
            },
        });

        assert_eq!(state.screenshots_hooked(), Some(false));
        assert_eq!(state.screenshot_trigger_count(), 2);
        assert_eq!(
            state.added_screenshots(),
            &[updated_submission.handle, second_submission.handle]
        );
        assert_eq!(
            state.submitted_screenshots(),
            &[second_submission.clone(), updated_submission.clone()]
        );
        assert_eq!(
            state.submitted_screenshot(updated_submission.handle),
            Some(&updated_submission)
        );
        assert_eq!(state.last_submitted_screenshot(), Some(&updated_submission));
        assert_eq!(state.screenshot_requested_count(), 3);
        assert_eq!(
            state.last_screenshot_ready(),
            Some(&SteamworksScreenshotReady {
                local_handle: Ok(updated_submission.handle),
            })
        );
    }

    #[test]
    fn screenshot_callbacks_are_bridged_without_client() {
        let mut app = App::new();
        let handle = 7;

        app.add_plugins(SteamworksScreenshotsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ScreenshotRequested(
                steamworks::screenshots::ScreenshotRequested,
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ScreenshotRequested(
                steamworks::screenshots::ScreenshotRequested,
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ScreenshotReady(
                steamworks::screenshots::ScreenshotReady {
                    local_handle: Ok(handle),
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ScreenshotReady(
                steamworks::screenshots::ScreenshotReady {
                    local_handle: Err(steamworks::screenshots::ScreenshotReadyError::IoFailure),
                },
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksScreenshotsResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        assert_eq!(
            drained,
            vec![
                SteamworksScreenshotsResult::Ok(
                    SteamworksScreenshotsOperation::ScreenshotRequested { count: 1 },
                ),
                SteamworksScreenshotsResult::Ok(
                    SteamworksScreenshotsOperation::ScreenshotRequested { count: 2 },
                ),
                SteamworksScreenshotsResult::Ok(SteamworksScreenshotsOperation::ScreenshotReady {
                    ready: SteamworksScreenshotReady {
                        local_handle: Ok(handle),
                    },
                },),
                SteamworksScreenshotsResult::Ok(SteamworksScreenshotsOperation::ScreenshotReady {
                    ready: SteamworksScreenshotReady {
                        local_handle: Err(SteamworksScreenshotReadyError::IoFailure),
                    },
                },),
            ]
        );

        let state = app.world().resource::<SteamworksScreenshotsState>();
        assert_eq!(state.screenshot_requested_count(), 2);
        assert_eq!(
            state.last_screenshot_ready(),
            Some(&SteamworksScreenshotReady {
                local_handle: Err(SteamworksScreenshotReadyError::IoFailure),
            })
        );
        assert_eq!(state.last_error(), None);
    }
}
