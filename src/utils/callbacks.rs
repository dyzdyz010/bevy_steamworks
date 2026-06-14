use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksUtilsOperation, SteamworksUtilsResult},
    state::SteamworksUtilsState,
    types::{SteamworksFloatingGamepadTextInputDismissed, SteamworksGamepadTextInputDismissed},
};

pub(super) fn process_utils_steam_events(
    state: &mut SteamworksUtilsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUtilsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::GamepadTextInputDismissed(event) => {
                SteamworksUtilsOperation::GamepadTextInputDismissed {
                    dismissed: SteamworksGamepadTextInputDismissed {
                        submitted_text_len: event.submitted_text_len,
                    },
                }
            }
            SteamworksEvent::FloatingGamepadTextInputDismissed(_) => {
                SteamworksUtilsOperation::FloatingGamepadTextInputDismissed {
                    dismissed: SteamworksFloatingGamepadTextInputDismissed,
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks utils callback"
        );
        results.write(SteamworksUtilsResult::Ok(operation));
    }
}
