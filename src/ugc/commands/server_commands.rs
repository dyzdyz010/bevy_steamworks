use crate::SteamworksServer;

use super::super::{SteamworksUgcError, SteamworksUgcOperation, SteamworksUgcWorkshopDepotId};

pub(super) fn init_workshop_for_game_server(
    server: Option<&SteamworksServer>,
    workshop_depot: SteamworksUgcWorkshopDepotId,
    folder: String,
) -> Result<SteamworksUgcOperation, SteamworksUgcError> {
    let server = server.ok_or(SteamworksUgcError::ServerUnavailable)?;
    if !server
        .ugc()
        .init_for_game_server(workshop_depot.raw(), &folder)
    {
        return Err(SteamworksUgcError::operation_failed(
            "ugc.init_for_game_server",
        ));
    }
    Ok(SteamworksUgcOperation::GameServerWorkshopInitialized {
        workshop_depot,
        folder,
    })
}
