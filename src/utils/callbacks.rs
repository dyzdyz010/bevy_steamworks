use bevy_ecs::{
    message::{MessageReader, MessageWriter},
    prelude::Resource,
};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksUtilsOperation, SteamworksUtilsResult},
    state::SteamworksUtilsState,
    types::{
        SteamworksFloatingGamepadTextInputDismissed, SteamworksGamepadTextInputDismissed,
        SteamworksGamepadTextInputSubmitted,
    },
};

#[derive(Default, Resource)]
pub(crate) struct SteamworksUtilsCallbackQueue {
    operations: Vec<SteamworksUtilsOperation>,
}

impl SteamworksUtilsCallbackQueue {
    pub(crate) fn push_gamepad_text_input_dismissed(
        &mut self,
        dismissed: SteamworksGamepadTextInputDismissed,
    ) {
        if let (Some(text), Some(submitted_text_len)) = (
            dismissed.submitted_text.clone(),
            dismissed.submitted_text_len,
        ) {
            self.operations
                .push(SteamworksUtilsOperation::GamepadTextInputSubmitted {
                    submitted: SteamworksGamepadTextInputSubmitted {
                        text,
                        submitted_text_len,
                    },
                });
        }
        self.operations
            .push(SteamworksUtilsOperation::GamepadTextInputDismissed { dismissed });
    }

    pub(crate) fn drain(&mut self) -> std::vec::Drain<'_, SteamworksUtilsOperation> {
        self.operations.drain(..)
    }
}

pub(super) fn process_utils_callback_queue(
    state: &mut SteamworksUtilsState,
    callback_queue: &mut SteamworksUtilsCallbackQueue,
    results: &mut MessageWriter<SteamworksUtilsResult>,
) -> usize {
    let mut gamepad_text_input_dismissals = 0;
    for operation in callback_queue.drain() {
        if matches!(
            operation,
            SteamworksUtilsOperation::GamepadTextInputDismissed { .. }
        ) {
            gamepad_text_input_dismissals += 1;
        }

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks utils callback"
        );
        results.write(SteamworksUtilsResult::Ok(operation));
    }
    gamepad_text_input_dismissals
}

pub(super) fn process_utils_steam_events(
    state: &mut SteamworksUtilsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUtilsResult>,
    mut skipped_gamepad_text_input_dismissals: usize,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::GamepadTextInputDismissed(_)
                if skipped_gamepad_text_input_dismissals > 0 =>
            {
                skipped_gamepad_text_input_dismissals -= 1;
                continue;
            }
            SteamworksEvent::GamepadTextInputDismissed(event) => {
                SteamworksUtilsOperation::GamepadTextInputDismissed {
                    dismissed: SteamworksGamepadTextInputDismissed {
                        submitted_text_len: event.submitted_text_len,
                        submitted_text: None,
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
