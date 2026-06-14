use super::{SteamworksLobbyListFilter, SteamworksMatchmakingError};

pub(super) fn apply_lobby_list_filter(
    matchmaking: &steamworks::Matchmaking,
    filter: &SteamworksLobbyListFilter,
) -> Result<(), SteamworksMatchmakingError> {
    for item in &filter.string {
        let key = lobby_key(&item.key)?;
        matchmaking.add_request_lobby_list_string_filter(steamworks::StringFilter(
            key,
            &item.value,
            item.comparison,
        ));
    }

    for item in &filter.number {
        let key = lobby_key(&item.key)?;
        matchmaking.add_request_lobby_list_numerical_filter(steamworks::NumberFilter(
            key,
            item.value,
            item.comparison,
        ));
    }

    for item in &filter.near_value {
        let key = lobby_key(&item.key)?;
        matchmaking
            .add_request_lobby_list_near_value_filter(steamworks::NearFilter(key, item.value));
    }

    if let Some(open_slots) = filter.open_slots {
        matchmaking.set_request_lobby_list_slots_available_filter(open_slots);
    }
    if let Some(distance) = filter.distance {
        matchmaking.set_request_lobby_list_distance_filter(distance);
    }
    if let Some(max_results) = filter.max_results {
        matchmaking.set_request_lobby_list_result_count_filter(max_results);
    }

    Ok(())
}

pub(super) fn lobby_key(key: &str) -> Result<steamworks::LobbyKey<'_>, SteamworksMatchmakingError> {
    steamworks::LobbyKey::try_new(key)
        .map_err(|_| SteamworksMatchmakingError::lobby_key_too_long(key))
}
