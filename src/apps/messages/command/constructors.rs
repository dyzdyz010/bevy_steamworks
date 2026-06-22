use super::SteamworksAppsCommand;

impl SteamworksAppsCommand {
    /// Creates a [`crate::SteamworksAppsCommand::GetCurrentAppInfo`] command.
    pub fn get_current_app_info() -> Self {
        Self::GetCurrentAppInfo
    }

    /// Creates a [`crate::SteamworksAppsCommand::IsSubscribed`] command.
    pub fn is_subscribed() -> Self {
        Self::IsSubscribed
    }

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

    /// Creates a [`crate::SteamworksAppsCommand::IsSubscribedFromFreeWeekend`] command.
    pub fn is_subscribed_from_free_weekend() -> Self {
        Self::IsSubscribedFromFreeWeekend
    }

    /// Creates a [`crate::SteamworksAppsCommand::IsVacBanned`] command.
    pub fn is_vac_banned() -> Self {
        Self::IsVacBanned
    }

    /// Creates a [`crate::SteamworksAppsCommand::IsCybercafe`] command.
    pub fn is_cybercafe() -> Self {
        Self::IsCybercafe
    }

    /// Creates a [`crate::SteamworksAppsCommand::IsLowViolence`] command.
    pub fn is_low_violence() -> Self {
        Self::IsLowViolence
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetAppBuildId`] command.
    pub fn get_app_build_id() -> Self {
        Self::GetAppBuildId
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetAppInstallDir`] command.
    pub fn get_app_install_dir(app_id: impl Into<steamworks::AppId>) -> Self {
        Self::GetAppInstallDir {
            app_id: app_id.into(),
        }
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetAppOwner`] command.
    pub fn get_app_owner() -> Self {
        Self::GetAppOwner
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetAvailableGameLanguages`] command.
    pub fn get_available_game_languages() -> Self {
        Self::GetAvailableGameLanguages
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetCurrentGameLanguage`] command.
    pub fn get_current_game_language() -> Self {
        Self::GetCurrentGameLanguage
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetCurrentBetaName`] command.
    pub fn get_current_beta_name() -> Self {
        Self::GetCurrentBetaName
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetLaunchCommandLine`] command.
    pub fn get_launch_command_line() -> Self {
        Self::GetLaunchCommandLine
    }

    /// Creates a [`crate::SteamworksAppsCommand::GetLaunchQueryParam`] command.
    pub fn get_launch_query_param(key: impl Into<String>) -> Self {
        Self::GetLaunchQueryParam { key: key.into() }
    }
}
