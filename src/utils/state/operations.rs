use super::SteamworksUtilsState;
use crate::utils::{SteamworksUtilsError, SteamworksUtilsOperation};

impl SteamworksUtilsState {
    pub(in crate::utils) fn record_error(&mut self, error: SteamworksUtilsError) {
        self.last_error = Some(error);
    }

    pub(in crate::utils) fn record_operation(&mut self, operation: &SteamworksUtilsOperation) {
        match operation {
            SteamworksUtilsOperation::CurrentInfoRead { info } => {
                self.current_info = Some(info.clone());
                self.app_id = Some(info.app_id);
                self.ip_country = Some(info.ip_country.clone());
                self.overlay_enabled = Some(info.overlay_enabled);
                self.ui_language = Some(info.ui_language.clone());
                self.server_real_time = Some(info.server_real_time);
                self.steam_in_big_picture_mode = Some(info.steam_in_big_picture_mode);
                self.steam_running_on_steam_deck = Some(info.steam_running_on_steam_deck);
            }
            SteamworksUtilsOperation::AppIdRead { app_id } => {
                self.app_id = Some(*app_id);
            }
            SteamworksUtilsOperation::IpCountryRead { country } => {
                self.ip_country = Some(country.clone());
            }
            SteamworksUtilsOperation::OverlayEnabledRead { enabled } => {
                self.overlay_enabled = Some(*enabled);
            }
            SteamworksUtilsOperation::UiLanguageRead { language } => {
                self.ui_language = Some(language.clone());
            }
            SteamworksUtilsOperation::ServerRealTimeRead { unix_epoch_seconds } => {
                self.server_real_time = Some(*unix_epoch_seconds);
            }
            SteamworksUtilsOperation::SteamInBigPictureModeRead { enabled } => {
                self.steam_in_big_picture_mode = Some(*enabled);
            }
            SteamworksUtilsOperation::SteamRunningOnSteamDeckRead { enabled } => {
                self.steam_running_on_steam_deck = Some(*enabled);
            }
            SteamworksUtilsOperation::WarningCallbackInstalled => {
                self.warning_callback_installed = true;
            }
            SteamworksUtilsOperation::OverlayNotificationPositionSet { position } => {
                self.overlay_notification_position = Some(*position);
            }
            SteamworksUtilsOperation::GamepadTextInputShown { shown } => {
                self.last_gamepad_text_input_shown = Some(shown.clone());
            }
            SteamworksUtilsOperation::FloatingGamepadTextInputShown { shown } => {
                self.last_floating_gamepad_text_input_shown = Some(shown.clone());
            }
            SteamworksUtilsOperation::GamepadTextInputSubmitted { submitted } => {
                self.last_gamepad_text_input_submitted = Some(submitted.clone());
            }
            SteamworksUtilsOperation::GamepadTextInputDismissed { dismissed } => {
                self.last_gamepad_text_input_dismissed = Some(dismissed.clone());
            }
            SteamworksUtilsOperation::FloatingGamepadTextInputDismissed { dismissed } => {
                self.last_floating_gamepad_text_input_dismissed = Some(*dismissed);
            }
        }
    }
}
