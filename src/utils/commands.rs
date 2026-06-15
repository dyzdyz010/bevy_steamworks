use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksServer};

use super::{
    callbacks::{
        process_utils_callback_queue, process_utils_steam_events, SteamworksUtilsCallbackQueue,
    },
    messages::{
        SteamworksUtilsCommand, SteamworksUtilsError, SteamworksUtilsOperation,
        SteamworksUtilsResult,
    },
    state::SteamworksUtilsState,
    types::{
        SteamworksFloatingGamepadTextInputShown, SteamworksGamepadTextInputShown,
        SteamworksUtilsInfo,
    },
};

pub(super) fn process_utils_commands(
    client: Option<Res<SteamworksClient>>,
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksUtilsState>,
    mut callback_queue: ResMut<SteamworksUtilsCallbackQueue>,
    mut commands: ResMut<Messages<SteamworksUtilsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksUtilsResult>,
) {
    let skipped_gamepad_text_input_dismissals =
        process_utils_callback_queue(&mut state, &mut callback_queue, &mut results);
    process_utils_steam_events(
        &mut state,
        &mut steam_events,
        &mut results,
        skipped_gamepad_text_input_dismissals,
    );

    for command in commands.drain() {
        match handle_utils_command(client.as_deref(), server.as_deref(), &command) {
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
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    command: &SteamworksUtilsCommand,
) -> Result<SteamworksUtilsOperation, SteamworksUtilsError> {
    validate_command(command)?;

    Ok(match command {
        SteamworksUtilsCommand::GetCurrentInfo => SteamworksUtilsOperation::CurrentInfoRead {
            info: snapshot_utils_info(&read_utils(client, server)?),
        },
        SteamworksUtilsCommand::GetAppId => SteamworksUtilsOperation::AppIdRead {
            app_id: read_utils(client, server)?.app_id(),
        },
        SteamworksUtilsCommand::GetIpCountry => SteamworksUtilsOperation::IpCountryRead {
            country: read_utils(client, server)?.ip_country(),
        },
        SteamworksUtilsCommand::IsOverlayEnabled => SteamworksUtilsOperation::OverlayEnabledRead {
            enabled: read_utils(client, server)?.is_overlay_enabled(),
        },
        SteamworksUtilsCommand::GetUiLanguage => SteamworksUtilsOperation::UiLanguageRead {
            language: read_utils(client, server)?.ui_language(),
        },
        SteamworksUtilsCommand::GetServerRealTime => SteamworksUtilsOperation::ServerRealTimeRead {
            unix_epoch_seconds: read_utils(client, server)?.get_server_real_time(),
        },
        SteamworksUtilsCommand::IsSteamInBigPictureMode => {
            SteamworksUtilsOperation::SteamInBigPictureModeRead {
                enabled: read_utils(client, server)?.is_steam_in_big_picture_mode(),
            }
        }
        SteamworksUtilsCommand::IsSteamRunningOnSteamDeck => {
            SteamworksUtilsOperation::SteamRunningOnSteamDeckRead {
                enabled: read_utils(client, server)?.is_steam_running_on_steam_deck(),
            }
        }
        SteamworksUtilsCommand::SetOverlayNotificationPosition { position } => {
            let utils = client_utils(client)?;
            utils.set_overlay_notification_position(position.to_steam());
            SteamworksUtilsOperation::OverlayNotificationPositionSet {
                position: *position,
            }
        }
        SteamworksUtilsCommand::ShowGamepadTextInput { request } => {
            let utils = client_utils(client)?;
            let shown = utils.show_gamepad_text_input(
                request.input_mode.to_steam(),
                request.line_mode.to_steam(),
                &request.description,
                request.max_characters,
                request.existing_text.as_deref(),
                |_| {},
            );
            SteamworksUtilsOperation::GamepadTextInputShown {
                shown: SteamworksGamepadTextInputShown {
                    request: request.clone(),
                    shown,
                },
            }
        }
        SteamworksUtilsCommand::ShowFloatingGamepadTextInput { request } => {
            let utils = client_utils(client)?;
            let shown = utils.show_floating_gamepad_text_input(
                request.input_mode.to_steam(),
                request.x,
                request.y,
                request.width,
                request.height,
                || {},
            );
            SteamworksUtilsOperation::FloatingGamepadTextInputShown {
                shown: SteamworksFloatingGamepadTextInputShown {
                    request: request.clone(),
                    shown,
                },
            }
        }
    })
}

fn read_utils(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Result<steamworks::Utils, SteamworksUtilsError> {
    if let Some(client) = client {
        Ok(client.utils())
    } else if let Some(server) = server {
        Ok(server.utils())
    } else {
        Err(SteamworksUtilsError::ClientUnavailable)
    }
}

fn client_utils(
    client: Option<&SteamworksClient>,
) -> Result<steamworks::Utils, SteamworksUtilsError> {
    client
        .map(|client| client.utils())
        .ok_or(SteamworksUtilsError::ClientUnavailable)
}

fn snapshot_utils_info(utils: &steamworks::Utils) -> SteamworksUtilsInfo {
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

fn validate_command(command: &SteamworksUtilsCommand) -> Result<(), SteamworksUtilsError> {
    match command {
        SteamworksUtilsCommand::ShowGamepadTextInput { request } => {
            validate_c_string("description", &request.description)?;
            if let Some(existing_text) = request.existing_text.as_ref() {
                validate_c_string("existing_text", existing_text)?;
            }
            Ok(())
        }
        SteamworksUtilsCommand::ShowFloatingGamepadTextInput { request }
            if request.width <= 0 || request.height <= 0 =>
        {
            Err(SteamworksUtilsError::InvalidFloatingTextInputBounds {
                width: request.width,
                height: request.height,
            })
        }
        _ => Ok(()),
    }
}

fn validate_c_string(field: &'static str, value: &str) -> Result<(), SteamworksUtilsError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksUtilsError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        SteamworksFloatingGamepadTextInputMode, SteamworksFloatingGamepadTextInputRequest,
        SteamworksGamepadTextInputRequest, SteamworksNotificationPosition,
    };
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
        assert_eq!(
            validate_command(&SteamworksUtilsCommand::show_gamepad_text_input(
                SteamworksGamepadTextInputRequest::new("Name", 32).with_existing_text("Existing")
            )),
            Ok(())
        );
        assert_eq!(
            validate_command(&SteamworksUtilsCommand::show_floating_gamepad_text_input(
                SteamworksFloatingGamepadTextInputRequest::new(
                    SteamworksFloatingGamepadTextInputMode::SingleLine,
                    1,
                    2,
                    300,
                    40,
                )
            )),
            Ok(())
        );
    }

    #[test]
    fn validation_rejects_text_input_values_that_would_panic_upstream() {
        assert_eq!(
            validate_command(&SteamworksUtilsCommand::show_gamepad_text_input(
                SteamworksGamepadTextInputRequest::new("bad\0description", 32)
            )),
            Err(SteamworksUtilsError::InvalidString {
                field: "description",
            })
        );
        assert_eq!(
            validate_command(&SteamworksUtilsCommand::show_gamepad_text_input(
                SteamworksGamepadTextInputRequest::new("Name", 32).with_existing_text("bad\0text")
            )),
            Err(SteamworksUtilsError::InvalidString {
                field: "existing_text",
            })
        );
        assert_eq!(
            validate_command(&SteamworksUtilsCommand::show_floating_gamepad_text_input(
                SteamworksFloatingGamepadTextInputRequest::new(
                    SteamworksFloatingGamepadTextInputMode::SingleLine,
                    1,
                    2,
                    0,
                    40,
                )
            )),
            Err(SteamworksUtilsError::InvalidFloatingTextInputBounds {
                width: 0,
                height: 40,
            })
        );
    }
}
