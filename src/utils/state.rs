use bevy_ecs::prelude::Resource;

use super::*;

mod accessors;
mod operations;

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
    warning_callback_installed: bool,
    overlay_notification_position: Option<SteamworksNotificationPosition>,
    last_gamepad_text_input_shown: Option<SteamworksGamepadTextInputShown>,
    last_floating_gamepad_text_input_shown: Option<SteamworksFloatingGamepadTextInputShown>,
    last_gamepad_text_input_submitted: Option<SteamworksGamepadTextInputSubmitted>,
    last_gamepad_text_input_dismissed: Option<SteamworksGamepadTextInputDismissed>,
    last_floating_gamepad_text_input_dismissed: Option<SteamworksFloatingGamepadTextInputDismissed>,
}
