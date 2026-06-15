use bevy_ecs::prelude::Resource;

use super::*;

/// Runtime state for [`crate::SteamworksUtilsPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUtilsState {
    last_error: Option<SteamworksUtilsError>,
    current_info: Option<SteamworksUtilsInfo>,
    app_id: Option<steamworks::AppId>,
    ip_country: Option<String>,
    overlay_enabled: Option<bool>,
    ui_language: Option<String>,
    server_real_time: Option<u32>,
    steam_in_big_picture_mode: Option<bool>,
    steam_running_on_steam_deck: Option<bool>,
    overlay_notification_position: Option<SteamworksNotificationPosition>,
    last_gamepad_text_input_shown: Option<SteamworksGamepadTextInputShown>,
    last_floating_gamepad_text_input_shown: Option<SteamworksFloatingGamepadTextInputShown>,
    last_gamepad_text_input_submitted: Option<SteamworksGamepadTextInputSubmitted>,
    last_gamepad_text_input_dismissed: Option<SteamworksGamepadTextInputDismissed>,
    last_floating_gamepad_text_input_dismissed: Option<SteamworksFloatingGamepadTextInputDismissed>,
}

impl SteamworksUtilsState {
    /// Returns the most recent synchronous error observed by the utils plugin.
    pub fn last_error(&self) -> Option<&SteamworksUtilsError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent utility snapshot read through the plugin.
    pub fn current_info(&self) -> Option<&SteamworksUtilsInfo> {
        self.current_info.as_ref()
    }

    /// Returns the most recent Steam app ID read through this plugin.
    pub fn app_id(&self) -> Option<steamworks::AppId> {
        self.app_id
    }

    /// Returns the most recent Steam IP country code read through this plugin.
    pub fn ip_country(&self) -> Option<&str> {
        self.ip_country.as_deref()
    }

    /// Returns the most recent Steam overlay enabled state read through this plugin.
    pub fn overlay_enabled(&self) -> Option<bool> {
        self.overlay_enabled
    }

    /// Returns the most recent Steam UI language read through this plugin.
    pub fn ui_language(&self) -> Option<&str> {
        self.ui_language.as_deref()
    }

    /// Returns the most recent Steam server real time read through this plugin.
    pub fn server_real_time(&self) -> Option<u32> {
        self.server_real_time
    }

    /// Returns the most recent Steam Big Picture mode state read through this plugin.
    pub fn steam_in_big_picture_mode(&self) -> Option<bool> {
        self.steam_in_big_picture_mode
    }

    /// Returns the most recent Steam Deck state read through this plugin.
    pub fn steam_running_on_steam_deck(&self) -> Option<bool> {
        self.steam_running_on_steam_deck
    }

    /// Returns the most recent overlay notification position submitted through this plugin.
    pub fn overlay_notification_position(&self) -> Option<SteamworksNotificationPosition> {
        self.overlay_notification_position
    }

    /// Returns the most recent Big Picture gamepad text input show result.
    pub fn last_gamepad_text_input_shown(&self) -> Option<&SteamworksGamepadTextInputShown> {
        self.last_gamepad_text_input_shown.as_ref()
    }

    /// Returns the most recent floating gamepad text input show result.
    pub fn last_floating_gamepad_text_input_shown(
        &self,
    ) -> Option<&SteamworksFloatingGamepadTextInputShown> {
        self.last_floating_gamepad_text_input_shown.as_ref()
    }

    /// Returns the most recent submitted gamepad text captured during callback timing.
    pub fn last_gamepad_text_input_submitted(
        &self,
    ) -> Option<&SteamworksGamepadTextInputSubmitted> {
        self.last_gamepad_text_input_submitted.as_ref()
    }

    /// Returns the most recent gamepad text input dismissal callback snapshot.
    pub fn last_gamepad_text_input_dismissed(
        &self,
    ) -> Option<&SteamworksGamepadTextInputDismissed> {
        self.last_gamepad_text_input_dismissed.as_ref()
    }

    /// Returns the most recent floating gamepad text input dismissal callback snapshot.
    pub fn last_floating_gamepad_text_input_dismissed(
        &self,
    ) -> Option<&SteamworksFloatingGamepadTextInputDismissed> {
        self.last_floating_gamepad_text_input_dismissed.as_ref()
    }

    pub(super) fn record_error(&mut self, error: SteamworksUtilsError) {
        self.last_error = Some(error);
    }

    pub(super) fn record_operation(&mut self, operation: &SteamworksUtilsOperation) {
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
