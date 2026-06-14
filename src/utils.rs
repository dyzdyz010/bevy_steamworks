//! High-level Bevy ECS integration for Steam utility queries and overlay helpers.
//!
//! This module builds on top of the upstream [`steamworks::Utils`] API. It keeps
//! common utility calls in Bevy messages while mirroring text-input dismissal
//! callbacks from [`crate::SteamworksEvent`] into [`SteamworksUtilsResult`].

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
pub use state::SteamworksUtilsState;
pub use types::*;

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
