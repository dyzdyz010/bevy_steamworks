//! High-level Bevy ECS integration for Steam utility queries and overlay helpers.
//!
//! This module builds on top of the upstream [`steamworks::Utils`] API. It keeps
//! common utility calls in Bevy messages and leaves Steam callback delivery in
//! [`crate::SteamworksEvent`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Bevy plugin for high-level Steam utility commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUtilsCommand`] and [`SteamworksUtilsResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUtilsPlugin;

impl SteamworksUtilsPlugin {
    /// Creates a utils plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUtilsState>()
            .add_message::<SteamworksUtilsCommand>()
            .add_message::<SteamworksUtilsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUtilsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_utils_commands.in_set(SteamworksSystem::ProcessUtilsCommands),
            );
    }
}

/// Runtime state for [`SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUtilsState {
    last_error: Option<SteamworksUtilsError>,
    current_info: Option<SteamworksUtilsInfo>,
    overlay_notification_position: Option<SteamworksNotificationPosition>,
}

impl SteamworksUtilsState {
    /// Returns the most recent synchronous error observed by the utils plugin.
    pub fn last_error(&self) -> Option<&SteamworksUtilsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent utility snapshot read through the plugin.
    pub fn current_info(&self) -> Option<&SteamworksUtilsInfo> {
        self.current_info.as_ref()
    }

    /// Returns the most recent overlay notification position submitted through this plugin.
    pub fn overlay_notification_position(&self) -> Option<SteamworksNotificationPosition> {
        self.overlay_notification_position
    }

    fn record_error(&mut self, error: SteamworksUtilsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksUtilsOperation) {
        match operation {
            SteamworksUtilsOperation::CurrentInfoRead { info } => {
                self.current_info = Some(info.clone());
            }
            SteamworksUtilsOperation::OverlayNotificationPositionSet { position } => {
                self.overlay_notification_position = Some(*position);
            }
            _ => {}
        }
    }
}

/// Snapshot of common Steam utility information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUtilsInfo {
    /// Current Steam app ID.
    pub app_id: steamworks::AppId,
    /// Country code inferred by Steam from the current user's IP address.
    pub ip_country: String,
    /// Whether the Steam overlay is enabled.
    pub overlay_enabled: bool,
    /// Current Steam client UI language.
    pub ui_language: String,
    /// Steam server real time as Unix epoch seconds.
    pub server_real_time: u32,
    /// Whether Steam and the Steam overlay are running in Big Picture mode.
    pub steam_in_big_picture_mode: bool,
    /// Whether Steam reports that it is running on a Steam Deck device.
    pub steam_running_on_steam_deck: bool,
}

/// Overlay popup position used by Steam notifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksNotificationPosition {
    /// Top-left corner.
    TopLeft,
    /// Top-right corner.
    TopRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom-right corner.
    BottomRight,
}

impl SteamworksNotificationPosition {
    fn to_steam(self) -> steamworks::NotificationPosition {
        match self {
            Self::TopLeft => steamworks::NotificationPosition::TopLeft,
            Self::TopRight => steamworks::NotificationPosition::TopRight,
            Self::BottomLeft => steamworks::NotificationPosition::BottomLeft,
            Self::BottomRight => steamworks::NotificationPosition::BottomRight,
        }
    }
}

/// A high-level command for Steam utility queries and overlay helpers.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUtilsCommand {
    /// Read a snapshot of common Steam utility information.
    GetCurrentInfo,
    /// Read the current Steam app ID.
    GetAppId,
    /// Read the country code inferred by Steam from the current user's IP address.
    GetIpCountry,
    /// Read whether the Steam overlay is enabled.
    IsOverlayEnabled,
    /// Read the current Steam client UI language.
    GetUiLanguage,
    /// Read Steam server real time as Unix epoch seconds.
    GetServerRealTime,
    /// Read whether Steam and the Steam overlay are running in Big Picture mode.
    IsSteamInBigPictureMode,
    /// Read whether Steam reports that it is running on a Steam Deck device.
    IsSteamRunningOnSteamDeck,
    /// Set where Steam overlay notification popups should appear.
    SetOverlayNotificationPosition {
        /// Popup position to submit to Steam.
        position: SteamworksNotificationPosition,
    },
}

impl SteamworksUtilsCommand {
    /// Creates a [`SteamworksUtilsCommand::SetOverlayNotificationPosition`] command.
    pub fn set_overlay_notification_position(position: SteamworksNotificationPosition) -> Self {
        Self::SetOverlayNotificationPosition { position }
    }
}

