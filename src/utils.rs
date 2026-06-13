//! High-level Bevy ECS integration for Steam utility queries and overlay helpers.
//!
//! This module builds on top of the upstream [`steamworks::Utils`] API. It keeps
//! common utility calls in Bevy messages while mirroring text-input dismissal
//! callbacks from [`crate::SteamworksEvent`] into [`SteamworksUtilsResult`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Bevy plugin for high-level Steam utility commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUtilsCommand`] and [`SteamworksUtilsResult`] messages and runs
/// its command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors gamepad text input dismissal callbacks into utils results.
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
            .add_message::<SteamworksEvent>()
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
    app_id: Option<steamworks::AppId>,
    ip_country: Option<String>,
    overlay_enabled: Option<bool>,
    ui_language: Option<String>,
    server_real_time: Option<u32>,
    steam_in_big_picture_mode: Option<bool>,
    steam_running_on_steam_deck: Option<bool>,
    overlay_notification_position: Option<SteamworksNotificationPosition>,
    last_gamepad_text_input_dismissed: Option<SteamworksGamepadTextInputDismissed>,
    last_floating_gamepad_text_input_dismissed: Option<SteamworksFloatingGamepadTextInputDismissed>,
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

    /// Returns the most recent Steam app ID read through this plugin.
    pub fn app_id(&self) -> Option<steamworks::AppId> {
        self.app_id
    }

    /// Returns the most recent Steam IP country code read through this plugin.
    pub fn ip_country(&self) -> Option<&str> {
        self.ip_country.as_deref()
    }

    /// Returns the most recent Steam overlay enabled state read through this plugin.
    pub fn overlay_enabled(&self) -> Option<bool> {
        self.overlay_enabled
    }

    /// Returns the most recent Steam UI language read through this plugin.
    pub fn ui_language(&self) -> Option<&str> {
        self.ui_language.as_deref()
    }

    /// Returns the most recent Steam server real time read through this plugin.
    pub fn server_real_time(&self) -> Option<u32> {
        self.server_real_time
    }

    /// Returns the most recent Steam Big Picture mode state read through this plugin.
    pub fn steam_in_big_picture_mode(&self) -> Option<bool> {
        self.steam_in_big_picture_mode
    }

    /// Returns the most recent Steam Deck state read through this plugin.
    pub fn steam_running_on_steam_deck(&self) -> Option<bool> {
        self.steam_running_on_steam_deck
    }

    /// Returns the most recent overlay notification position submitted through this plugin.
    pub fn overlay_notification_position(&self) -> Option<SteamworksNotificationPosition> {
        self.overlay_notification_position
    }

    /// Returns the most recent gamepad text input dismissal callback snapshot.
    pub fn last_gamepad_text_input_dismissed(
        &self,
    ) -> Option<&SteamworksGamepadTextInputDismissed> {
        self.last_gamepad_text_input_dismissed.as_ref()
    }

    /// Returns the most recent floating gamepad text input dismissal callback snapshot.
    pub fn last_floating_gamepad_text_input_dismissed(
        &self,
    ) -> Option<&SteamworksFloatingGamepadTextInputDismissed> {
        self.last_floating_gamepad_text_input_dismissed.as_ref()
    }

    fn record_error(&mut self, error: SteamworksUtilsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksUtilsOperation) {
        match operation {
            SteamworksUtilsOperation::CurrentInfoRead { info } => {
                self.current_info = Some(info.clone());
                self.app_id = Some(info.app_id);
                self.ip_country = Some(info.ip_country.clone());
                self.overlay_enabled = Some(info.overlay_enabled);
                self.ui_language = Some(info.ui_language.clone());
                self.server_real_time = Some(info.server_real_time);
                self.steam_in_big_picture_mode = Some(info.steam_in_big_picture_mode);
                self.steam_running_on_steam_deck = Some(info.steam_running_on_steam_deck);
            }
            SteamworksUtilsOperation::AppIdRead { app_id } => {
                self.app_id = Some(*app_id);
            }
            SteamworksUtilsOperation::IpCountryRead { country } => {
                self.ip_country = Some(country.clone());
            }
            SteamworksUtilsOperation::OverlayEnabledRead { enabled } => {
                self.overlay_enabled = Some(*enabled);
            }
            SteamworksUtilsOperation::UiLanguageRead { language } => {
                self.ui_language = Some(language.clone());
            }
            SteamworksUtilsOperation::ServerRealTimeRead { unix_epoch_seconds } => {
                self.server_real_time = Some(*unix_epoch_seconds);
            }
            SteamworksUtilsOperation::SteamInBigPictureModeRead { enabled } => {
                self.steam_in_big_picture_mode = Some(*enabled);
            }
            SteamworksUtilsOperation::SteamRunningOnSteamDeckRead { enabled } => {
                self.steam_running_on_steam_deck = Some(*enabled);
            }
            SteamworksUtilsOperation::OverlayNotificationPositionSet { position } => {
                self.overlay_notification_position = Some(*position);
            }
            SteamworksUtilsOperation::GamepadTextInputDismissed { dismissed } => {
                self.last_gamepad_text_input_dismissed = Some(dismissed.clone());
            }
            SteamworksUtilsOperation::FloatingGamepadTextInputDismissed { dismissed } => {
                self.last_floating_gamepad_text_input_dismissed = Some(*dismissed);
            }
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

/// Gamepad text input dismissal callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksGamepadTextInputDismissed {
    /// Submitted text length reported by Steam, or `None` when the input was cancelled.
    ///
    /// The submitted text itself must be read inside Steam's original callback timing
    /// through the upstream `steamworks::Utils` helper.
    pub submitted_text_len: Option<u32>,
}

/// Floating gamepad text input dismissal callback snapshot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamworksFloatingGamepadTextInputDismissed;

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
    /// A gamepad text input dismissal callback was observed.
    GamepadTextInputDismissed {
        /// Callback snapshot.
        dismissed: SteamworksGamepadTextInputDismissed,
    },
    /// A floating gamepad text input dismissal callback was observed.
    FloatingGamepadTextInputDismissed {
        /// Callback snapshot.
        dismissed: SteamworksFloatingGamepadTextInputDismissed,
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
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksUtilsResult>,
) {
    process_utils_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksUtilsError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
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

fn process_utils_steam_events(
    state: &mut SteamworksUtilsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUtilsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::GamepadTextInputDismissed(event) => {
                SteamworksUtilsOperation::GamepadTextInputDismissed {
                    dismissed: SteamworksGamepadTextInputDismissed {
                        submitted_text_len: event.submitted_text_len,
                    },
                }
            }
            SteamworksEvent::FloatingGamepadTextInputDismissed(_) => {
                SteamworksUtilsOperation::FloatingGamepadTextInputDismissed {
                    dismissed: SteamworksFloatingGamepadTextInputDismissed,
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks utils callback"
        );
        results.write(SteamworksUtilsResult::Ok(operation));
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
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
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
    fn text_input_callbacks_are_bridged_without_client() {
        let mut app = App::new();

        app.add_plugins(SteamworksUtilsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::GamepadTextInputDismissed(
                steamworks::GamepadTextInputDismissed {
                    submitted_text_len: Some(12),
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::GamepadTextInputDismissed(
                steamworks::GamepadTextInputDismissed {
                    submitted_text_len: None,
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::FloatingGamepadTextInputDismissed(
                steamworks::FloatingGamepadTextInputDismissed,
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksUtilsResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        let submitted = SteamworksGamepadTextInputDismissed {
            submitted_text_len: Some(12),
        };
        let cancelled = SteamworksGamepadTextInputDismissed {
            submitted_text_len: None,
        };
        let floating = SteamworksFloatingGamepadTextInputDismissed;

        assert_eq!(
            drained,
            vec![
                SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputDismissed {
                    dismissed: submitted,
                }),
                SteamworksUtilsResult::Ok(SteamworksUtilsOperation::GamepadTextInputDismissed {
                    dismissed: cancelled.clone(),
                }),
                SteamworksUtilsResult::Ok(
                    SteamworksUtilsOperation::FloatingGamepadTextInputDismissed {
                        dismissed: floating,
                    },
                ),
            ]
        );

        let state = app.world().resource::<SteamworksUtilsState>();
        assert_eq!(state.last_gamepad_text_input_dismissed(), Some(&cancelled));
        assert_eq!(
            state.last_floating_gamepad_text_input_dismissed(),
            Some(&floating)
        );
        assert_eq!(state.last_error(), None);
    }

    #[test]
    fn state_records_utility_operations() {
        let mut state = SteamworksUtilsState::default();
        let info = SteamworksUtilsInfo {
            app_id: steamworks::AppId(480),
            ip_country: "US".to_owned(),
            overlay_enabled: true,
            ui_language: "english".to_owned(),
            server_real_time: 123,
            steam_in_big_picture_mode: false,
            steam_running_on_steam_deck: false,
        };

        state.record_operation(&SteamworksUtilsOperation::CurrentInfoRead { info: info.clone() });
        assert_eq!(state.current_info(), Some(&info));
        assert_eq!(state.app_id(), Some(steamworks::AppId(480)));
        assert_eq!(state.ip_country(), Some("US"));
        assert_eq!(state.overlay_enabled(), Some(true));
        assert_eq!(state.ui_language(), Some("english"));
        assert_eq!(state.server_real_time(), Some(123));
        assert_eq!(state.steam_in_big_picture_mode(), Some(false));
        assert_eq!(state.steam_running_on_steam_deck(), Some(false));

        state.record_operation(&SteamworksUtilsOperation::AppIdRead {
            app_id: steamworks::AppId(481),
        });
        state.record_operation(&SteamworksUtilsOperation::IpCountryRead {
            country: "CN".to_owned(),
        });
        state.record_operation(&SteamworksUtilsOperation::OverlayEnabledRead { enabled: false });
        state.record_operation(&SteamworksUtilsOperation::UiLanguageRead {
            language: "schinese".to_owned(),
        });
        state.record_operation(&SteamworksUtilsOperation::ServerRealTimeRead {
            unix_epoch_seconds: 456,
        });
        state.record_operation(&SteamworksUtilsOperation::SteamInBigPictureModeRead {
            enabled: true,
        });
        state.record_operation(&SteamworksUtilsOperation::SteamRunningOnSteamDeckRead {
            enabled: true,
        });
        state.record_operation(&SteamworksUtilsOperation::OverlayNotificationPositionSet {
            position: SteamworksNotificationPosition::BottomRight,
        });

        assert_eq!(state.current_info(), Some(&info));
        assert_eq!(state.app_id(), Some(steamworks::AppId(481)));
        assert_eq!(state.ip_country(), Some("CN"));
        assert_eq!(state.overlay_enabled(), Some(false));
        assert_eq!(state.ui_language(), Some("schinese"));
        assert_eq!(state.server_real_time(), Some(456));
        assert_eq!(state.steam_in_big_picture_mode(), Some(true));
        assert_eq!(state.steam_running_on_steam_deck(), Some(true));
        assert_eq!(
            state.overlay_notification_position(),
            Some(SteamworksNotificationPosition::BottomRight)
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
