use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksEvent};

use super::{
    callbacks::process_networking_utils_steam_events,
    messages::{
        SteamworksNetworkingUtilsCommand, SteamworksNetworkingUtilsError,
        SteamworksNetworkingUtilsOperation, SteamworksNetworkingUtilsResult,
    },
    state::SteamworksNetworkingUtilsState,
    types::SteamworksRelayNetworkStatus,
    validation::validate_command,
};

pub(super) fn process_networking_utils_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingUtilsState>,
    mut commands: ResMut<Messages<SteamworksNetworkingUtilsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksNetworkingUtilsResult>,
) {
    let Some(client) = client else {
        let error = SteamworksNetworkingUtilsError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks networking utils command failed"
            );
            results.write(SteamworksNetworkingUtilsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    process_networking_utils_steam_events(&client, &mut state, &mut steam_events, &mut results);

    for command in commands.drain() {
        match handle_networking_utils_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks networking utils command"
                );
                results.write(SteamworksNetworkingUtilsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks networking utils command failed"
                );
                results.write(SteamworksNetworkingUtilsResult::Err { command, error });
            }
        }
    }
}

fn handle_networking_utils_command(
    client: &SteamworksClient,
    command: &SteamworksNetworkingUtilsCommand,
) -> Result<SteamworksNetworkingUtilsOperation, SteamworksNetworkingUtilsError> {
    validate_command(command)?;

    let networking_utils = client.networking_utils();
    Ok(match command {
        SteamworksNetworkingUtilsCommand::InitRelayNetworkAccess => {
            networking_utils.init_relay_network_access();
            SteamworksNetworkingUtilsOperation::RelayNetworkAccessInitialized
        }
        SteamworksNetworkingUtilsCommand::GetRelayNetworkStatus => {
            SteamworksNetworkingUtilsOperation::RelayNetworkStatusRead {
                availability: networking_utils.relay_network_status(),
            }
        }
        SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus => {
            SteamworksNetworkingUtilsOperation::DetailedRelayNetworkStatusRead {
                status: SteamworksRelayNetworkStatus::from_steam(
                    networking_utils.detailed_relay_network_status(),
                ),
            }
        }
    })
}
