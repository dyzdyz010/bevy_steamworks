use std::net::SocketAddr;

use crate::{SteamworksClient, SteamworksServer};

use super::super::{
    handles::{
        SteamworksNetworkingSocketsConnectionMetadata, SteamworksNetworkingSocketsHandleStorage,
    },
    snapshots::{snapshot_connection_info, snapshot_realtime_status},
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsConnectionId,
    SteamworksNetworkingSocketsConnectionTarget, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsPollGroupId,
};
use super::helpers::{
    connection_owner, connection_user_data_from_info_result, networking_sockets,
    networking_sockets_for_owner, poll_group_owner, steam_config_entries,
};

pub(super) fn connect_by_ip_address(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    address: SocketAddr,
    options: &[SteamworksNetworkingSocketsConfigEntry],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, owner) = networking_sockets(client, server)?;
    let options = steam_config_entries(options);
    let connection = sockets
        .connect_by_ip_address(address, options)
        .map_err(|_| {
            SteamworksNetworkingSocketsError::invalid_handle(
                "networking_sockets.connect_by_ip_address",
            )
        })?;
    let connection = handles.insert_connection(
        connection,
        SteamworksNetworkingSocketsConnectionMetadata::independent(),
        owner,
    );
    Ok(SteamworksNetworkingSocketsOperation::ConnectionCreated {
        connection,
        target: SteamworksNetworkingSocketsConnectionTarget::Ip(address),
    })
}

pub(super) fn connect_p2p(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    identity: &steamworks::networking_types::NetworkingIdentity,
    remote_virtual_port: i32,
    options: &[SteamworksNetworkingSocketsConfigEntry],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, owner) = networking_sockets(client, server)?;
    let options = steam_config_entries(options);
    let connection = sockets
        .connect_p2p(identity.clone(), remote_virtual_port, options)
        .map_err(|_| {
            SteamworksNetworkingSocketsError::invalid_handle("networking_sockets.connect_p2p")
        })?;
    let connection = handles.insert_connection(
        connection,
        SteamworksNetworkingSocketsConnectionMetadata::independent(),
        owner,
    );
    Ok(SteamworksNetworkingSocketsOperation::ConnectionCreated {
        connection,
        target: SteamworksNetworkingSocketsConnectionTarget::P2p {
            identity: identity.clone(),
            remote_virtual_port,
        },
    })
}

pub(super) fn get_connection_info(
    handles: &SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    let info = connection_ref
        .info()
        .map_err(|_| SteamworksNetworkingSocketsError::invalid_handle("net_connection.info"))?;
    Ok(SteamworksNetworkingSocketsOperation::ConnectionInfoRead {
        info: snapshot_connection_info(connection, info),
    })
}

pub(super) fn get_connection_user_data(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    let user_data =
        connection_user_data_from_info_result(connection_ref.info().map(|info| info.user_data()))?;
    handles.update_connection_user_data(connection, user_data);
    Ok(
        SteamworksNetworkingSocketsOperation::ConnectionUserDataRead {
            connection,
            user_data,
        },
    )
}

pub(super) fn get_realtime_connection_status(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    lanes: u32,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let owner = connection_owner(handles, connection)?;
    let (sockets, _) = networking_sockets_for_owner(client, server, owner)?;
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    let (info, lanes) = sockets
        .get_realtime_connection_status(connection_ref, lanes as i32)
        .map_err(|source| {
            SteamworksNetworkingSocketsError::steam_error(
                "networking_sockets.get_realtime_connection_status",
                source,
            )
        })?;
    Ok(
        SteamworksNetworkingSocketsOperation::RealtimeConnectionStatusRead {
            status: snapshot_realtime_status(connection, info, lanes),
        },
    )
}

pub(super) fn set_connection_poll_group(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    poll_group: SteamworksNetworkingSocketsPollGroupId,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_owner = connection_owner(handles, connection)?;
    let poll_group_owner = poll_group_owner(handles, poll_group)?;
    if connection_owner != poll_group_owner {
        return Err(SteamworksNetworkingSocketsError::HandleOwnerMismatch {
            connection,
            poll_group,
        });
    }
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    let poll_group_ref = handles
        .poll_groups
        .get(&poll_group)
        .ok_or(SteamworksNetworkingSocketsError::PollGroupNotFound { id: poll_group })?;
    connection_ref.set_poll_group(poll_group_ref);
    handles.set_connection_poll_group(connection, poll_group);
    Ok(
        SteamworksNetworkingSocketsOperation::ConnectionPollGroupSet {
            connection,
            poll_group,
        },
    )
}

pub(super) fn clear_connection_poll_group(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    connection_ref.clear_poll_group().map_err(|_| {
        SteamworksNetworkingSocketsError::invalid_handle("net_connection.clear_poll_group")
    })?;
    handles.clear_connection_poll_group(connection);
    Ok(SteamworksNetworkingSocketsOperation::ConnectionPollGroupCleared { connection })
}

pub(super) fn configure_connection_lanes(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    lane_priorities: &[i32],
    lane_weights: &[u16],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let owner = connection_owner(handles, connection)?;
    let (sockets, _) = networking_sockets_for_owner(client, server, owner)?;
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    sockets
        .configure_connection_lanes(
            connection_ref,
            lane_priorities.len() as i32,
            lane_priorities,
            lane_weights,
        )
        .map_err(|source| {
            SteamworksNetworkingSocketsError::steam_error(
                "networking_sockets.configure_connection_lanes",
                source,
            )
        })?;
    Ok(
        SteamworksNetworkingSocketsOperation::ConnectionLanesConfigured {
            connection,
            lanes: lane_priorities.len(),
        },
    )
}

pub(super) fn set_connection_user_data(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    user_data: i64,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    connection_ref
        .set_connection_user_data(user_data)
        .map_err(|_| {
            SteamworksNetworkingSocketsError::invalid_handle(
                "net_connection.set_connection_user_data",
            )
        })?;
    handles.update_connection_user_data(connection, user_data);
    Ok(
        SteamworksNetworkingSocketsOperation::ConnectionUserDataSet {
            connection,
            user_data,
        },
    )
}

pub(super) fn set_connection_name(
    handles: &SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    name: &str,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_ref = handles
        .connections
        .get(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    connection_ref.set_connection_name(name);
    Ok(SteamworksNetworkingSocketsOperation::ConnectionNameSet {
        connection,
        name: name.to_owned(),
    })
}

pub(super) fn close_connection(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    connection: SteamworksNetworkingSocketsConnectionId,
    reason: steamworks::networking_types::NetConnectionEnd,
    debug: Option<&str>,
    enable_linger: bool,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let connection_handle = handles
        .remove_connection(&connection)
        .ok_or(SteamworksNetworkingSocketsError::ConnectionNotFound { id: connection })?;
    let close_succeeded = connection_handle.close(reason, debug, enable_linger);
    Ok(SteamworksNetworkingSocketsOperation::ConnectionClosed {
        connection,
        close_succeeded,
    })
}
