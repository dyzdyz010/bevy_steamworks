use super::SteamworksMatchmakingCommand;

impl std::fmt::Debug for SteamworksMatchmakingCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestLobbyList { filter } => f
                .debug_struct("RequestLobbyList")
                .field("filter", filter)
                .finish(),
            Self::CreateLobby {
                lobby_type,
                max_members,
            } => f
                .debug_struct("CreateLobby")
                .field("lobby_type", lobby_type)
                .field("max_members", max_members)
                .finish(),
            Self::JoinLobby { lobby } => f.debug_struct("JoinLobby").field("lobby", lobby).finish(),
            Self::LeaveLobby { lobby } => {
                f.debug_struct("LeaveLobby").field("lobby", lobby).finish()
            }
            Self::GetLobbyDataCount { lobby } => f
                .debug_struct("GetLobbyDataCount")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyData { lobby, key } => f
                .debug_struct("GetLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::GetLobbyDataByIndex { lobby, index } => f
                .debug_struct("GetLobbyDataByIndex")
                .field("lobby", lobby)
                .field("index", index)
                .finish(),
            Self::GetAllLobbyData { lobby } => f
                .debug_struct("GetAllLobbyData")
                .field("lobby", lobby)
                .finish(),
            Self::SetLobbyData { lobby, key, value } => f
                .debug_struct("SetLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::DeleteLobbyData { lobby, key } => f
                .debug_struct("DeleteLobbyData")
                .field("lobby", lobby)
                .field("key", key)
                .finish(),
            Self::SetLobbyMemberData { lobby, key, value } => f
                .debug_struct("SetLobbyMemberData")
                .field("lobby", lobby)
                .field("key", key)
                .field("value", value)
                .finish(),
            Self::GetLobbyMemberData { lobby, user, key } => f
                .debug_struct("GetLobbyMemberData")
                .field("lobby", lobby)
                .field("user", user)
                .field("key", key)
                .finish(),
            Self::GetLobbyMemberLimit { lobby } => f
                .debug_struct("GetLobbyMemberLimit")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyOwner { lobby } => f
                .debug_struct("GetLobbyOwner")
                .field("lobby", lobby)
                .finish(),
            Self::GetLobbyMemberCount { lobby } => f
                .debug_struct("GetLobbyMemberCount")
                .field("lobby", lobby)
                .finish(),
            Self::ListLobbyMembers { lobby } => f
                .debug_struct("ListLobbyMembers")
                .field("lobby", lobby)
                .finish(),
            Self::SetLobbyJoinable { lobby, joinable } => f
                .debug_struct("SetLobbyJoinable")
                .field("lobby", lobby)
                .field("joinable", joinable)
                .finish(),
            Self::SendLobbyChatMessage { lobby, data } => f
                .debug_struct("SendLobbyChatMessage")
                .field("lobby", lobby)
                .field("data_len", &data.len())
                .finish(),
            Self::GetLobbyChatEntry {
                lobby,
                chat_id,
                max_bytes,
            } => f
                .debug_struct("GetLobbyChatEntry")
                .field("lobby", lobby)
                .field("chat_id", chat_id)
                .field("max_bytes", max_bytes)
                .finish(),
            Self::SetLobbyGameServer {
                lobby,
                address,
                steam_id,
            } => f
                .debug_struct("SetLobbyGameServer")
                .field("lobby", lobby)
                .field("address", address)
                .field("steam_id", steam_id)
                .finish(),
            Self::GetLobbyGameServer { lobby } => f
                .debug_struct("GetLobbyGameServer")
                .field("lobby", lobby)
                .finish(),
        }
    }
}
