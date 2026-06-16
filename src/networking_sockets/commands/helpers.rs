use crate::{SteamworksClient, SteamworksServer};

use super::super::{
    handles::{SteamworksNetworkingSocketsHandleOwner, SteamworksNetworkingSocketsHandleStorage},
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsOutboundMessage,
    SteamworksNetworkingSocketsPollGroupId,
};

pub(super) fn steam_config_entries(
    options: &[SteamworksNetworkingSocketsConfigEntry],
) -> Vec<steamworks::networking_types::NetworkingConfigEntry> {
    options
        .iter()
        .map(SteamworksNetworkingSocketsConfigEntry::to_steam)
        .collect()
}

pub(super) fn networking_sockets(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Result<
    (
        steamworks::networking_sockets::NetworkingSockets,
        SteamworksNetworkingSocketsHandleOwner,
    ),
    SteamworksNetworkingSocketsError,
> {
    if let Some(client) = client {
        Ok((
            client.networking_sockets(),
            SteamworksNetworkingSocketsHandleOwner::Client,
        ))
    } else if let Some(server) = server {
        Ok((
            server.networking_sockets(),
            SteamworksNetworkingSocketsHandleOwner::Server,
        ))
    } else {
        Err(SteamworksNetworkingSocketsError::ClientUnavailable)
    }
}

pub(super) fn server_networking_sockets(
    server: Option<&SteamworksServer>,
) -> Result<
    (
        steamworks::networking_sockets::NetworkingSockets,
        SteamworksNetworkingSocketsHandleOwner,
    ),
    SteamworksNetworkingSocketsError,
> {
    if let Some(server) = server {
        Ok((
            server.networking_sockets(),
            SteamworksNetworkingSocketsHandleOwner::Server,
        ))
    } else {
        Err(SteamworksNetworkingSocketsError::ServerUnavailable)
    }
}

pub(super) fn networking_sockets_for_owner(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    owner: SteamworksNetworkingSocketsHandleOwner,
) -> Result<
    (
        steamworks::networking_sockets::NetworkingSockets,
        SteamworksNetworkingSocketsHandleOwner,
    ),
    SteamworksNetworkingSocketsError,
> {
    match owner {
        SteamworksNetworkingSocketsHandleOwner::Client => client
            .map(|client| {
                (
                    client.networking_sockets(),
                    SteamworksNetworkingSocketsHandleOwner::Client,
                )
            })
            .ok_or(SteamworksNetworkingSocketsError::ClientUnavailable),
        SteamworksNetworkingSocketsHandleOwner::Server => server_networking_sockets(server),
    }
}

pub(super) fn connection_owner(
    handles: &SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
) -> Result<SteamworksNetworkingSocketsHandleOwner, SteamworksNetworkingSocketsError> {
    handles
        .connection_owner(connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })
}

pub(super) fn poll_group_owner(
    handles: &SteamworksNetworkingSocketsHandleStorage,
    poll_group: SteamworksNetworkingSocketsPollGroupId,
) -> Result<SteamworksNetworkingSocketsHandleOwner, SteamworksNetworkingSocketsError> {
    handles
        .poll_group_owner(poll_group)
        .ok_or(SteamworksNetworkingSocketsError::PollGroupNotFound { id: poll_group })
}

pub(in crate::networking_sockets) fn connection_user_data_from_info_result(
    result: Result<i64, steamworks::networking_sockets::InvalidHandle>,
) -> Result<i64, SteamworksNetworkingSocketsError> {
    result.map_err(|_| SteamworksNetworkingSocketsError::invalid_handle("net_connection.info"))
}

pub(super) fn allocate_outbound_message(
    client: &SteamworksClient,
    connection: &steamworks::networking_sockets::NetConnection,
    message: &SteamworksNetworkingSocketsOutboundMessage,
) -> Result<steamworks::networking_types::NetworkingMessage, SteamworksNetworkingSocketsError> {
    let mut outbound = client
        .networking_utils()
        .allocate_message(message.data.len());
    outbound.set_connection(connection);
    outbound.set_send_flags(message.send_flags);
    outbound.set_channel(message.channel);
    outbound.set_user_data(message.user_data);

    if !message.data.is_empty() {
        outbound
            .copy_data_into_buffer(&message.data)
            .map_err(|source| {
                SteamworksNetworkingSocketsError::message_error(
                    "networking_utils.allocate_message.copy_data_into_buffer",
                    source,
                )
            })?;
    }

    Ok(outbound)
}
