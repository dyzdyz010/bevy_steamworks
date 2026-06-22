use super::super::super::{
    SteamworksFloatingGamepadTextInputRequest, SteamworksGamepadTextInputRequest,
    SteamworksNotificationPosition,
};
use super::SteamworksUtilsCommand;

impl SteamworksUtilsCommand {
    /// Creates a [`crate::SteamworksUtilsCommand::GetCurrentInfo`] command.
    pub fn get_current_info() -> Self {
        Self::GetCurrentInfo
    }

    /// Creates a [`crate::SteamworksUtilsCommand::GetAppId`] command.
    pub fn get_app_id() -> Self {
        Self::GetAppId
    }

    /// Creates a [`crate::SteamworksUtilsCommand::GetIpCountry`] command.
    pub fn get_ip_country() -> Self {
        Self::GetIpCountry
    }

    /// Creates a [`crate::SteamworksUtilsCommand::IsOverlayEnabled`] command.
    pub fn is_overlay_enabled() -> Self {
        Self::IsOverlayEnabled
    }

    /// Creates a [`crate::SteamworksUtilsCommand::GetUiLanguage`] command.
    pub fn get_ui_language() -> Self {
        Self::GetUiLanguage
    }

    /// Creates a [`crate::SteamworksUtilsCommand::GetServerRealTime`] command.
    pub fn get_server_real_time() -> Self {
        Self::GetServerRealTime
    }

    /// Creates a [`crate::SteamworksUtilsCommand::IsSteamInBigPictureMode`] command.
    pub fn is_steam_in_big_picture_mode() -> Self {
        Self::IsSteamInBigPictureMode
    }

    /// Creates a [`crate::SteamworksUtilsCommand::IsSteamRunningOnSteamDeck`] command.
    pub fn is_steam_running_on_steam_deck() -> Self {
        Self::IsSteamRunningOnSteamDeck
    }

    /// Creates a [`crate::SteamworksUtilsCommand::InstallWarningCallback`] command.
    pub fn install_warning_callback() -> Self {
        Self::InstallWarningCallback
    }

    /// Creates a [`crate::SteamworksUtilsCommand::SetOverlayNotificationPosition`] command.
    pub fn set_overlay_notification_position(position: SteamworksNotificationPosition) -> Self {
        Self::SetOverlayNotificationPosition { position }
    }

    /// Creates a [`crate::SteamworksUtilsCommand::ShowGamepadTextInput`] command.
    pub fn show_gamepad_text_input(request: SteamworksGamepadTextInputRequest) -> Self {
        Self::ShowGamepadTextInput { request }
    }

    /// Creates a [`crate::SteamworksUtilsCommand::ShowFloatingGamepadTextInput`] command.
    pub fn show_floating_gamepad_text_input(
        request: SteamworksFloatingGamepadTextInputRequest,
    ) -> Self {
        Self::ShowFloatingGamepadTextInput { request }
    }
}
