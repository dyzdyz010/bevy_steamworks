//! High-level Bevy ECS integration for Steam app, ownership, language, and
//! launch-parameter queries.
//!
//! This module builds on top of the upstream [`steamworks::Apps`] API. It keeps
//! common application-level Steam checks in Bevy messages while preserving raw
//! API access through [`SteamworksClient`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Bevy plugin for high-level Steam app and launch-parameter commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksAppsCommand`] and [`SteamworksAppsResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksAppsPlugin;

impl SteamworksAppsPlugin {
    /// Creates an apps plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksAppsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksAppsState>()
            .add_message::<SteamworksAppsCommand>()
            .add_message::<SteamworksAppsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessAppsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_apps_commands.in_set(SteamworksSystem::ProcessAppsCommands),
            );
    }
}

/// Runtime state for [`SteamworksAppsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksAppsState {
    last_error: Option<SteamworksAppsError>,
    current_app_info: Option<SteamworksCurrentAppInfo>,
}

impl SteamworksAppsState {
    /// Returns the most recent synchronous error observed by the apps plugin.
    pub fn last_error(&self) -> Option<&SteamworksAppsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent current-app snapshot read through the plugin.
    pub fn current_app_info(&self) -> Option<&SteamworksCurrentAppInfo> {
        self.current_app_info.as_ref()
    }

    fn record_error(&mut self, error: SteamworksAppsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksAppsOperation) {
        if let SteamworksAppsOperation::CurrentAppInfoRead { info } = operation {
            self.current_app_info = Some(info.clone());
        }
    }
}

/// Snapshot of common Steam app information for the current process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksCurrentAppInfo {
    /// Current Steam app ID.
    pub app_id: steamworks::AppId,
    /// Current app build ID.
    pub build_id: i32,
    /// Original owner Steam ID for this app.
    pub owner: steamworks::SteamId,
    /// Whether the current user is subscribed to this app.
    pub subscribed: bool,
    /// Whether the current user is subscribed via a free weekend.
    pub subscribed_from_free_weekend: bool,
    /// Whether the current user has a VAC ban.
    pub vac_banned: bool,
    /// Whether the current license is for a cyber cafe.
    pub cybercafe: bool,
    /// Whether the current license is a low-violence depot.
    pub low_violence: bool,
    /// Languages supported by the app.
    pub available_game_languages: Vec<String>,
    /// Current game language.
    pub current_game_language: String,
    /// Current beta branch name, if any.
    pub current_beta_name: Option<String>,
}

/// A high-level command for Steam app and launch-parameter queries.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksAppsCommand {
    /// Read a snapshot of common current-app information.
    GetCurrentAppInfo,
    /// Read whether the current user is subscribed to this app.
    IsSubscribed,
    /// Read whether another Steam app is installed.
    IsAppInstalled {
        /// Steam app ID to check.
        app_id: steamworks::AppId,
    },
    /// Read whether a DLC app is owned and installed.
    IsDlcInstalled {
        /// Steam DLC app ID to check.
        app_id: steamworks::AppId,
    },
    /// Read whether the current user is subscribed to another app.
    IsSubscribedApp {
        /// Steam app ID to check.
        app_id: steamworks::AppId,
    },
    /// Read whether the current subscription is from a free weekend.
    IsSubscribedFromFreeWeekend,
    /// Read whether the current user has a VAC ban.
    IsVacBanned,
    /// Read whether the current license is for a cyber cafe.
    IsCybercafe,
    /// Read whether the current license is a low-violence depot.
    IsLowViolence,
    /// Read the current app build ID.
    GetAppBuildId,
    /// Read the installation directory for an app.
    GetAppInstallDir {
        /// Steam app ID to inspect.
        app_id: steamworks::AppId,
    },
    /// Read the original owner of this app.
    GetAppOwner,
    /// Read the languages supported by this app.
    GetAvailableGameLanguages,
    /// Read the current game language.
    GetCurrentGameLanguage,
    /// Read the current beta branch name.
    GetCurrentBetaName,
    /// Read the launch command line from Steam.
    GetLaunchCommandLine,
    /// Read one launch query parameter from Steam.
    GetLaunchQueryParam {
        /// Launch query parameter key.
        key: String,
    },
}

