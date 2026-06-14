use bevy_ecs::prelude::Resource;

use super::{
    messages::{SteamworksAppsError, SteamworksAppsOperation},
    types::SteamworksCurrentAppInfo,
};

/// Runtime state for [`super::SteamworksAppsPlugin`].
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

    pub(super) fn record_error(&mut self, error: SteamworksAppsError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksAppsOperation) {
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
