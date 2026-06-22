use super::{
    upsert_app_value, SteamworksAppsError, SteamworksAppsOperation, SteamworksAppsState,
    STEAMWORKS_APPS_STATE_CACHE_LIMIT,
};
use crate::cache::trim_oldest;

impl SteamworksAppsState {
    pub(in crate::apps) fn record_error(&mut self, error: SteamworksAppsError) {
        self.last_error = Some(error);
    }

    pub(in crate::apps) fn record_operation(&mut self, operation: &SteamworksAppsOperation) {
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
                    trim_oldest(
                        &mut self.launch_query_params,
                        STEAMWORKS_APPS_STATE_CACHE_LIMIT,
                    );
                }
            }
            SteamworksAppsOperation::NewUrlLaunchParametersReceived { count } => {
                self.new_url_launch_parameters_count = *count;
            }
        }
    }
}
