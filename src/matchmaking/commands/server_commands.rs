use std::net::SocketAddrV4;

use super::super::{SteamworksLobbyGameServer, SteamworksMatchmakingOperation};

pub(super) fn set_lobby_game_server(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    address: SocketAddrV4,
    steam_id: Option<steamworks::SteamId>,
) -> SteamworksMatchmakingOperation {
    matchmaking.set_lobby_game_server(lobby, address, steam_id);
    SteamworksMatchmakingOperation::LobbyGameServerSet {
        lobby,
        server: SteamworksLobbyGameServer { address, steam_id },
    }
}

pub(super) fn read_lobby_game_server(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    let server = matchmaking
        .get_lobby_game_server(lobby)
        .map(|(address, steam_id)| SteamworksLobbyGameServer { address, steam_id });
    SteamworksMatchmakingOperation::LobbyGameServerRead { lobby, server }
}
