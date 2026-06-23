use crate::matchmaking::*;

impl SteamworksMatchmakingState {
    /// Returns the most recent lobby member limit read.
    pub fn last_lobby_member_limit(&self) -> Option<&SteamworksLobbyMemberLimit> {
        self.last_lobby_member_limit.as_ref()
    }

    /// Returns bounded lobby member limit snapshots by lobby.
    pub fn lobby_member_limits(&self) -> &[SteamworksLobbyMemberLimit] {
        &self.lobby_member_limits
    }

    /// Returns the latest member limit snapshot for a lobby.
    pub fn lobby_member_limit(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyMemberLimit> {
        self.lobby_member_limits
            .iter()
            .find(|limit| limit.lobby == lobby)
    }

    /// Returns the latest known member limit for a lobby, preserving an unknown limit as `Some(None)`.
    pub fn lobby_member_limit_value(&self, lobby: steamworks::LobbyId) -> Option<Option<usize>> {
        self.lobby_member_limit(lobby).map(|limit| limit.limit)
    }

    /// Returns the most recent lobby owner read.
    pub fn last_lobby_owner(&self) -> Option<&SteamworksLobbyOwner> {
        self.last_lobby_owner.as_ref()
    }

    /// Returns bounded lobby owner snapshots by lobby.
    pub fn lobby_owners(&self) -> &[SteamworksLobbyOwner] {
        &self.lobby_owners
    }

    /// Returns the latest owner snapshot for a lobby.
    pub fn lobby_owner(&self, lobby: steamworks::LobbyId) -> Option<&SteamworksLobbyOwner> {
        self.lobby_owners.iter().find(|owner| owner.lobby == lobby)
    }

    /// Returns the latest known lobby owner Steam ID.
    pub fn lobby_owner_id(&self, lobby: steamworks::LobbyId) -> Option<steamworks::SteamId> {
        self.lobby_owner(lobby).map(|owner| owner.owner)
    }

    /// Returns the most recent lobby member count read.
    pub fn last_lobby_member_count(&self) -> Option<&SteamworksLobbyMemberCount> {
        self.last_lobby_member_count.as_ref()
    }

    /// Returns bounded lobby member count snapshots by lobby.
    pub fn lobby_member_counts(&self) -> &[SteamworksLobbyMemberCount] {
        &self.lobby_member_counts
    }

    /// Returns the latest member count snapshot for a lobby.
    pub fn lobby_member_count(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyMemberCount> {
        self.lobby_member_counts
            .iter()
            .find(|count| count.lobby == lobby)
    }

    /// Returns the latest known lobby member count.
    pub fn lobby_member_count_value(&self, lobby: steamworks::LobbyId) -> Option<usize> {
        self.lobby_member_count(lobby).map(|count| count.count)
    }

    /// Returns the most recent lobby member list read.
    pub fn last_lobby_members(&self) -> Option<&SteamworksLobbyMembers> {
        self.last_lobby_members.as_ref()
    }

    /// Returns bounded lobby member list snapshots by lobby.
    pub fn lobby_member_lists(&self) -> &[SteamworksLobbyMembers] {
        &self.lobby_member_lists
    }

    /// Returns the latest member list snapshot for a lobby.
    pub fn lobby_members(&self, lobby: steamworks::LobbyId) -> Option<&SteamworksLobbyMembers> {
        self.lobby_member_lists
            .iter()
            .find(|members| members.lobby == lobby)
    }

    /// Returns the latest known member IDs for a lobby.
    pub fn lobby_member_ids(&self, lobby: steamworks::LobbyId) -> Option<&[steamworks::SteamId]> {
        self.lobby_members(lobby)
            .map(|members| members.members.as_slice())
    }

    /// Returns whether the latest known member list for a lobby contains a user.
    pub fn has_lobby_member(
        &self,
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
    ) -> Option<bool> {
        self.lobby_member_ids(lobby)
            .map(|members| members.contains(&user))
    }

    /// Returns the most recent lobby joinability value set.
    pub fn last_lobby_joinability(&self) -> Option<&SteamworksLobbyJoinability> {
        self.last_lobby_joinability.as_ref()
    }

    /// Returns bounded lobby joinability snapshots by lobby.
    pub fn lobby_joinabilities(&self) -> &[SteamworksLobbyJoinability] {
        &self.lobby_joinabilities
    }

    /// Returns the latest joinability snapshot for a lobby.
    pub fn lobby_joinability(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyJoinability> {
        self.lobby_joinabilities
            .iter()
            .find(|joinability| joinability.lobby == lobby)
    }

    /// Returns the latest known joinable flag for a lobby.
    pub fn lobby_joinable(&self, lobby: steamworks::LobbyId) -> Option<bool> {
        self.lobby_joinability(lobby)
            .map(|joinability| joinability.joinable)
    }
}
