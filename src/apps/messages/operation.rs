use super::super::SteamworksCurrentAppInfo;

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
    /// Send [`crate::SteamworksAppsCommand::GetLaunchCommandLine`] or
    /// [`crate::SteamworksAppsCommand::GetLaunchQueryParam`] after this operation to read
    /// the latest launch data.
    NewUrlLaunchParametersReceived {
        /// Total number of new URL launch parameter callbacks observed by this plugin.
        count: u64,
    },
}
