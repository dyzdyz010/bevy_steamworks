use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    callbacks::process_apps_steam_events,
    messages::{
        SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation, SteamworksAppsResult,
    },
    state::SteamworksAppsState,
    types::SteamworksCurrentAppInfo,
};

pub(super) fn process_apps_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksAppsState>,
    mut commands: ResMut<Messages<SteamworksAppsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksAppsResult>,
) {
    process_apps_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksAppsError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            results.write(SteamworksAppsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_apps_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks apps command"
                );
                results.write(SteamworksAppsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks apps command failed"
                );
                results.write(SteamworksAppsResult::Err { command, error });
            }
        }
    }
}

fn handle_apps_command(
    client: &SteamworksClient,
    command: &SteamworksAppsCommand,
) -> Result<SteamworksAppsOperation, SteamworksAppsError> {
    validate_command(command)?;

    let apps = client.apps();
    Ok(match command {
        SteamworksAppsCommand::GetCurrentAppInfo => SteamworksAppsOperation::CurrentAppInfoRead {
            info: SteamworksCurrentAppInfo {
                app_id: client.utils().app_id(),
                build_id: apps.app_build_id(),
                owner: apps.app_owner(),
                subscribed: apps.is_subscribed(),
                subscribed_from_free_weekend: apps.is_subscribed_from_free_weekend(),
                vac_banned: apps.is_vac_banned(),
                cybercafe: apps.is_cybercafe(),
                low_violence: apps.is_low_violence(),
                available_game_languages: apps.available_game_languages(),
                current_game_language: apps.current_game_language(),
                current_beta_name: apps.current_beta_name(),
            },
        },
        SteamworksAppsCommand::IsSubscribed => SteamworksAppsOperation::SubscriptionRead {
            subscribed: apps.is_subscribed(),
        },
        SteamworksAppsCommand::IsAppInstalled { app_id } => {
            SteamworksAppsOperation::AppInstalledRead {
                app_id: *app_id,
                installed: apps.is_app_installed(*app_id),
            }
        }
        SteamworksAppsCommand::IsDlcInstalled { app_id } => {
            SteamworksAppsOperation::DlcInstalledRead {
                app_id: *app_id,
                installed: apps.is_dlc_installed(*app_id),
            }
        }
        SteamworksAppsCommand::IsSubscribedApp { app_id } => {
            SteamworksAppsOperation::SubscribedAppRead {
                app_id: *app_id,
                subscribed: apps.is_subscribed_app(*app_id),
            }
        }
        SteamworksAppsCommand::IsSubscribedFromFreeWeekend => {
            SteamworksAppsOperation::SubscribedFromFreeWeekendRead {
                subscribed_from_free_weekend: apps.is_subscribed_from_free_weekend(),
            }
        }
        SteamworksAppsCommand::IsVacBanned => SteamworksAppsOperation::VacBannedRead {
            vac_banned: apps.is_vac_banned(),
        },
        SteamworksAppsCommand::IsCybercafe => SteamworksAppsOperation::CybercafeRead {
            cybercafe: apps.is_cybercafe(),
        },
        SteamworksAppsCommand::IsLowViolence => SteamworksAppsOperation::LowViolenceRead {
            low_violence: apps.is_low_violence(),
        },
        SteamworksAppsCommand::GetAppBuildId => SteamworksAppsOperation::AppBuildIdRead {
            build_id: apps.app_build_id(),
        },
        SteamworksAppsCommand::GetAppInstallDir { app_id } => {
            SteamworksAppsOperation::AppInstallDirRead {
                app_id: *app_id,
                install_dir: apps.app_install_dir(*app_id),
            }
        }
        SteamworksAppsCommand::GetAppOwner => SteamworksAppsOperation::AppOwnerRead {
            owner: apps.app_owner(),
        },
        SteamworksAppsCommand::GetAvailableGameLanguages => {
            SteamworksAppsOperation::AvailableGameLanguagesRead {
                languages: apps.available_game_languages(),
            }
        }
        SteamworksAppsCommand::GetCurrentGameLanguage => {
            SteamworksAppsOperation::CurrentGameLanguageRead {
                language: apps.current_game_language(),
            }
        }
        SteamworksAppsCommand::GetCurrentBetaName => SteamworksAppsOperation::CurrentBetaNameRead {
            beta_name: apps.current_beta_name(),
        },
        SteamworksAppsCommand::GetLaunchCommandLine => {
            SteamworksAppsOperation::LaunchCommandLineRead {
                command_line: apps.launch_command_line(),
            }
        }
        SteamworksAppsCommand::GetLaunchQueryParam { key } => {
            SteamworksAppsOperation::LaunchQueryParamRead {
                key: key.clone(),
                value: apps.launch_query_param(key),
            }
        }
    })
}

fn validate_command(command: &SteamworksAppsCommand) -> Result<(), SteamworksAppsError> {
    if let SteamworksAppsCommand::GetLaunchQueryParam { key } = command {
        validate_steam_string("key", key)?;
    }

    Ok(())
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksAppsError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksAppsError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_validation_rejects_interior_nul() {
        let command = SteamworksAppsCommand::get_launch_query_param("connect\0bad");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksAppsError::InvalidString { field: "key" })
        );
    }
}
