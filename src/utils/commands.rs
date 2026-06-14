use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    callbacks::process_utils_steam_events,
    messages::{
        SteamworksUtilsCommand, SteamworksUtilsError, SteamworksUtilsOperation,
        SteamworksUtilsResult,
    },
    state::SteamworksUtilsState,
    types::SteamworksUtilsInfo,
};

pub(super) fn process_utils_commands(
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
    use super::super::SteamworksNotificationPosition;
    use super::*;

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
