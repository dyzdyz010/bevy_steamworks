use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    callbacks::process_screenshots_steam_events,
    messages::{
        SteamworksScreenshotsCommand, SteamworksScreenshotsError, SteamworksScreenshotsOperation,
        SteamworksScreenshotsResult,
    },
    state::SteamworksScreenshotsState,
};

pub(super) fn process_screenshots_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksScreenshotsState>,
    mut commands: ResMut<Messages<SteamworksScreenshotsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksScreenshotsResult>,
) {
    process_screenshots_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksScreenshotsError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks screenshots command failed"
            );
            results.write(SteamworksScreenshotsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_screenshots_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks screenshots command"
                );
                results.write(SteamworksScreenshotsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks screenshots command failed"
                );
                results.write(SteamworksScreenshotsResult::Err { command, error });
            }
        }
    }
}

fn handle_screenshots_command(
    client: &SteamworksClient,
    command: &SteamworksScreenshotsCommand,
) -> Result<SteamworksScreenshotsOperation, SteamworksScreenshotsError> {
    validate_command(command)?;

    let screenshots = client.screenshots();
    match command {
        SteamworksScreenshotsCommand::HookScreenshots { hook } => {
            screenshots.hook_screenshots(*hook);
            Ok(SteamworksScreenshotsOperation::ScreenshotsHookSet { hook: *hook })
        }
        SteamworksScreenshotsCommand::IsScreenshotsHooked => {
            Ok(SteamworksScreenshotsOperation::ScreenshotsHookedRead {
                hooked: screenshots.is_screenshots_hooked(),
            })
        }
        SteamworksScreenshotsCommand::TriggerScreenshot => {
            screenshots.trigger_screenshot();
            Ok(SteamworksScreenshotsOperation::ScreenshotTriggered)
        }
        SteamworksScreenshotsCommand::AddScreenshotToLibrary {
            filename,
            thumbnail_filename,
            width,
            height,
        } => screenshots
            .add_screenshot_to_library(filename, thumbnail_filename.as_deref(), *width, *height)
            .map(
                |handle| SteamworksScreenshotsOperation::ScreenshotLibraryAddSubmitted {
                    handle,
                    filename: filename.clone(),
                    thumbnail_filename: thumbnail_filename.clone(),
                    width: *width,
                    height: *height,
                },
            )
            .map_err(SteamworksScreenshotsError::library_add_failed),
    }
}

fn validate_command(
    command: &SteamworksScreenshotsCommand,
) -> Result<(), SteamworksScreenshotsError> {
    if let SteamworksScreenshotsCommand::AddScreenshotToLibrary { width, height, .. } = command {
        if *width <= 0 || *height <= 0 {
            return Err(SteamworksScreenshotsError::InvalidDimensions {
                width: *width,
                height: *height,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_invalid_dimensions() {
        let command = SteamworksScreenshotsCommand::add_screenshot_to_library(
            "shot.png",
            None::<&str>,
            0,
            720,
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksScreenshotsError::InvalidDimensions {
                width: 0,
                height: 720,
            })
        );
    }
}
