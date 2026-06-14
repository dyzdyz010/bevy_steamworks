use bevy_ecs::message::Message;
use thiserror::Error;

use super::types::SteamworksCurrentAppInfo;

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

/// Result message emitted by [`super::SteamworksAppsPlugin`].
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

/// Synchronous errors from [`super::SteamworksAppsPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksAppsError {
    /// No [`crate::SteamworksClient`] resource exists.
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
    pub(super) fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }
}
