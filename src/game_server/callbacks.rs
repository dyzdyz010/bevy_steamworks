use bevy_ecs::{
    message::{MessageReader, MessageWriter},
    prelude::Res,
};

use crate::{
    user::{
        SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
        SteamworksSteamServerConnectionEvent,
    },
    SteamworksEvent,
};

use super::{
    SteamworksServer, SteamworksServerClientApproval, SteamworksServerClientDenial,
    SteamworksServerClientGroupStatus, SteamworksServerClientKick, SteamworksServerOperation,
    SteamworksServerResult, SteamworksServerState,
};

pub(super) fn process_server_steam_events(
    state: &mut SteamworksServerState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksServerResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::AuthSessionTicketResponse(event) => {
                SteamworksServerOperation::AuthenticationSessionTicketResponse {
                    response: SteamworksAuthSessionTicketResponse {
                        ticket: event.ticket,
                        result: event.result,
                    },
                }
            }
            SteamworksEvent::ValidateAuthTicketResponse(event) => {
                SteamworksServerOperation::AuthenticationTicketValidationReceived {
                    validation: SteamworksAuthTicketValidation {
                        steam_id: event.steam_id,
                        owner_steam_id: event.owner_steam_id,
                        response: event.response.clone().map_err(Into::into),
                    },
                }
            }
            SteamworksEvent::SteamServersConnected(_) => {
                SteamworksServerOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Connected,
                }
            }
            SteamworksEvent::SteamServersDisconnected(event) => {
                SteamworksServerOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Disconnected {
                        reason: event.reason,
                    },
                }
            }
            SteamworksEvent::SteamServerConnectFailure(event) => {
                SteamworksServerOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::ConnectFailure {
                        reason: event.reason,
                        still_retrying: event.still_retrying,
                    },
                }
            }
            SteamworksEvent::GSClientApprove(event) => SteamworksServerOperation::ClientApproved {
                approval: SteamworksServerClientApproval {
                    user: event.user,
                    owner: event.owner,
                },
            },
            SteamworksEvent::GSClientDeny(event) => SteamworksServerOperation::ClientDenied {
                denial: SteamworksServerClientDenial {
                    user: event.user,
                    deny_reason: event.deny_reason,
                    optional_text: event.optional_text.clone(),
                },
            },
            SteamworksEvent::GSClientKick(event) => SteamworksServerOperation::ClientKicked {
                kick: SteamworksServerClientKick {
                    user: event.user,
                    deny_reason: event.deny_reason,
                },
            },
            SteamworksEvent::GSClientGroupStatus(event) => {
                SteamworksServerOperation::ClientGroupStatusReceived {
                    status: SteamworksServerClientGroupStatus {
                        user: event.user,
                        group: event.group,
                        member: event.member,
                        officer: event.officer,
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steam Game Server callback"
        );
        results.write(SteamworksServerResult::Ok(operation));
    }
}

pub(super) fn run_steam_server_callbacks(
    server: Option<Res<SteamworksServer>>,
    mut output: MessageWriter<SteamworksEvent>,
) {
    let Some(server) = server else {
        return;
    };

    server.process_callbacks(|callback| {
        output.write(SteamworksEvent::from(callback));
    });
}
