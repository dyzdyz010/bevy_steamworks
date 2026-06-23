use crate::matchmaking::*;

impl SteamworksMatchmakingState {
    /// Returns the most recent lobby metadata count read.
    pub fn last_lobby_data_count(&self) -> Option<&SteamworksLobbyDataCount> {
        self.last_lobby_data_count.as_ref()
    }

    /// Returns bounded lobby metadata count snapshots by lobby.
    pub fn lobby_data_counts(&self) -> &[SteamworksLobbyDataCount] {
        &self.lobby_data_counts
    }

    /// Returns the latest metadata count snapshot for a lobby.
    pub fn lobby_data_count(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyDataCount> {
        self.lobby_data_counts
            .iter()
            .find(|count| count.lobby == lobby)
    }

    /// Returns the latest known metadata entry count for a lobby.
    pub fn lobby_data_count_value(&self, lobby: steamworks::LobbyId) -> Option<u32> {
        self.lobby_data_count(lobby).map(|count| count.count)
    }

    /// Returns the most recent lobby metadata value read.
    pub fn last_lobby_data(&self) -> Option<&SteamworksLobbyDataValue> {
        self.last_lobby_data.as_ref()
    }

    /// Returns bounded lobby metadata value snapshots by lobby and key.
    pub fn lobby_data_values(&self) -> &[SteamworksLobbyDataValue] {
        &self.lobby_data_values
    }

    /// Returns the latest metadata value snapshot for a lobby/key pair.
    pub fn lobby_data_value(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataValue> {
        let key = key.as_ref();
        self.lobby_data_values
            .iter()
            .find(|value| value.lobby == lobby && value.key.as_str() == key)
    }

    /// Returns the latest known lobby metadata value for a key.
    ///
    /// The outer `Option` distinguishes no cached data from cached data. The
    /// inner `Option` is `None` when a direct read reported no value or a full
    /// metadata snapshot did not include the key.
    pub fn lobby_data(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<Option<&str>> {
        let key = key.as_ref();
        self.lobby_data_value(lobby, key)
            .map(|value| value.value.as_deref())
            .or_else(|| {
                self.all_lobby_data(lobby).map(|entries| {
                    entries
                        .entries
                        .iter()
                        .find(|(entry_key, _)| entry_key == key)
                        .map(|(_, value)| value.as_str())
                })
            })
    }

    /// Returns whether the latest known metadata for a lobby has a value for a key.
    pub fn has_lobby_data(&self, lobby: steamworks::LobbyId, key: impl AsRef<str>) -> Option<bool> {
        self.lobby_data(lobby, key).map(|value| value.is_some())
    }

    /// Returns the most recent lobby metadata entry read by index.
    pub fn last_lobby_data_entry(&self) -> Option<&SteamworksLobbyDataEntry> {
        self.last_lobby_data_entry.as_ref()
    }

    /// Returns bounded lobby metadata entry snapshots by lobby and index.
    pub fn lobby_data_entries(&self) -> &[SteamworksLobbyDataEntry] {
        &self.lobby_data_entries
    }

    /// Returns the latest metadata entry snapshot for a lobby/index pair.
    pub fn lobby_data_entry(
        &self,
        lobby: steamworks::LobbyId,
        index: u32,
    ) -> Option<&SteamworksLobbyDataEntry> {
        self.lobby_data_entries
            .iter()
            .find(|entry| entry.lobby == lobby && entry.index == index)
    }

    /// Returns the most recent full lobby metadata snapshot read.
    pub fn last_all_lobby_data(&self) -> Option<&SteamworksLobbyDataEntries> {
        self.last_all_lobby_data.as_ref()
    }

    /// Returns bounded full metadata snapshots by lobby.
    pub fn all_lobby_data_entries(&self) -> &[SteamworksLobbyDataEntries] {
        &self.all_lobby_data
    }

    /// Returns the latest full metadata snapshot for a lobby.
    pub fn all_lobby_data(
        &self,
        lobby: steamworks::LobbyId,
    ) -> Option<&SteamworksLobbyDataEntries> {
        self.all_lobby_data
            .iter()
            .find(|entries| entries.lobby == lobby)
    }

    /// Returns one value from the latest full lobby metadata snapshot.
    pub fn all_lobby_data_value(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&str> {
        let key = key.as_ref();
        self.all_lobby_data(lobby).and_then(|entries| {
            entries
                .entries
                .iter()
                .find(|(entry_key, _)| entry_key == key)
                .map(|(_, value)| value.as_str())
        })
    }

    /// Returns the most recent lobby metadata key set.
    pub fn last_lobby_data_set(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_data_set.as_ref()
    }

    /// Returns bounded lobby metadata set snapshots by lobby and key.
    pub fn lobby_data_sets(&self) -> &[SteamworksLobbyDataMutation] {
        &self.lobby_data_sets
    }

    /// Returns the latest metadata set snapshot for a lobby/key pair.
    pub fn lobby_data_set(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataMutation> {
        let key = key.as_ref();
        self.lobby_data_sets
            .iter()
            .find(|set| set.lobby == lobby && set.key.as_str() == key)
    }

    /// Returns the most recent lobby metadata key deleted.
    pub fn last_lobby_data_deleted(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_data_deleted.as_ref()
    }

    /// Returns bounded lobby metadata deletion snapshots by lobby and key.
    pub fn lobby_data_deletions(&self) -> &[SteamworksLobbyDataMutation] {
        &self.lobby_data_deletions
    }

    /// Returns the latest metadata deletion snapshot for a lobby/key pair.
    pub fn lobby_data_deletion(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataMutation> {
        let key = key.as_ref();
        self.lobby_data_deletions
            .iter()
            .find(|deleted| deleted.lobby == lobby && deleted.key.as_str() == key)
    }

    /// Returns the most recent local-user lobby metadata key set.
    pub fn last_lobby_member_data_set(&self) -> Option<&SteamworksLobbyDataMutation> {
        self.last_lobby_member_data_set.as_ref()
    }

    /// Returns bounded local-user lobby metadata set snapshots by lobby and key.
    pub fn lobby_member_data_sets(&self) -> &[SteamworksLobbyDataMutation] {
        &self.lobby_member_data_sets
    }

    /// Returns the latest local-user metadata set snapshot for a lobby/key pair.
    pub fn lobby_member_data_set(
        &self,
        lobby: steamworks::LobbyId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyDataMutation> {
        let key = key.as_ref();
        self.lobby_member_data_sets
            .iter()
            .find(|set| set.lobby == lobby && set.key.as_str() == key)
    }

    /// Returns the most recent lobby member metadata value read.
    pub fn last_lobby_member_data(&self) -> Option<&SteamworksLobbyMemberDataValue> {
        self.last_lobby_member_data.as_ref()
    }

    /// Returns bounded lobby member metadata value snapshots by lobby, user, and key.
    pub fn lobby_member_data_values(&self) -> &[SteamworksLobbyMemberDataValue] {
        &self.lobby_member_data_values
    }

    /// Returns the latest member metadata value snapshot for a lobby/user/key triple.
    pub fn lobby_member_data_value(
        &self,
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
        key: impl AsRef<str>,
    ) -> Option<&SteamworksLobbyMemberDataValue> {
        let key = key.as_ref();
        self.lobby_member_data_values
            .iter()
            .find(|value| value.lobby == lobby && value.user == user && value.key.as_str() == key)
    }

    /// Returns the latest known member metadata value for a lobby/user/key triple.
    ///
    /// The outer `Option` distinguishes no cached read from a completed read.
    /// The inner `Option` is `None` when Steam reported no value for the key.
    pub fn lobby_member_data(
        &self,
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
        key: impl AsRef<str>,
    ) -> Option<Option<&str>> {
        self.lobby_member_data_value(lobby, user, key)
            .map(|value| value.value.as_deref())
    }

    /// Returns whether the latest known member metadata has a value for a lobby/user/key triple.
    pub fn has_lobby_member_data(
        &self,
        lobby: steamworks::LobbyId,
        user: steamworks::SteamId,
        key: impl AsRef<str>,
    ) -> Option<bool> {
        self.lobby_member_data(lobby, user, key)
            .map(|value| value.is_some())
    }
}
