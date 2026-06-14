use bevy_ecs::message::{MessageReader, MessageWriter};

use crate::SteamworksEvent;

use super::{
    messages::{SteamworksUserOperation, SteamworksUserResult},
    state::SteamworksUserState,
    types::{
        SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
        SteamworksMicroTxnAuthorizationResponse, SteamworksSteamServerConnectionEvent,
        SteamworksWebApiTicketResponse,
    },
};

pub(super) fn process_user_steam_events(
    state: &mut SteamworksUserState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksUserResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::AuthSessionTicketResponse(event) => {
                SteamworksUserOperation::AuthenticationSessionTicketResponse {
                    response: SteamworksAuthSessionTicketResponse {
                        ticket: event.ticket,
                        result: event.result,
                    },
                }
            }
            SteamworksEvent::TicketForWebApiResponse(event) => {
                SteamworksUserOperation::WebApiAuthenticationTicketReceived {
                    response: snapshot_web_api_ticket_response(event),
                }
            }
            SteamworksEvent::ValidateAuthTicketResponse(event) => {
                SteamworksUserOperation::AuthenticationTicketValidationReceived {
                    validation: SteamworksAuthTicketValidation {
                        steam_id: event.steam_id,
                        owner_steam_id: event.owner_steam_id,
                        response: event.response.clone().map_err(Into::into),
                    },
                }
            }
            SteamworksEvent::SteamServersConnected(_) => {
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Connected,
                }
            }
            SteamworksEvent::SteamServersDisconnected(event) => {
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Disconnected {
                        reason: event.reason,
                    },
                }
            }
            SteamworksEvent::SteamServerConnectFailure(event) => {
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::ConnectFailure {
                        reason: event.reason,
                        still_retrying: event.still_retrying,
                    },
                }
            }
            SteamworksEvent::MicroTxnAuthorizationResponse(event) => {
                SteamworksUserOperation::MicroTxnAuthorizationResponseReceived {
                    response: SteamworksMicroTxnAuthorizationResponse {
                        app_id: event.app_id,
                        order_id: event.order_id,
                        authorized: event.authorized,
                    },
                }
            }
            _ => continue,
        };
        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks user callback"
        );
        results.write(SteamworksUserResult::Ok(operation));
    }
}

fn snapshot_web_api_ticket_response(
    response: &steamworks::TicketForWebApiResponse,
) -> SteamworksWebApiTicketResponse {
    let ticket_len = usize::try_from(response.ticket_len).unwrap_or(0);
    let mut ticket_bytes = response.ticket.clone();
    ticket_bytes.truncate(ticket_len.min(ticket_bytes.len()));
    SteamworksWebApiTicketResponse {
        ticket_handle: response.ticket_handle,
        result: response.result,
        ticket_bytes,
    }
}
