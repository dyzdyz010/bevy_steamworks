use super::SteamworksUtilsState;
use crate::utils::{
    SteamworksFloatingGamepadTextInputDismissed, SteamworksFloatingGamepadTextInputShown,
    SteamworksGamepadTextInputDismissed, SteamworksGamepadTextInputShown,
    SteamworksGamepadTextInputSubmitted, SteamworksNotificationPosition, SteamworksUtilsError,
    SteamworksUtilsInfo,
};

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

    /// Returns whether this plugin installed the Steam SDK warning callback.
    pub fn warning_callback_installed(&self) -> bool {
        self.warning_callback_installed
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
}
