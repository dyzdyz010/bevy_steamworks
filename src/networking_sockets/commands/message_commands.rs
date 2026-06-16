use crate::{SteamworksClient, SteamworksServer};

use super::super::{
    handles::{SteamworksNetworkingSocketsHandleOwner, SteamworksNetworkingSocketsHandleStorage},
    snapshots::snapshot_message,
    SteamworksNetworkingSocketsConnectionId, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsMessageSendResult, SteamworksNetworkingSocketsOperation,
    SteamworksNetworkingSocketsOutboundMessage,
};
use super::helpers::{allocate_outbound_message, networking_sockets_for_owner};

pub(super) fn send_message(
    handles: &SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    send_flags: steamworks::networking_types::SendFlags,
    data: &[u8],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    let message_number = connection_ref
        .send_message(data, send_flags)
        .map_err(|source| {
            SteamworksNetworkingSocketsError::steam_error("net_connection.send_message", source)
        })?;
    Ok(SteamworksNetworkingSocketsOperation::MessageSent {
        connection,
        message_number: u64::from(message_number),
        bytes: data.len(),
    })
}

pub(super) fn send_messages(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &SteamworksNetworkingSocketsHandleStorage,
    messages: &[SteamworksNetworkingSocketsOutboundMessage],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let mut outbound = Vec::with_capacity(messages.len());
    for message in messages {
        if handles.connection_owner(message.connection)
            == Some(SteamworksNetworkingSocketsHandleOwner::Server)
        {
            return Err(
                SteamworksNetworkingSocketsError::ServerConnectionBatchSendUnsupported {
                    connection: message.connection,
                },
            );
        }
    }

    let client = client.ok_or(SteamworksNetworkingSocketsError::ClientUnavailable)?;
    let (sockets, _) = networking_sockets_for_owner(
        Some(client),
        server,
        SteamworksNetworkingSocketsHandleOwner::Client,
    )?;
    for message in messages {
        let connection_ref = handles.connections.get(&message.connection).ok_or(
            SteamworksNetworkingSocketsError::ConnectionNotFound {
                id: message.connection,
            },
        )?;
        outbound.push(allocate_outbound_message(client, connection_ref, message)?);
    }

    let send_results = sockets.send_messages(outbound);
    let messages: Vec<SteamworksNetworkingSocketsMessageSendResult> = messages
        .iter()
        .zip(send_results)
        .map(
            |(message, result)| SteamworksNetworkingSocketsMessageSendResult {
                connection: message.connection,
                send_flags: message.send_flags,
                channel: message.channel,
                bytes: message.data.len(),
                user_data: message.user_data,
                result: result.map(u64::from),
            },
        )
        .collect();
    if messages
        .iter()
        .any(|message: &SteamworksNetworkingSocketsMessageSendResult| message.result.is_err())
    {
        tracing::warn!(
            target: "bevy_steamworks",
            messages = ?messages,
            "Steamworks networking sockets batch send had per-message failures"
        );
    }
    Ok(SteamworksNetworkingSocketsOperation::MessagesSent { messages })
}

pub(super) fn receive_messages(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    batch_size: usize,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get_mut(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    let messages = connection_ref.receive_messages(batch_size).map_err(|_| {
        SteamworksNetworkingSocketsError::invalid_handle("net_connection.receive_messages")
    })?;
    let messages = messages
        .into_iter()
        .map(|message| snapshot_message(connection, message))
        .collect();
    Ok(SteamworksNetworkingSocketsOperation::MessagesReceived {
        connection,
        messages,
    })
}

pub(super) fn flush_messages(
    handles: &SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    connection_ref.flush_messages().map_err(|source| {
        SteamworksNetworkingSocketsError::steam_error("net_connection.flush_messages", source)
    })?;
    Ok(SteamworksNetworkingSocketsOperation::MessagesFlushed { connection })
}
