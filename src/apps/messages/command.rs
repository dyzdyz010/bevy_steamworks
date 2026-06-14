use bevy_ecs::message::Message;

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
    /// Creates a [`crate::SteamworksAppsCommand::IsAppInstalled`] command.
    pub fn is_app_installed(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::IsAppInstalled {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`crate::SteamworksAppsCommand::IsDlcInstalled`] command.
    pub fn is_dlc_installed(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::IsDlcInstalled {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`crate::SteamworksAppsCommand::IsSubscribedApp`] command.
    pub fn is_subscribed_app(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::IsSubscribedApp {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetAppInstallDir`] command.
    pub fn get_app_install_dir(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::GetAppInstallDir {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetLaunchQueryParam`] command.
    pub fn get_launch_query_param(key: impl Into<String>) -> Self {
        Self::GetLaunchQueryParam { key: key.into() }
    }
}
