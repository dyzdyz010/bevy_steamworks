use std::sync::{Arc, Mutex};

use super::{
    SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksServerListFilters, SteamworksServerListKind, SteamworksServerListRequestId,
    STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
};

pub(super) fn validate_server_index_in_request(
    handle: &Arc<Mutex<steamworks::ServerListRequest>>,
    request: SteamworksServerListRequestId,
    server: i32,
) -> Result<(), SteamworksMatchmakingServersError> {
    let count = handle
        .lock()
        .expect("Steamworks server-list request mutex was poisoned")
        .get_server_count()
        .map_err(|_| SteamworksMatchmakingServersError::ServerListRequestReleased { request })?;
    if server >= count {
        return Err(SteamworksMatchmakingServersError::ServerIndexOutOfRange {
            request,
            server,
            count,
        });
    }

    Ok(())
}

pub(super) fn validate_command(
    command: &SteamworksMatchmakingServersCommand,
) -> Result<(), SteamworksMatchmakingServersError> {
    match command {
        SteamworksMatchmakingServersCommand::RequestServerList { kind, filters, .. } => {
            validate_filters(*kind, filters)
        }
        SteamworksMatchmakingServersCommand::RefreshServer { server, .. }
        | SteamworksMatchmakingServersCommand::GetServerDetails { server, .. } => {
            validate_server_index(*server)
        }
        _ => Ok(()),
    }
}

fn validate_filters(
    kind: SteamworksServerListKind,
    filters: &SteamworksServerListFilters,
) -> Result<(), SteamworksMatchmakingServersError> {
    if kind == SteamworksServerListKind::Lan && !filters.is_empty() {
        return Err(SteamworksMatchmakingServersError::LanFiltersUnsupported);
    }

    for (key, value) in filters.entries() {
        validate_filter_text("filter key", key)?;
        validate_filter_text("filter value", value)?;
    }

    Ok(())
}

fn validate_filter_text(
    field: &'static str,
    value: &str,
) -> Result<(), SteamworksMatchmakingServersError> {
    if value.as_bytes().contains(&0) {
        return Err(SteamworksMatchmakingServersError::invalid_string(field));
    }
    if value.len() > STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES {
        return Err(SteamworksMatchmakingServersError::FilterTooLong {
            field,
            requested: value.len(),
            max_supported: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
        });
    }
    Ok(())
}

fn validate_server_index(server: i32) -> Result<(), SteamworksMatchmakingServersError> {
    if server < 0 {
        Err(SteamworksMatchmakingServersError::InvalidServerIndex { server })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(&SteamworksMatchmakingServersCommand::RequestServerList {
                app_id: steamworks::AppId(480),
                kind: SteamworksServerListKind::Lan,
                filters: SteamworksServerListFilters::new().with("map", "arena"),
            }),
            Err(SteamworksMatchmakingServersError::LanFiltersUnsupported)
        );
        assert_eq!(
            validate_command(
                &SteamworksMatchmakingServersCommand::request_internet_server_list(
                    480,
                    SteamworksServerListFilters::new().with("bad\0key", "arena"),
                )
            ),
            Err(SteamworksMatchmakingServersError::InvalidString {
                field: "filter key",
            })
        );
        assert_eq!(
            validate_command(
                &SteamworksMatchmakingServersCommand::request_internet_server_list(
                    480,
                    SteamworksServerListFilters::new().with(
                        "map",
                        "x".repeat(STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES + 1),
                    ),
                )
            ),
            Err(SteamworksMatchmakingServersError::FilterTooLong {
                field: "filter value",
                requested: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES + 1,
                max_supported: STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
            })
        );
        assert_eq!(
            validate_command(&SteamworksMatchmakingServersCommand::get_server_details(
                SteamworksServerListRequestId::from_raw(1),
                -1,
            )),
            Err(SteamworksMatchmakingServersError::InvalidServerIndex { server: -1 })
        );
    }
}
