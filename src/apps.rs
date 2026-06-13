//! High-level Bevy ECS integration for Steam app, ownership, language, and
//! launch-parameter queries.
//!
//! This module builds on top of the upstream [`steamworks::Apps`] API. It keeps
//! common application-level Steam checks in Bevy messages while preserving raw
//! API access through [`SteamworksClient`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Bevy plugin for high-level Steam app and launch-parameter commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksAppsCommand`] and [`SteamworksAppsResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks. It also
/// mirrors [`crate::SteamworksEvent::NewUrlLaunchParameters`] into apps results.
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
            .add_message::<SteamworksEvent>()
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
    subscribed: Option<bool>,
    installed_apps: Vec<(steamworks::AppId, bool)>,
    installed_dlcs: Vec<(steamworks::AppId, bool)>,
    subscribed_apps: Vec<(steamworks::AppId, bool)>,
    subscribed_from_free_weekend: Option<bool>,
    vac_banned: Option<bool>,
    cybercafe: Option<bool>,
    low_violence: Option<bool>,
    app_build_id: Option<i32>,
    app_install_dirs: Vec<(steamworks::AppId, String)>,
    app_owner: Option<steamworks::SteamId>,
    available_game_languages: Option<Vec<String>>,
    current_game_language: Option<String>,
    current_beta_name: Option<Option<String>>,
    launch_command_line: Option<String>,
    launch_query_params: Vec<(String, String)>,
    new_url_launch_parameters_count: u64,
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

    /// Returns the most recent current-app subscription result.
    pub fn subscribed(&self) -> Option<bool> {
        self.subscribed
    }

    /// Returns the most recent installation result for an app ID.
    pub fn app_installed(&self, app_id: steamworks::AppId) -> Option<bool> {
        app_value(&self.installed_apps, app_id).copied()
    }

    /// Returns the most recent DLC installation result for an app ID.
    pub fn dlc_installed(&self, app_id: steamworks::AppId) -> Option<bool> {
        app_value(&self.installed_dlcs, app_id).copied()
    }

    /// Returns the most recent subscription result for another app ID.
    pub fn subscribed_app(&self, app_id: steamworks::AppId) -> Option<bool> {
        app_value(&self.subscribed_apps, app_id).copied()
    }

    /// Returns the most recent free-weekend subscription result.
    pub fn subscribed_from_free_weekend(&self) -> Option<bool> {
        self.subscribed_from_free_weekend
    }

    /// Returns the most recent VAC ban result.
    pub fn vac_banned(&self) -> Option<bool> {
        self.vac_banned
    }

    /// Returns the most recent cyber cafe license result.
    pub fn cybercafe(&self) -> Option<bool> {
        self.cybercafe
    }

    /// Returns the most recent low-violence license result.
    pub fn low_violence(&self) -> Option<bool> {
        self.low_violence
    }

    /// Returns the most recent app build ID read through this plugin.
    pub fn app_build_id(&self) -> Option<i32> {
        self.app_build_id
    }

    /// Returns the most recent install directory for an app ID.
    pub fn app_install_dir(&self, app_id: steamworks::AppId) -> Option<&str> {
        app_value(&self.app_install_dirs, app_id).map(String::as_str)
    }

    /// Returns the most recent original app owner read through this plugin.
    pub fn app_owner(&self) -> Option<steamworks::SteamId> {
        self.app_owner
    }

    /// Returns the most recent available game language list.
    pub fn available_game_languages(&self) -> Option<&[String]> {
        self.available_game_languages.as_deref()
    }

    /// Returns the most recent current game language.
    pub fn current_game_language(&self) -> Option<&str> {
        self.current_game_language.as_deref()
    }

    /// Returns the most recent current beta branch name, if Steam reported one.
    pub fn current_beta_name(&self) -> Option<&str> {
        self.current_beta_name
            .as_ref()
            .and_then(|name| name.as_deref())
    }

    /// Returns the most recent current beta branch result, preserving "no beta" as `Some(None)`.
    pub fn current_beta_name_result(&self) -> Option<Option<&str>> {
        self.current_beta_name.as_ref().map(|name| name.as_deref())
    }

    /// Returns the most recent Steam URL launch command line.
    pub fn launch_command_line(&self) -> Option<&str> {
        self.launch_command_line.as_deref()
    }

    /// Returns the most recent launch query parameter value for a key.
    pub fn launch_query_param(&self, key: &str) -> Option<&str> {
        self.launch_query_params
            .iter()
            .find_map(|(known_key, value)| (known_key == key).then_some(value.as_str()))
    }

    /// Returns all launch query parameter reads cached by this plugin.
    pub fn launch_query_params(&self) -> &[(String, String)] {
        &self.launch_query_params
    }

    /// Returns how many new URL launch parameter callbacks this plugin observed.
    pub fn new_url_launch_parameters_count(&self) -> u64 {
        self.new_url_launch_parameters_count
    }

    fn record_error(&mut self, error: SteamworksAppsError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksAppsOperation) {
        match operation {
            SteamworksAppsOperation::CurrentAppInfoRead { info } => {
                self.current_app_info = Some(info.clone());
                self.app_build_id = Some(info.build_id);
                self.app_owner = Some(info.owner);
                self.subscribed = Some(info.subscribed);
                self.subscribed_from_free_weekend = Some(info.subscribed_from_free_weekend);
                self.vac_banned = Some(info.vac_banned);
                self.cybercafe = Some(info.cybercafe);
                self.low_violence = Some(info.low_violence);
                self.available_game_languages = Some(info.available_game_languages.clone());
                self.current_game_language = Some(info.current_game_language.clone());
                self.current_beta_name = Some(info.current_beta_name.clone());
            }
            SteamworksAppsOperation::SubscriptionRead { subscribed } => {
                self.subscribed = Some(*subscribed);
            }
            SteamworksAppsOperation::AppInstalledRead { app_id, installed } => {
                upsert_app_value(&mut self.installed_apps, *app_id, *installed);
            }
            SteamworksAppsOperation::DlcInstalledRead { app_id, installed } => {
                upsert_app_value(&mut self.installed_dlcs, *app_id, *installed);
            }
            SteamworksAppsOperation::SubscribedAppRead { app_id, subscribed } => {
                upsert_app_value(&mut self.subscribed_apps, *app_id, *subscribed);
            }
            SteamworksAppsOperation::SubscribedFromFreeWeekendRead {
                subscribed_from_free_weekend,
            } => {
                self.subscribed_from_free_weekend = Some(*subscribed_from_free_weekend);
            }
            SteamworksAppsOperation::VacBannedRead { vac_banned } => {
                self.vac_banned = Some(*vac_banned);
            }
            SteamworksAppsOperation::CybercafeRead { cybercafe } => {
                self.cybercafe = Some(*cybercafe);
            }
            SteamworksAppsOperation::LowViolenceRead { low_violence } => {
                self.low_violence = Some(*low_violence);
            }
            SteamworksAppsOperation::AppBuildIdRead { build_id } => {
                self.app_build_id = Some(*build_id);
            }
            SteamworksAppsOperation::AppInstallDirRead {
                app_id,
                install_dir,
            } => {
                upsert_app_value(&mut self.app_install_dirs, *app_id, install_dir.clone());
            }
            SteamworksAppsOperation::AppOwnerRead { owner } => {
                self.app_owner = Some(*owner);
            }
            SteamworksAppsOperation::AvailableGameLanguagesRead { languages } => {
                self.available_game_languages = Some(languages.clone());
            }
            SteamworksAppsOperation::CurrentGameLanguageRead { language } => {
                self.current_game_language = Some(language.clone());
            }
            SteamworksAppsOperation::CurrentBetaNameRead { beta_name } => {
                self.current_beta_name = Some(beta_name.clone());
            }
            SteamworksAppsOperation::LaunchCommandLineRead { command_line } => {
                self.launch_command_line = Some(command_line.clone());
            }
            SteamworksAppsOperation::LaunchQueryParamRead { key, value } => {
                if let Some((_, known_value)) = self
                    .launch_query_params
                    .iter_mut()
                    .find(|(known_key, _)| known_key == key)
                {
                    *known_value = value.clone();
                } else {
                    self.launch_query_params.push((key.clone(), value.clone()));
                }
            }
            SteamworksAppsOperation::NewUrlLaunchParametersReceived { count } => {
                self.new_url_launch_parameters_count = *count;
            }
        }
    }
}

