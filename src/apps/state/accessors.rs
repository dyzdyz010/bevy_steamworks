use super::{app_value, SteamworksAppsError, SteamworksAppsState, SteamworksCurrentAppInfo};

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
}
