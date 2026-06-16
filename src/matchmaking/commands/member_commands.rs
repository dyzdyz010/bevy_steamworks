use super::super::SteamworksMatchmakingOperation;

pub(super) fn set_lobby_member_data(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    key: String,
    value: String,
) -> SteamworksMatchmakingOperation {
    matchmaking.set_lobby_member_data(lobby, &key, &value);
    SteamworksMatchmakingOperation::LobbyMemberDataSet { lobby, key }
}

pub(super) fn read_lobby_member_data(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
    user: steamworks::SteamId,
    key: String,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyMemberDataRead {
        lobby,
        user,
        value: matchmaking.get_lobby_member_data(lobby, user, &key),
        key,
    }
}

pub(super) fn read_lobby_member_limit(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyMemberLimitRead {
        lobby,
        limit: matchmaking.lobby_member_limit(lobby),
    }
}

pub(super) fn read_lobby_owner(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyOwnerRead {
        lobby,
        owner: matchmaking.lobby_owner(lobby),
    }
}

pub(super) fn read_lobby_member_count(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyMemberCountRead {
        lobby,
        count: matchmaking.lobby_member_count(lobby),
    }
}

pub(super) fn list_lobby_members(
    matchmaking: &steamworks::Matchmaking,
    lobby: steamworks::LobbyId,
) -> SteamworksMatchmakingOperation {
    SteamworksMatchmakingOperation::LobbyMembersListed {
        lobby,
        members: matchmaking.lobby_members(lobby),
    }
}