/// A successfully submitted Steam utility operation or synchronous read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksUtilsOperation {
    /// Common Steam utility information was read.
    CurrentInfoRead {
        /// Utility snapshot.
        info: SteamworksUtilsInfo,
    },
    /// Current Steam app ID was read.
    AppIdRead {
        /// Current Steam app ID.
        app_id: steamworks::AppId,
    },
    /// Steam IP country code was read.
    IpCountryRead {
        /// Country code reported by Steam.
        country: String,
    },
    /// Steam overlay enabled state was read.
    OverlayEnabledRead {
        /// Whether the Steam overlay is enabled.
        enabled: bool,
    },
    /// Steam UI language was read.
    UiLanguageRead {
        /// Current Steam client UI language.
        language: String,
    },
    /// Steam server real time was read.
    ServerRealTimeRead {
        /// Unix epoch seconds reported by Steam.
        unix_epoch_seconds: u32,
    },
    /// Big Picture mode state was read.
    SteamInBigPictureModeRead {
        /// Whether Steam and the overlay are running in Big Picture mode.
        enabled: bool,
    },
    /// Steam Deck state was read.
    SteamRunningOnSteamDeckRead {
        /// Whether Steam reports it is running on Steam Deck.
        enabled: bool,
    },
    /// Overlay notification popup position was set.
    OverlayNotificationPositionSet {
        /// Position submitted to Steam.
        position: SteamworksNotificationPosition,
    },
}

/// Result message emitted by [`SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUtilsResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksUtilsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksUtilsCommand,
        /// Failure reason.
        error: SteamworksUtilsError,
    },
}

/// Synchronous errors from [`SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUtilsError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
}

fn process_utils_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksUtilsState>,
    mut commands: ResMut<Messages<SteamworksUtilsCommand>>,
    mut results: MessageWriter<SteamworksUtilsResult>,
) {
    let Some(client) = client else {
        let error = SteamworksUtilsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks utils command failed"
            );
            results.write(SteamworksUtilsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_utils_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks utils command"
                );
                results.write(SteamworksUtilsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks utils command failed"
                );
                results.write(SteamworksUtilsResult::Err { command, error });
            }
        }
    }
}

fn handle_utils_command(
    client: &SteamworksClient,
    command: &SteamworksUtilsCommand,
) -> Result<SteamworksUtilsOperation, SteamworksUtilsError> {
    validate_command(command)?;

    let utils = client.utils();
    Ok(match command {
        SteamworksUtilsCommand::GetCurrentInfo => SteamworksUtilsOperation::CurrentInfoRead {
            info: snapshot_utils_info(client),
        },
        SteamworksUtilsCommand::GetAppId => SteamworksUtilsOperation::AppIdRead {
            app_id: utils.app_id(),
        },
        SteamworksUtilsCommand::GetIpCountry => SteamworksUtilsOperation::IpCountryRead {
            country: utils.ip_country(),
        },
        SteamworksUtilsCommand::IsOverlayEnabled => SteamworksUtilsOperation::OverlayEnabledRead {
            enabled: utils.is_overlay_enabled(),
        },
        SteamworksUtilsCommand::GetUiLanguage => SteamworksUtilsOperation::UiLanguageRead {
            language: utils.ui_language(),
        },
        SteamworksUtilsCommand::GetServerRealTime => SteamworksUtilsOperation::ServerRealTimeRead {
            unix_epoch_seconds: utils.get_server_real_time(),
        },
        SteamworksUtilsCommand::IsSteamInBigPictureMode => {
            SteamworksUtilsOperation::SteamInBigPictureModeRead {
                enabled: utils.is_steam_in_big_picture_mode(),
            }
        }
        SteamworksUtilsCommand::IsSteamRunningOnSteamDeck => {
            SteamworksUtilsOperation::SteamRunningOnSteamDeckRead {
                enabled: utils.is_steam_running_on_steam_deck(),
            }
        }
        SteamworksUtilsCommand::SetOverlayNotificationPosition { position } => {
            utils.set_overlay_notification_position(position.to_steam());
            SteamworksUtilsOperation::OverlayNotificationPositionSet {
                position: *position,
            }
        }
    })
}

fn snapshot_utils_info(client: &SteamworksClient) -> SteamworksUtilsInfo {
    let utils = client.utils();
    SteamworksUtilsInfo {
        app_id: utils.app_id(),
        ip_country: utils.ip_country(),
        overlay_enabled: utils.is_overlay_enabled(),
        ui_language: utils.ui_language(),
        server_real_time: utils.get_server_real_time(),
        steam_in_big_picture_mode: utils.is_steam_in_big_picture_mode(),
        steam_running_on_steam_deck: utils.is_steam_running_on_steam_deck(),
    }
}

fn validate_command(_command: &SteamworksUtilsCommand) -> Result<(), SteamworksUtilsError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn utils_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksUtilsPlugin::new());

        assert!(app.world().contains_resource::<SteamworksUtilsState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUtilsCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUtilsResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksUtilsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksUtilsCommand>>()
            .write(SteamworksUtilsCommand::GetCurrentInfo);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksUtilsResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksUtilsResult::Err {
                command: SteamworksUtilsCommand::GetCurrentInfo,
                error: SteamworksUtilsError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksUtilsState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksUtilsError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_accepts_all_current_commands() {
        assert_eq!(
            validate_command(&SteamworksUtilsCommand::GetCurrentInfo),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksUtilsCommand::set_overlay_notification_position(
                SteamworksNotificationPosition::BottomRight
            )),
            Ok(())
        );
    }
}
