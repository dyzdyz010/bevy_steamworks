use crate::{SteamworksClient, SteamworksServer};

use super::super::{
    handles::{SteamworksNetworkingSocketsHandleOwner, SteamworksNetworkingSocketsHandleStorage},
    snapshots::snapshot_message,
    SteamworksNetworkingSocketsConnectionId, SteamworksNetworkingSocketsConnectionMessages,
    SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsMessageSendResult,
    SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsOutboundMessage,
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
    let client = client.ok_or(SteamworksNetworkingSocketsError::ClientUnavailable)?;
    let mut client_outbound = Vec::new();
    let mut server_outbound = Vec::new();

    for (index, message) in messages.iter().enumerate() {
        let owner = handles.connection_owner(message.connection).ok_or(
            SteamworksNetworkingSocketsError::ConnectionNotFound {
                id: message.connection,
            },
        )?;
        if owner == SteamworksNetworkingSocketsHandleOwner::Server && server.is_none() {
            return Err(SteamworksNetworkingSocketsError::ServerUnavailable);
        }
        let connection_ref = handles.connections.get(&message.connection).ok_or(
            SteamworksNetworkingSocketsError::ConnectionNotFound {
                id: message.connection,
            },
        )?;
        let outbound = allocate_outbound_message(client, connection_ref, message)?;
        match owner {
            SteamworksNetworkingSocketsHandleOwner::Client => {
                client_outbound.push((index, outbound));
            }
            SteamworksNetworkingSocketsHandleOwner::Server => {
                server_outbound.push((index, outbound));
            }
        }
    }

    let mut send_results = std::iter::repeat_with(|| None)
        .take(messages.len())
        .collect::<Vec<_>>();
    send_message_batch(
        Some(client),
        server,
        SteamworksNetworkingSocketsHandleOwner::Client,
        client_outbound,
        messages,
        &mut send_results,
    )?;
    send_message_batch(
        Some(client),
        server,
        SteamworksNetworkingSocketsHandleOwner::Server,
        server_outbound,
        messages,
        &mut send_results,
    )?;

    let messages: Vec<SteamworksNetworkingSocketsMessageSendResult> = send_results
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            SteamworksNetworkingSocketsError::operation_failed("networking_sockets.send_messages")
        })?;
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

fn send_message_batch(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    owner: SteamworksNetworkingSocketsHandleOwner,
    batch: Vec<(usize, steamworks::networking_types::NetworkingMessage)>,
    source_messages: &[SteamworksNetworkingSocketsOutboundMessage],
    send_results: &mut [Option<SteamworksNetworkingSocketsMessageSendResult>],
) -> Result<(), SteamworksNetworkingSocketsError> {
    if batch.is_empty() {
        return Ok(());
    }

    let (indices, outbound): (Vec<_>, Vec<_>) = batch.into_iter().unzip();
    let (sockets, _) = networking_sockets_for_owner(client, server, owner)?;
    for (index, result) in indices.into_iter().zip(sockets.send_messages(outbound)) {
        let message = &source_messages[index];
        send_results[index] = Some(SteamworksNetworkingSocketsMessageSendResult {
            connection: message.connection,
            send_flags: message.send_flags,
            channel: message.channel,
            bytes: message.data.len(),
            user_data: message.user_data,
            result: result.map(u64::from),
        });
    }

    Ok(())
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

pub(super) fn receive_all_messages(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    batch_size_per_connection: usize,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let mut connections = handles.connections.keys().copied().collect::<Vec<_>>();
    connections.sort_by_key(|connection| connection.raw());

    let mut batches = Vec::with_capacity(connections.len());
    for connection in connections {
        if !handles.connections.contains_key(&connection) {
            continue;
        }
        let SteamworksNetworkingSocketsOperation::MessagesReceived {
            connection,
            messages,
        } = receive_messages(handles, connection, batch_size_per_connection)?
        else {
            unreachable!("receive_messages returns MessagesReceived");
        };
        batches.push(SteamworksNetworkingSocketsConnectionMessages {
            connection,
            messages,
        });
    }

    Ok(SteamworksNetworkingSocketsOperation::AllMessagesReceived {
        connections: batches,
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
