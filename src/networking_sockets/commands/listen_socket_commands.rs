use std::net::SocketAddr;

use crate::{SteamworksClient, SteamworksServer};

use super::super::{
    handles::SteamworksNetworkingSocketsHandleStorage, SteamworksListenSocketId,
    SteamworksNetworkingSocketsConfigEntry, SteamworksNetworkingSocketsError,
    SteamworksNetworkingSocketsListenEndpoint, SteamworksNetworkingSocketsOperation,
};
use super::helpers::{networking_sockets, server_networking_sockets, steam_config_entries};

pub(super) fn create_listen_socket_ip(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    local_address: SocketAddr,
    options: &[SteamworksNetworkingSocketsConfigEntry],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, owner) = networking_sockets(client, server)?;
    let options = steam_config_entries(options);
    let socket = sockets
        .create_listen_socket_ip(local_address, options)
        .map_err(|_| {
            SteamworksNetworkingSocketsError::invalid_handle(
                "networking_sockets.create_listen_socket_ip",
            )
        })?;
    let listen_socket = handles.insert_listen_socket(socket, owner);
    Ok(SteamworksNetworkingSocketsOperation::ListenSocketCreated {
        listen_socket,
        endpoint: SteamworksNetworkingSocketsListenEndpoint::Ip(local_address),
    })
}

pub(super) fn create_listen_socket_p2p(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    local_virtual_port: i32,
    options: &[SteamworksNetworkingSocketsConfigEntry],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, owner) = networking_sockets(client, server)?;
    let options = steam_config_entries(options);
    let socket = sockets
        .create_listen_socket_p2p(local_virtual_port, options)
        .map_err(|_| {
            SteamworksNetworkingSocketsError::invalid_handle(
                "networking_sockets.create_listen_socket_p2p",
            )
        })?;
    let listen_socket = handles.insert_listen_socket(socket, owner);
    Ok(SteamworksNetworkingSocketsOperation::ListenSocketCreated {
        listen_socket,
        endpoint: SteamworksNetworkingSocketsListenEndpoint::P2p { local_virtual_port },
    })
}

pub(super) fn create_hosted_dedicated_server_listen_socket(
    server: Option<&SteamworksServer>,
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    local_virtual_port: u32,
    options: &[SteamworksNetworkingSocketsConfigEntry],
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, owner) = server_networking_sockets(server)?;
    let options = steam_config_entries(options);
    let socket = sockets
        .create_hosted_dedicated_server_listen_socket(local_virtual_port, options)
        .map_err(|_| {
            SteamworksNetworkingSocketsError::invalid_handle(
                "networking_sockets.create_hosted_dedicated_server_listen_socket",
            )
        })?;
    let listen_socket = handles.insert_listen_socket(socket, owner);
    Ok(SteamworksNetworkingSocketsOperation::ListenSocketCreated {
        listen_socket,
        endpoint: SteamworksNetworkingSocketsListenEndpoint::HostedDedicatedServer {
            local_virtual_port,
        },
    })
}

pub(super) fn close_listen_socket(
    handles: &mut SteamworksNetworkingSocketsHandleStorage,
    listen_socket: SteamworksListenSocketId,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    if !handles.listen_sockets.contains_key(&listen_socket) {
        return Err(SteamworksNetworkingSocketsError::ListenSocketNotFound { id: listen_socket });
    }
    let closed_connections = handles.remove_connections_for_listen_socket(listen_socket);
    handles.remove_listen_socket(&listen_socket);
    Ok(SteamworksNetworkingSocketsOperation::ListenSocketClosed {
        listen_socket,
        closed_connections,
    })
}