impl SteamworksAppsCommand {
    /// Creates a [`SteamworksAppsCommand::IsAppInstalled`] command.
    pub fn is_app_installed(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::IsAppInstalled {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`SteamworksAppsCommand::IsDlcInstalled`] command.
    pub fn is_dlc_installed(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::IsDlcInstalled {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`SteamworksAppsCommand::IsSubscribedApp`] command.
    pub fn is_subscribed_app(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::IsSubscribedApp {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`SteamworksAppsCommand::GetAppInstallDir`] command.
    pub fn get_app_install_dir(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::GetAppInstallDir {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`SteamworksAppsCommand::GetLaunchQueryParam`] command.
    pub fn get_launch_query_param(key: impl Into<String>) -> Self {
        Self::GetLaunchQueryParam { key: key.into() }
    }
}

/// A successfully processed Steam app operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksAppsOperation {
    /// Common current-app information was read.
    CurrentAppInfoRead {
        /// Current-app snapshot.
        info: SteamworksCurrentAppInfo,
    },
    /// Current-app subscription state was read.
    SubscriptionRead {
        /// Whether the current user is subscribed.
        subscribed: bool,
    },
    /// App installation state was read.
    AppInstalledRead {
        /// Steam app ID checked.
        app_id: steamworks::AppId,
        /// Whether the app is installed.
        installed: bool,
    },
    /// DLC installation state was read.
    DlcInstalledRead {
        /// Steam DLC app ID checked.
        app_id: steamworks::AppId,
        /// Whether the DLC is owned and installed.
        installed: bool,
    },
    /// Another app subscription state was read.
    SubscribedAppRead {
        /// Steam app ID checked.
        app_id: steamworks::AppId,
        /// Whether the current user is subscribed to the app.
        subscribed: bool,
    },
    /// Free-weekend subscription state was read.
    SubscribedFromFreeWeekendRead {
        /// Whether the current subscription is from a free weekend.
        subscribed_from_free_weekend: bool,
    },
    /// VAC ban state was read.
    VacBannedRead {
        /// Whether the current user has a VAC ban.
        vac_banned: bool,
    },
    /// Cyber cafe license state was read.
    CybercafeRead {
        /// Whether the current license is for a cyber cafe.
        cybercafe: bool,
    },
    /// Low-violence license state was read.
    LowViolenceRead {
        /// Whether the current license is a low-violence depot.
        low_violence: bool,
    },
    /// Current app build ID was read.
    AppBuildIdRead {
        /// Build ID.
        build_id: i32,
    },
    /// App installation directory was read.
    AppInstallDirRead {
        /// Steam app ID checked.
        app_id: steamworks::AppId,
        /// Installation directory.
        install_dir: String,
    },
    /// Original app owner was read.
    AppOwnerRead {
        /// Original app owner Steam ID.
        owner: steamworks::SteamId,
    },
    /// Available game languages were read.
    AvailableGameLanguagesRead {
        /// Languages supported by the app.
        languages: Vec<String>,
    },
    /// Current game language was read.
    CurrentGameLanguageRead {
        /// Current game language.
        language: String,
    },
    /// Current beta branch name was read.
    CurrentBetaNameRead {
        /// Current beta branch name, if any.
        beta_name: Option<String>,
    },
    /// Launch command line was read.
    LaunchCommandLineRead {
        /// Launch command line, or an empty string when Steam has none.
        command_line: String,
    },
    /// Launch query parameter was read.
    LaunchQueryParamRead {
        /// Launch query parameter key.
        key: String,
        /// Launch query parameter value.
        value: String,
    },
}

/// Result message emitted by [`SteamworksAppsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksAppsResult {
    /// The command was processed successfully.
    Ok(SteamworksAppsOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksAppsCommand,
        /// Failure reason.
        error: SteamworksAppsError,
    },
}

/// Synchronous errors from [`SteamworksAppsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksAppsError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks apps command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
}

impl SteamworksAppsError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}

fn process_apps_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksAppsState>,
    mut commands: ResMut<Messages<SteamworksAppsCommand>>,
    mut results: MessageWriter<SteamworksAppsResult>,
) {
    let Some(client) = client else {
        let error = SteamworksAppsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
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
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn apps_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksAppsPlugin::new());

        assert!(app.world().contains_resource::<SteamworksAppsState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksAppsCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksAppsResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksAppsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksAppsCommand>>()
            .write(SteamworksAppsCommand::GetCurrentAppInfo);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksAppsResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksAppsResult::Err {
                command: SteamworksAppsCommand::GetCurrentAppInfo,
                error: SteamworksAppsError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksAppsState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksAppsError::ClientUnavailable)
        );
    }

    #[test]
    fn string_validation_rejects_interior_nul() {
        let command = SteamworksAppsCommand::get_launch_query_param("connect\0bad");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksAppsError::InvalidString { field: "key" })
        );
    }
}
