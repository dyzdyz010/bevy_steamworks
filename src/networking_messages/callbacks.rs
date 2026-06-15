use bevy_ecs::{
    message::MessageReader,
    prelude::{Res, ResMut},
};

use crate::{SteamworksClient, SteamworksServer};

use super::{
    messages::{
        SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesOperation,
        SteamworksNetworkingMessagesResult,
    },
    snapshots::snapshot_session_connection_info,
    state::SteamworksNetworkingMessagesState,
    types::SteamworksNetworkingMessagesSessionRequestInfo,
};

pub(super) fn ensure_networking_messages_callbacks(
    client: Option<Res<SteamworksClient>>,
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksNetworkingMessagesState>,
) {
    if state.callbacks_registered() {
        return;
    }

    let Some(networking_messages) = networking_messages(client.as_deref(), server.as_deref())
    else {
        return;
    };

    let request_queue = state.callback_results_queue();
    let auto_accept = state.auto_accept_session_requests_policy();
    networking_messages.session_request_callback(move |request| {
        let remote = request.remote().clone();
        let should_accept = *auto_accept
            .lock()
            .expect("Steamworks Networking Messages policy mutex was poisoned");
        let accepted = should_accept && request.accept();
        let result = SteamworksNetworkingMessagesResult::Ok(
            SteamworksNetworkingMessagesOperation::SessionRequestReceived {
                request: SteamworksNetworkingMessagesSessionRequestInfo { remote, accepted },
            },
        );
        request_queue
            .lock()
            .expect("Steamworks Networking Messages callback queue mutex was poisoned")
            .push(result);
    });

    let failure_queue = state.callback_results_queue();
    networking_messages.session_failed_callback(move |info| {
        let result = SteamworksNetworkingMessagesResult::Ok(
            SteamworksNetworkingMessagesOperation::SessionFailed {
                info: snapshot_session_connection_info(
                    info.state()
                        .unwrap_or(steamworks::networking_types::NetworkingConnectionState::None),
                    Some(&info),
                    None,
                ),
            },
        );
        failure_queue
            .lock()
            .expect("Steamworks Networking Messages callback queue mutex was poisoned")
            .push(result);
    });

    state.mark_callbacks_registered();
    tracing::debug!(
        target: "bevy_steamworks",
        auto_accept_session_requests = state.auto_accept_session_requests(),
        "registered Steamworks Networking Messages callbacks"
    );
}

fn networking_messages(
    client: Option<&SteamworksClient>,
    server: Option<&SteamworksServer>,
) -> Option<steamworks::networking_messages::NetworkingMessages> {
    if let Some(client) = client {
        Some(client.networking_messages())
    } else {
        server.map(|server| server.networking_messages())
    }
}

pub(super) fn apply_networking_messages_policy_commands(
    state: Res<SteamworksNetworkingMessagesState>,
    mut commands: MessageReader<SteamworksNetworkingMessagesCommand>,
) {
    for command in commands.read() {
        if let SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled } =
            command
        {
            state.set_auto_accept_session_requests(*enabled);
        }
    }
}
