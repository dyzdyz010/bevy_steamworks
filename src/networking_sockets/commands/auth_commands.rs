use crate::{SteamworksClient, SteamworksServer};

use super::super::{SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsOperation};
use super::helpers::networking_sockets;

pub(super) fn init_authentication(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, _) = networking_sockets(client, server)?;
    Ok(
        SteamworksNetworkingSocketsOperation::AuthenticationInitialized {
            availability: sockets.init_authentication(),
        },
    )
}

pub(super) fn get_authentication_status(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Result<SteamworksNetworkingSocketsOperation, SteamworksNetworkingSocketsError> {
    let (sockets, _) = networking_sockets(client, server)?;
    Ok(
        SteamworksNetworkingSocketsOperation::AuthenticationStatusRead {
            availability: sockets.get_authentication_status(),
        },
    )
}