fn app_value<T>(values: &[(steamworks::AppId, T)], app_id: steamworks::AppId) -> Option<&T> {
    values
        .iter()
        .find_map(|(known_app_id, value)| (*known_app_id == app_id).then_some(value))
}

fn upsert_app_value<T>(
    values: &mut Vec<(steamworks::AppId, T)>,
    app_id: steamworks::AppId,
    value: T,
) {
    if let Some((_, known_value)) = values
        .iter_mut()
        .find(|(known_app_id, _)| *known_app_id == app_id)
    {
        *known_value = value;
    } else {
        values.push((app_id, value));
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
    /// Steam reported new URL launch parameters while the app was already running.
    ///
    /// Send [`SteamworksAppsCommand::GetLaunchCommandLine`] or
    /// [`SteamworksAppsCommand::GetLaunchQueryParam`] after this operation to read
    /// the latest launch data.
    NewUrlLaunchParametersReceived {
        /// Total number of new URL launch parameter callbacks observed by this plugin.
        count: u64,
    },
}

/// Result message emitted by [`SteamworksAppsPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksAppsResult {
    /// The command or observed callback was processed successfully.
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

fn process_apps_steam_events(
    state: &mut SteamworksAppsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksAppsResult>,
) {
    for event in steam_events.read() {
        if !matches!(event, SteamworksEvent::NewUrlLaunchParameters(_)) {
            continue;
        }

        let operation = SteamworksAppsOperation::NewUrlLaunchParametersReceived {
            count: state.new_url_launch_parameters_count.saturating_add(1),
        };
        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks apps callback"
        );
        results.write(SteamworksAppsResult::Ok(operation));
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
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
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

    #[test]
    fn state_records_app_operations() {
        let mut state = SteamworksAppsState::default();
        let app_id = steamworks::AppId(480);
        let dlc_id = steamworks::AppId(481);
        let owner = steamworks::SteamId::from_raw(7);
        let owner_from_command = steamworks::SteamId::from_raw(8);
        let info = SteamworksCurrentAppInfo {
            app_id,
            build_id: 12,
            owner,
            subscribed: true,
            subscribed_from_free_weekend: false,
            vac_banned: false,
            cybercafe: false,
            low_violence: true,
            available_game_languages: vec!["english".to_owned(), "schinese".to_owned()],
            current_game_language: "english".to_owned(),
            current_beta_name: Some("preview".to_owned()),
        };

        assert_eq!(state.current_beta_name_result(), None);
        state.record_operation(&SteamworksAppsOperation::CurrentAppInfoRead { info: info.clone() });
        assert_eq!(state.current_beta_name(), Some("preview"));
        assert_eq!(state.current_beta_name_result(), Some(Some("preview")));
        state.record_operation(&SteamworksAppsOperation::AppInstalledRead {
            app_id,
            installed: false,
        });
        state.record_operation(&SteamworksAppsOperation::AppInstalledRead {
            app_id,
            installed: true,
        });
        state.record_operation(&SteamworksAppsOperation::DlcInstalledRead {
            app_id: dlc_id,
            installed: true,
        });
        state.record_operation(&SteamworksAppsOperation::SubscribedAppRead {
            app_id,
            subscribed: true,
        });
        state.record_operation(&SteamworksAppsOperation::AppInstallDirRead {
            app_id,
            install_dir: "C:/Steam/steamapps/common/Spacewar".to_owned(),
        });
        state.record_operation(&SteamworksAppsOperation::LaunchCommandLineRead {
            command_line: "+connect 127.0.0.1".to_owned(),
        });
        state.record_operation(&SteamworksAppsOperation::LaunchQueryParamRead {
            key: "connect".to_owned(),
            value: "127.0.0.1".to_owned(),
        });
        state.record_operation(&SteamworksAppsOperation::LaunchQueryParamRead {
            key: "connect".to_owned(),
            value: "localhost".to_owned(),
        });
        state.record_operation(&SteamworksAppsOperation::CurrentBetaNameRead { beta_name: None });
        state.record_operation(&SteamworksAppsOperation::NewUrlLaunchParametersReceived {
            count: 3,
        });
        state.record_operation(&SteamworksAppsOperation::SubscriptionRead { subscribed: false });
        state.record_operation(&SteamworksAppsOperation::SubscribedFromFreeWeekendRead {
            subscribed_from_free_weekend: true,
        });
        state.record_operation(&SteamworksAppsOperation::VacBannedRead { vac_banned: true });
        state.record_operation(&SteamworksAppsOperation::CybercafeRead { cybercafe: true });
        state.record_operation(&SteamworksAppsOperation::LowViolenceRead {
            low_violence: false,
        });
        state.record_operation(&SteamworksAppsOperation::AppBuildIdRead { build_id: 34 });
        state.record_operation(&SteamworksAppsOperation::AppOwnerRead {
            owner: owner_from_command,
        });
        state.record_operation(&SteamworksAppsOperation::AvailableGameLanguagesRead {
            languages: vec!["german".to_owned()],
        });
        state.record_operation(&SteamworksAppsOperation::CurrentGameLanguageRead {
            language: "german".to_owned(),
        });

        assert_eq!(state.current_app_info(), Some(&info));
        assert_eq!(state.subscribed(), Some(false));
        assert_eq!(state.subscribed_from_free_weekend(), Some(true));
        assert_eq!(state.vac_banned(), Some(true));
        assert_eq!(state.cybercafe(), Some(true));
        assert_eq!(state.low_violence(), Some(false));
        assert_eq!(state.app_build_id(), Some(34));
        assert_eq!(state.app_owner(), Some(owner_from_command));
        assert_eq!(
            state.available_game_languages(),
            Some(["german".to_owned()].as_slice())
        );
        assert_eq!(state.current_game_language(), Some("german"));
        assert_eq!(state.current_beta_name(), None);
        assert_eq!(state.current_beta_name_result(), Some(None));
        assert_eq!(state.app_installed(app_id), Some(true));
        assert_eq!(state.dlc_installed(dlc_id), Some(true));
        assert_eq!(state.subscribed_app(app_id), Some(true));
        assert_eq!(
            state.app_install_dir(app_id),
            Some("C:/Steam/steamapps/common/Spacewar")
        );
        assert_eq!(state.launch_command_line(), Some("+connect 127.0.0.1"));
        assert_eq!(state.launch_query_param("connect"), Some("localhost"));
        assert_eq!(
            state.launch_query_params(),
            &[("connect".to_owned(), "localhost".to_owned())]
        );
        assert_eq!(state.new_url_launch_parameters_count(), 3);
    }

    #[test]
    fn new_url_launch_parameters_callbacks_are_bridged_without_client() {
        let mut app = App::new();

        app.add_plugins(SteamworksAppsPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::NewUrlLaunchParameters(
                steamworks::NewUrlLaunchParameters,
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::NewUrlLaunchParameters(
                steamworks::NewUrlLaunchParameters,
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksAppsResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        assert_eq!(
            drained,
            vec![
                SteamworksAppsResult::Ok(SteamworksAppsOperation::NewUrlLaunchParametersReceived {
                    count: 1
                },),
                SteamworksAppsResult::Ok(SteamworksAppsOperation::NewUrlLaunchParametersReceived {
                    count: 2
                },),
            ]
        );

        let state = app.world().resource::<SteamworksAppsState>();
        assert_eq!(state.new_url_launch_parameters_count(), 2);
        assert_eq!(state.last_error(), None);
    }
}
