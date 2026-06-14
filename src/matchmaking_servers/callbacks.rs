use super::{
    requests::SteamworksMatchmakingServersAsyncResults, SteamworksGameServerItem,
    SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
    SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersResult,
    SteamworksServerListRequestId,
};

pub(super) fn server_list_callbacks(
    request: SteamworksServerListRequestId,
    async_results: SteamworksMatchmakingServersAsyncResults,
) -> steamworks::ServerListCallbacks {
    let responded_results = async_results.clone();
    let failed_results = async_results.clone();

    steamworks::ServerListCallbacks::new(
        Box::new(move |list, server_index| {
            let result = list
                .lock()
                .expect("Steamworks server-list request mutex was poisoned")
                .get_server_details(server_index);
            responded_results.push(match result {
                Ok(server) => SteamworksMatchmakingServersResult::Ok(
                    SteamworksMatchmakingServersOperation::ServerResponded {
                        request,
                        server_index,
                        server: SteamworksGameServerItem::from_steam(server),
                    },
                ),
                Err(()) => SteamworksMatchmakingServersResult::Err {
                    command: SteamworksMatchmakingServersCommand::GetServerDetails {
                        request,
                        server: server_index,
                    },
                    error: SteamworksMatchmakingServersError::ServerDetailsUnavailable {
                        request,
                        server: server_index,
                    },
                },
            });
        }),
        Box::new(move |_list, server_index| {
            failed_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerFailedToRespond {
                    request,
                    server_index,
                },
            ));
        }),
        Box::new(move |_list, response| {
            async_results.push(SteamworksMatchmakingServersResult::Ok(
                SteamworksMatchmakingServersOperation::ServerListRefreshCompleted {
                    request,
                    response: response.into(),
                },
            ));
        }),
    )
}
