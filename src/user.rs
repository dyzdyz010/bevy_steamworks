//! High-level Bevy ECS integration for Steam user identity and authentication.
//!
//! This module builds on top of the upstream [`steamworks::User`] API. It keeps
//! common authentication flows in Bevy messages while mirroring relevant
//! low-level callback confirmations from [`crate::SteamworksEvent`] into
//! [`SteamworksUserResult`].

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Bevy plugin for high-level Steam user identity and authentication commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksUserCommand`] and [`SteamworksUserResult`] messages and runs its
/// command processor in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksUserPlugin;

impl SteamworksUserPlugin {
    /// Creates a user plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksUserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksUserState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksUserCommand>()
            .add_message::<SteamworksUserResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessUserCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_user_commands.in_set(SteamworksSystem::ProcessUserCommands),
            );
    }
}

/// Runtime state for [`SteamworksUserPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksUserState {
    last_error: Option<SteamworksUserError>,
    current_user: Option<SteamworksUserInfo>,
    last_steam_id: Option<steamworks::SteamId>,
    last_level: Option<u32>,
    steam_server_connected: Option<bool>,
    active_auth_tickets: Vec<steamworks::AuthTicket>,
    authenticated_users: Vec<steamworks::SteamId>,
    last_auth_session_ticket: Option<SteamworksIssuedAuthSessionTicket>,
    last_web_api_ticket_request: Option<SteamworksWebApiAuthenticationTicketRequest>,
    last_cancelled_auth_ticket: Option<steamworks::AuthTicket>,
    last_started_authentication_session: Option<steamworks::SteamId>,
    last_ended_authentication_session: Option<steamworks::SteamId>,
    last_user_license_for_app: Option<SteamworksUserLicenseForApp>,
    auth_session_ticket_issue_count: u64,
    web_api_ticket_request_count: u64,
    auth_ticket_cancel_count: u64,
    authentication_session_start_count: u64,
    authentication_session_end_count: u64,
    user_license_check_count: u64,
    last_steam_server_connection_event: Option<SteamworksSteamServerConnectionEvent>,
    last_micro_txn_authorization_response: Option<SteamworksMicroTxnAuthorizationResponse>,
    last_auth_ticket_response: Option<SteamworksAuthSessionTicketResponse>,
    last_web_api_ticket_response: Option<SteamworksWebApiTicketResponse>,
    last_auth_ticket_validation: Option<SteamworksAuthTicketValidation>,
}

impl SteamworksUserState {
    /// Returns the most recent synchronous error observed by the user plugin.
    pub fn last_error(&self) -> Option<&SteamworksUserError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent current-user snapshot read through the plugin.
    pub fn current_user(&self) -> Option<&SteamworksUserInfo> {
        self.current_user.as_ref()
    }

    /// Returns the most recent Steam ID read through this plugin.
    pub fn last_steam_id(&self) -> Option<steamworks::SteamId> {
        self.last_steam_id
    }

    /// Returns the most recent Steam user level read through this plugin.
    pub fn last_level(&self) -> Option<u32> {
        self.last_level
    }

    /// Returns the latest known Steam server connection state.
    ///
    /// This is updated by [`SteamworksUserCommand::IsLoggedOn`],
    /// [`SteamworksUserCommand::GetCurrentUserInfo`], and Steam server
    /// connection callbacks.
    pub fn steam_server_connected(&self) -> Option<bool> {
        self.steam_server_connected
    }

    /// Returns authentication ticket handles issued through this command layer.
    ///
    /// Handles are removed after [`SteamworksUserCommand::CancelAuthenticationTicket`]
    /// is processed for the same ticket.
    pub fn active_auth_tickets(&self) -> &[steamworks::AuthTicket] {
        &self.active_auth_tickets
    }

    /// Returns users with sessions started through this command layer.
    ///
    /// Entries are removed after [`SteamworksUserCommand::EndAuthenticationSession`]
    /// is processed for the same user or after Steam reports validation failure
    /// for the same user.
    pub fn authenticated_users(&self) -> &[steamworks::SteamId] {
        &self.authenticated_users
    }

    /// Returns the most recent auth session ticket issued through this command layer.
    pub fn last_auth_session_ticket(&self) -> Option<&SteamworksIssuedAuthSessionTicket> {
        self.last_auth_session_ticket.as_ref()
    }

    /// Returns the most recent Web API auth ticket request submitted through this command layer.
    pub fn last_web_api_ticket_request(
        &self,
    ) -> Option<&SteamworksWebApiAuthenticationTicketRequest> {
        self.last_web_api_ticket_request.as_ref()
    }

    /// Returns the most recent auth ticket cancelled through this command layer.
    pub fn last_cancelled_auth_ticket(&self) -> Option<steamworks::AuthTicket> {
        self.last_cancelled_auth_ticket
    }

    /// Returns the most recent remote authentication session started through this command layer.
    pub fn last_started_authentication_session(&self) -> Option<steamworks::SteamId> {
        self.last_started_authentication_session
    }

    /// Returns the most recent remote authentication session ended through this command layer.
    pub fn last_ended_authentication_session(&self) -> Option<steamworks::SteamId> {
        self.last_ended_authentication_session
    }

    /// Returns the most recent app-license check submitted through this command layer.
    pub fn last_user_license_for_app(&self) -> Option<&SteamworksUserLicenseForApp> {
        self.last_user_license_for_app.as_ref()
    }

    /// Returns how many auth session tickets this plugin issued.
    pub fn auth_session_ticket_issue_count(&self) -> u64 {
        self.auth_session_ticket_issue_count
    }

    /// Returns how many Web API auth ticket requests this plugin submitted.
    pub fn web_api_ticket_request_count(&self) -> u64 {
        self.web_api_ticket_request_count
    }

    /// Returns how many auth tickets this plugin cancelled.
    pub fn auth_ticket_cancel_count(&self) -> u64 {
        self.auth_ticket_cancel_count
    }

    /// Returns how many remote authentication sessions this plugin started.
    pub fn authentication_session_start_count(&self) -> u64 {
        self.authentication_session_start_count
    }

    /// Returns how many remote authentication sessions this plugin ended.
    pub fn authentication_session_end_count(&self) -> u64 {
        self.authentication_session_end_count
    }

    /// Returns how many app-license checks this plugin performed.
    pub fn user_license_check_count(&self) -> u64 {
        self.user_license_check_count
    }

    /// Returns the most recent Steam server connection callback snapshot.
    pub fn last_steam_server_connection_event(
        &self,
    ) -> Option<&SteamworksSteamServerConnectionEvent> {
        self.last_steam_server_connection_event.as_ref()
    }

    /// Returns the most recent microtransaction authorization callback snapshot.
    pub fn last_micro_txn_authorization_response(
        &self,
    ) -> Option<&SteamworksMicroTxnAuthorizationResponse> {
        self.last_micro_txn_authorization_response.as_ref()
    }

    /// Returns the most recent auth session ticket response callback snapshot.
    pub fn last_auth_ticket_response(&self) -> Option<&SteamworksAuthSessionTicketResponse> {
        self.last_auth_ticket_response.as_ref()
    }

    /// Returns the most recent Web API ticket response callback snapshot.
    pub fn last_web_api_ticket_response(&self) -> Option<&SteamworksWebApiTicketResponse> {
        self.last_web_api_ticket_response.as_ref()
    }

    /// Returns the most recent auth ticket validation callback snapshot.
    pub fn last_auth_ticket_validation(&self) -> Option<&SteamworksAuthTicketValidation> {
        self.last_auth_ticket_validation.as_ref()
    }

    fn record_error(&mut self, error: SteamworksUserError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksUserOperation) {
        match operation {
            SteamworksUserOperation::CurrentUserInfoRead { info } => {
                self.current_user = Some(info.clone());
                self.last_steam_id = Some(info.steam_id);
                self.last_level = Some(info.level);
                self.steam_server_connected = Some(info.logged_on);
            }
            SteamworksUserOperation::SteamIdRead { steam_id } => {
                self.last_steam_id = Some(*steam_id);
                if let Some(info) = &mut self.current_user {
                    info.steam_id = *steam_id;
                }
            }
            SteamworksUserOperation::LevelRead { level } => {
                self.last_level = Some(*level);
                if let Some(info) = &mut self.current_user {
                    info.level = *level;
                }
            }
            SteamworksUserOperation::LoggedOnRead { logged_on } => {
                self.steam_server_connected = Some(*logged_on);
                if let Some(info) = &mut self.current_user {
                    info.logged_on = *logged_on;
                }
            }
            SteamworksUserOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id,
            } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                self.last_auth_session_ticket = Some(SteamworksIssuedAuthSessionTicket {
                    ticket: *ticket,
                    ticket_bytes: ticket_bytes.clone(),
                    steam_id: *steam_id,
                });
                self.auth_session_ticket_issue_count =
                    self.auth_session_ticket_issue_count.saturating_add(1);
            }
            SteamworksUserOperation::WebApiAuthenticationTicketRequested { ticket, identity } => {
                if !self.active_auth_tickets.contains(ticket) {
                    self.active_auth_tickets.push(*ticket);
                }
                self.last_web_api_ticket_request =
                    Some(SteamworksWebApiAuthenticationTicketRequest {
                        ticket: *ticket,
                        identity: identity.clone(),
                    });
                self.web_api_ticket_request_count =
                    self.web_api_ticket_request_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationTicketCancelled { ticket } => {
                self.active_auth_tickets.retain(|known| known != ticket);
                self.last_cancelled_auth_ticket = Some(*ticket);
                self.auth_ticket_cancel_count = self.auth_ticket_cancel_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionStarted { user } => {
                if !self.authenticated_users.contains(user) {
                    self.authenticated_users.push(*user);
                }
                self.last_started_authentication_session = Some(*user);
                self.authentication_session_start_count =
                    self.authentication_session_start_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionEnded { user } => {
                self.authenticated_users.retain(|known| known != user);
                self.last_ended_authentication_session = Some(*user);
                self.authentication_session_end_count =
                    self.authentication_session_end_count.saturating_add(1);
            }
            SteamworksUserOperation::UserLicenseForAppRead {
                user,
                app_id,
                license,
            } => {
                self.last_user_license_for_app = Some(SteamworksUserLicenseForApp {
                    user: *user,
                    app_id: *app_id,
                    license: license.clone(),
                });
                self.user_license_check_count = self.user_license_check_count.saturating_add(1);
            }
            SteamworksUserOperation::AuthenticationSessionTicketResponse { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket);
                }
                self.last_auth_ticket_response = Some(response.clone());
            }
            SteamworksUserOperation::WebApiAuthenticationTicketReceived { response } => {
                if response.result.is_err() {
                    self.active_auth_tickets
                        .retain(|known| *known != response.ticket_handle);
                }
                self.last_web_api_ticket_response = Some(response.clone());
            }
            SteamworksUserOperation::AuthenticationTicketValidationReceived { validation } => {
                if validation.response.is_err() {
                    self.authenticated_users
                        .retain(|known| *known != validation.steam_id);
                }
                self.last_auth_ticket_validation = Some(validation.clone());
            }
            SteamworksUserOperation::SteamServerConnectionEventReceived { event } => {
                let connected = matches!(event, SteamworksSteamServerConnectionEvent::Connected);
                self.steam_server_connected = Some(connected);
                if let Some(info) = &mut self.current_user {
                    info.logged_on = connected;
                }
                self.last_steam_server_connection_event = Some(event.clone());
            }
            SteamworksUserOperation::MicroTxnAuthorizationResponseReceived { response } => {
                self.last_micro_txn_authorization_response = Some(response.clone());
            }
        }
    }
}

/// Snapshot of common information about the current Steam user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUserInfo {
    /// Current Steam user ID.
    pub steam_id: steamworks::SteamId,
    /// Current Steam user level.
    pub level: u32,
    /// Whether the local Steam client is connected to Steam servers.
    pub logged_on: bool,
}

/// Auth session ticket issued through this command layer.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksIssuedAuthSessionTicket {
    /// Ticket handle that should be cancelled when no longer needed.
    pub ticket: steamworks::AuthTicket,
    /// Raw ticket bytes returned by Steam.
    ///
    /// Treat this as credential material; avoid logging it or storing it longer than needed.
    pub ticket_bytes: Vec<u8>,
    /// Steam ID used as the network identity for the verifier.
    pub steam_id: steamworks::SteamId,
}

impl std::fmt::Debug for SteamworksIssuedAuthSessionTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksIssuedAuthSessionTicket")
            .field("ticket", &self.ticket)
            .field("ticket_bytes_len", &self.ticket_bytes.len())
            .field("steam_id", &self.steam_id)
            .finish()
    }
}

/// Web API auth ticket request submitted through this command layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksWebApiAuthenticationTicketRequest {
    /// Ticket handle that should be cancelled when no longer needed.
    pub ticket: steamworks::AuthTicket,
    /// Identity string submitted for the consuming service.
    pub identity: String,
}

/// App-license check result for an authenticated user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksUserLicenseForApp {
    /// Steam user that was checked.
    pub user: steamworks::SteamId,
    /// Steam app ID that was checked.
    pub app_id: steamworks::AppId,
    /// License state reported by Steam.
    pub license: steamworks::UserHasLicense,
}

/// Auth session ticket creation callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksAuthSessionTicketResponse {
    /// Ticket handle reported by Steam.
    pub ticket: steamworks::AuthTicket,
    /// Steam result for ticket creation.
    pub result: Result<(), steamworks::SteamError>,
}

/// Web API auth ticket callback snapshot.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksWebApiTicketResponse {
    /// Ticket handle reported by Steam.
    pub ticket_handle: steamworks::AuthTicket,
    /// Steam result for ticket creation.
    pub result: Result<(), steamworks::SteamError>,
    /// Ticket bytes returned by Steam, truncated to Steam's reported length.
    ///
    /// Treat this as credential material; avoid logging it or storing it longer than needed.
    pub ticket_bytes: Vec<u8>,
}

impl std::fmt::Debug for SteamworksWebApiTicketResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksWebApiTicketResponse")
            .field("ticket_handle", &self.ticket_handle)
            .field("result", &self.result)
            .field("ticket_bytes_len", &self.ticket_bytes.len())
            .finish()
    }
}

/// Auth ticket validation callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksAuthTicketValidation {
    /// Steam user whose ticket was validated.
    pub steam_id: steamworks::SteamId,
    /// Owner of the game license used by the ticket.
    pub owner_steam_id: steamworks::SteamId,
    /// Validation result.
    pub response: Result<(), SteamworksAuthSessionValidateError>,
}

/// Steam server connection callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksSteamServerConnectionEvent {
    /// The local Steam client connected to Steam servers.
    Connected,
    /// The local Steam client disconnected from Steam servers.
    Disconnected {
        /// Reason reported by Steam.
        reason: steamworks::SteamError,
    },
    /// The local Steam client failed to connect to Steam servers.
    ConnectFailure {
        /// Reason reported by Steam.
        reason: steamworks::SteamError,
        /// Whether Steam is still retrying the connection.
        still_retrying: bool,
    },
}

/// Microtransaction authorization callback snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksMicroTxnAuthorizationResponse {
    /// App ID reported by Steam.
    pub app_id: steamworks::AppId,
    /// Order ID supplied by the Steam microtransaction flow.
    pub order_id: u64,
    /// Whether the user authorized the transaction.
    pub authorized: bool,
}

/// A high-level command for Steam user identity and authentication workflows.
#[derive(Clone, Message, PartialEq, Eq)]
pub enum SteamworksUserCommand {
    /// Read a snapshot of common current-user information.
    GetCurrentUserInfo,
    /// Read the current Steam user ID.
    GetSteamId,
    /// Read the current Steam user level.
    GetLevel,
    /// Read whether the local Steam client is connected to Steam servers.
    IsLoggedOn,
    /// Request an authentication session ticket for an entity identified by Steam ID.
    ///
    /// Final ticket creation confirmation arrives later through both
    /// [`crate::SteamworksEvent::AuthSessionTicketResponse`] and
    /// [`SteamworksUserOperation::AuthenticationSessionTicketResponse`].
    GetAuthenticationSessionTicket {
        /// Steam ID for the entity that will verify the ticket.
        steam_id: steamworks::SteamId,
    },
    /// Request an authentication ticket for Steam Web API verification.
    ///
    /// The ticket bytes arrive later through both
    /// [`crate::SteamworksEvent::TicketForWebApiResponse`] and
    /// [`SteamworksUserOperation::WebApiAuthenticationTicketReceived`].
    GetAuthenticationSessionTicketForWebApi {
        /// Identity string for the service that will consume the ticket.
        identity: String,
    },
    /// Cancel a locally issued authentication ticket.
    CancelAuthenticationTicket {
        /// Ticket handle to cancel.
        ticket: steamworks::AuthTicket,
    },
    /// Begin validating a ticket received from another Steam user.
    BeginAuthenticationSession {
        /// Steam user that provided the ticket.
        user: steamworks::SteamId,
        /// Raw authentication ticket bytes.
        ticket: Vec<u8>,
    },
    /// End a session started with [`SteamworksUserCommand::BeginAuthenticationSession`].
    EndAuthenticationSession {
        /// Steam user whose authentication session should end.
        user: steamworks::SteamId,
    },
    /// Check whether an authenticated user owns a specific app or DLC.
    UserHasLicenseForApp {
        /// Steam user to check.
        user: steamworks::SteamId,
        /// Steam app ID to check.
        app_id: steamworks::AppId,
    },
}

impl std::fmt::Debug for SteamworksUserCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetCurrentUserInfo => f.write_str("GetCurrentUserInfo"),
            Self::GetSteamId => f.write_str("GetSteamId"),
            Self::GetLevel => f.write_str("GetLevel"),
            Self::IsLoggedOn => f.write_str("IsLoggedOn"),
            Self::GetAuthenticationSessionTicket { steam_id } => f
                .debug_struct("GetAuthenticationSessionTicket")
                .field("steam_id", steam_id)
                .finish(),
            Self::GetAuthenticationSessionTicketForWebApi { identity } => f
                .debug_struct("GetAuthenticationSessionTicketForWebApi")
                .field("identity", identity)
                .finish(),
            Self::CancelAuthenticationTicket { ticket } => f
                .debug_struct("CancelAuthenticationTicket")
                .field("ticket", ticket)
                .finish(),
            Self::BeginAuthenticationSession { user, ticket } => f
                .debug_struct("BeginAuthenticationSession")
                .field("user", user)
                .field("ticket_len", &ticket.len())
                .finish(),
            Self::EndAuthenticationSession { user } => f
                .debug_struct("EndAuthenticationSession")
                .field("user", user)
                .finish(),
            Self::UserHasLicenseForApp { user, app_id } => f
                .debug_struct("UserHasLicenseForApp")
                .field("user", user)
                .field("app_id", app_id)
                .finish(),
        }
    }
}

impl SteamworksUserCommand {
    /// Creates a [`SteamworksUserCommand::GetAuthenticationSessionTicket`] command.
    pub fn get_authentication_session_ticket(steam_id: steamworks::SteamId) -> Self {
        Self::GetAuthenticationSessionTicket { steam_id }
    }

    /// Creates a [`SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi`] command.
    pub fn get_authentication_session_ticket_for_web_api(identity: impl Into<String>) -> Self {
        Self::GetAuthenticationSessionTicketForWebApi {
            identity: identity.into(),
        }
    }

    /// Creates a [`SteamworksUserCommand::CancelAuthenticationTicket`] command.
    pub fn cancel_authentication_ticket(ticket: steamworks::AuthTicket) -> Self {
        Self::CancelAuthenticationTicket { ticket }
    }

    /// Creates a [`SteamworksUserCommand::BeginAuthenticationSession`] command.
    pub fn begin_authentication_session(
        user: steamworks::SteamId,
        ticket: impl Into<Vec<u8>>,
    ) -> Self {
        Self::BeginAuthenticationSession {
            user,
            ticket: ticket.into(),
        }
    }

    /// Creates a [`SteamworksUserCommand::EndAuthenticationSession`] command.
    pub fn end_authentication_session(user: steamworks::SteamId) -> Self {
        Self::EndAuthenticationSession { user }
    }

    /// Creates a [`SteamworksUserCommand::UserHasLicenseForApp`] command.
    pub fn user_has_license_for_app(
        user: steamworks::SteamId,
        app_id: impl Into<steamworks::AppId>,
    ) -> Self {
        Self::UserHasLicenseForApp {
            user,
            app_id: app_id.into(),
        }
    }
}

/// A successfully submitted Steam user operation or synchronous read.
#[derive(Clone, PartialEq, Eq)]
pub enum SteamworksUserOperation {
    /// Common current-user information was read.
    CurrentUserInfoRead {
        /// Current-user snapshot.
        info: SteamworksUserInfo,
    },
    /// Current Steam user ID was read.
    SteamIdRead {
        /// Current Steam user ID.
        steam_id: steamworks::SteamId,
    },
    /// Current Steam user level was read.
    LevelRead {
        /// Current Steam user level.
        level: u32,
    },
    /// Steam server connection state was read.
    LoggedOnRead {
        /// Whether the local Steam client is connected to Steam servers.
        logged_on: bool,
    },
    /// Authentication session ticket bytes were issued.
    ///
    /// Final creation confirmation arrives later through both
    /// [`crate::SteamworksEvent::AuthSessionTicketResponse`] and
    /// [`SteamworksUserOperation::AuthenticationSessionTicketResponse`].
    AuthenticationSessionTicketIssued {
        /// Ticket handle that should be cancelled when no longer needed.
        ticket: steamworks::AuthTicket,
        /// Raw ticket bytes to send to the verifying entity.
        ticket_bytes: Vec<u8>,
        /// Steam ID used as the network identity for the verifier.
        steam_id: steamworks::SteamId,
    },
    /// A Steam Web API authentication ticket request was submitted.
    ///
    /// Ticket bytes arrive later through both
    /// [`crate::SteamworksEvent::TicketForWebApiResponse`] and
    /// [`SteamworksUserOperation::WebApiAuthenticationTicketReceived`].
    WebApiAuthenticationTicketRequested {
        /// Ticket handle that should be cancelled when no longer needed.
        ticket: steamworks::AuthTicket,
        /// Identity string submitted for the consuming service.
        identity: String,
    },
    /// A locally issued authentication ticket was cancelled.
    AuthenticationTicketCancelled {
        /// Ticket handle that was cancelled.
        ticket: steamworks::AuthTicket,
    },
    /// Authentication began for a remote user ticket.
    ///
    /// Later validation callbacks arrive through both
    /// [`crate::SteamworksEvent::ValidateAuthTicketResponse`] and
    /// [`SteamworksUserOperation::AuthenticationTicketValidationReceived`].
    AuthenticationSessionStarted {
        /// Steam user whose ticket was accepted for validation.
        user: steamworks::SteamId,
    },
    /// Authentication ended for a remote user.
    AuthenticationSessionEnded {
        /// Steam user whose authentication session ended.
        user: steamworks::SteamId,
    },
    /// App license state was read for an authenticated user.
    UserLicenseForAppRead {
        /// Steam user that was checked.
        user: steamworks::SteamId,
        /// Steam app ID that was checked.
        app_id: steamworks::AppId,
        /// License state reported by Steam.
        license: steamworks::UserHasLicense,
    },
    /// Auth session ticket creation callback was observed.
    AuthenticationSessionTicketResponse {
        /// Callback snapshot.
        response: SteamworksAuthSessionTicketResponse,
    },
    /// Web API auth ticket creation callback was observed.
    WebApiAuthenticationTicketReceived {
        /// Callback snapshot.
        response: SteamworksWebApiTicketResponse,
    },
    /// Auth ticket validation callback was observed.
    AuthenticationTicketValidationReceived {
        /// Callback snapshot.
        validation: SteamworksAuthTicketValidation,
    },
    /// Steam server connection state callback was observed.
    SteamServerConnectionEventReceived {
        /// Callback snapshot.
        event: SteamworksSteamServerConnectionEvent,
    },
    /// Microtransaction authorization callback was observed.
    MicroTxnAuthorizationResponseReceived {
        /// Callback snapshot.
        response: SteamworksMicroTxnAuthorizationResponse,
    },
}

impl std::fmt::Debug for SteamworksUserOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurrentUserInfoRead { info } => f
                .debug_struct("CurrentUserInfoRead")
                .field("info", info)
                .finish(),
            Self::SteamIdRead { steam_id } => f
                .debug_struct("SteamIdRead")
                .field("steam_id", steam_id)
                .finish(),
            Self::LevelRead { level } => f.debug_struct("LevelRead").field("level", level).finish(),
            Self::LoggedOnRead { logged_on } => f
                .debug_struct("LoggedOnRead")
                .field("logged_on", logged_on)
                .finish(),
            Self::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id,
            } => f
                .debug_struct("AuthenticationSessionTicketIssued")
                .field("ticket", ticket)
                .field("ticket_bytes_len", &ticket_bytes.len())
                .field("steam_id", steam_id)
                .finish(),
            Self::WebApiAuthenticationTicketRequested { ticket, identity } => f
                .debug_struct("WebApiAuthenticationTicketRequested")
                .field("ticket", ticket)
                .field("identity", identity)
                .finish(),
            Self::AuthenticationTicketCancelled { ticket } => f
                .debug_struct("AuthenticationTicketCancelled")
                .field("ticket", ticket)
                .finish(),
            Self::AuthenticationSessionStarted { user } => f
                .debug_struct("AuthenticationSessionStarted")
                .field("user", user)
                .finish(),
            Self::AuthenticationSessionEnded { user } => f
                .debug_struct("AuthenticationSessionEnded")
                .field("user", user)
                .finish(),
            Self::UserLicenseForAppRead {
                user,
                app_id,
                license,
            } => f
                .debug_struct("UserLicenseForAppRead")
                .field("user", user)
                .field("app_id", app_id)
                .field("license", license)
                .finish(),
            Self::AuthenticationSessionTicketResponse { response } => f
                .debug_struct("AuthenticationSessionTicketResponse")
                .field("response", response)
                .finish(),
            Self::WebApiAuthenticationTicketReceived { response } => f
                .debug_struct("WebApiAuthenticationTicketReceived")
                .field("response", response)
                .finish(),
            Self::AuthenticationTicketValidationReceived { validation } => f
                .debug_struct("AuthenticationTicketValidationReceived")
                .field("validation", validation)
                .finish(),
            Self::SteamServerConnectionEventReceived { event } => f
                .debug_struct("SteamServerConnectionEventReceived")
                .field("event", event)
                .finish(),
            Self::MicroTxnAuthorizationResponseReceived { response } => f
                .debug_struct("MicroTxnAuthorizationResponseReceived")
                .field("response", response)
                .finish(),
        }
    }
}

/// Result message emitted by [`SteamworksUserPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksUserResult {
    /// The command was submitted to Steamworks or a value was read.
    Ok(SteamworksUserOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksUserCommand,
        /// Failure reason.
        error: SteamworksUserError,
    },
}

/// Synchronous errors from [`SteamworksUserPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksUserError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steamworks user command field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// A remote authentication session was requested with no ticket bytes.
    #[error("Steamworks user command requires a non-empty authentication ticket")]
    EmptyTicket,
    /// The upstream Steamworks API rejected an authentication session.
    #[error("Steamworks authentication session failed: {source}")]
    AuthSession {
        /// Authentication session failure reason.
        #[source]
        source: SteamworksAuthSessionError,
    },
}

impl SteamworksUserError {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn auth_session(source: steamworks::AuthSessionError) -> Self {
        Self::AuthSession {
            source: source.into(),
        }
    }
}

/// Cloneable, comparable mirror of upstream [`steamworks::AuthSessionError`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum SteamworksAuthSessionError {
    /// The ticket is invalid.
    #[error("invalid ticket")]
    InvalidTicket,
    /// A ticket has already been submitted for this Steam ID.
    #[error("duplicate ticket request")]
    DuplicateRequest,
    /// The ticket is from an incompatible interface version.
    #[error("incompatible interface version")]
    InvalidVersion,
    /// The ticket is not for this game.
    #[error("incorrect game for ticket")]
    GameMismatch,
    /// The ticket has expired.
    #[error("ticket has expired")]
    ExpiredTicket,
}

impl From<steamworks::AuthSessionError> for SteamworksAuthSessionError {
    fn from(error: steamworks::AuthSessionError) -> Self {
        match error {
            steamworks::AuthSessionError::InvalidTicket => Self::InvalidTicket,
            steamworks::AuthSessionError::DuplicateRequest => Self::DuplicateRequest,
            steamworks::AuthSessionError::InvalidVersion => Self::InvalidVersion,
            steamworks::AuthSessionError::GameMismatch => Self::GameMismatch,
            steamworks::AuthSessionError::ExpiredTicket => Self::ExpiredTicket,
        }
    }
}

/// Cloneable, comparable mirror of upstream [`steamworks::AuthSessionValidateError`].
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum SteamworksAuthSessionValidateError {
    /// The user is not connected to Steam.
    #[error("user not connected to Steam")]
    UserNotConnectedToSteam,
    /// The user has no license or the license expired.
    #[error("no license or expired license")]
    NoLicenseOrExpired,
    /// The user is VAC banned.
    #[error("VAC banned")]
    VacBanned,
    /// The user is logged in elsewhere.
    #[error("logged in elsewhere")]
    LoggedInElseWhere,
    /// VAC check timed out.
    #[error("VAC check timed out")]
    VacCheckTimedOut,
    /// The auth ticket was cancelled.
    #[error("auth ticket cancelled")]
    AuthTicketCancelled,
    /// The auth ticket was already used.
    #[error("auth ticket already used")]
    AuthTicketInvalidAlreadyUsed,
    /// The auth ticket is invalid.
    #[error("auth ticket invalid")]
    AuthTicketInvalid,
    /// Publisher issued a ban.
    #[error("publisher issued ban")]
    PublisherIssuedBan,
    /// The ticket network identity did not match.
    #[error("auth ticket network identity failure")]
    AuthTicketNetworkIdentityFailure,
}

impl From<steamworks::AuthSessionValidateError> for SteamworksAuthSessionValidateError {
    fn from(error: steamworks::AuthSessionValidateError) -> Self {
        match error {
            steamworks::AuthSessionValidateError::UserNotConnectedToSteam => {
                Self::UserNotConnectedToSteam
            }
            steamworks::AuthSessionValidateError::NoLicenseOrExpired => Self::NoLicenseOrExpired,
            steamworks::AuthSessionValidateError::VACBanned => Self::VacBanned,
            steamworks::AuthSessionValidateError::LoggedInElseWhere => Self::LoggedInElseWhere,
            steamworks::AuthSessionValidateError::VACCheckTimedOut => Self::VacCheckTimedOut,
            steamworks::AuthSessionValidateError::AuthTicketCancelled => Self::AuthTicketCancelled,
            steamworks::AuthSessionValidateError::AuthTicketInvalidAlreadyUsed => {
                Self::AuthTicketInvalidAlreadyUsed
            }
            steamworks::AuthSessionValidateError::AuthTicketInvalid => Self::AuthTicketInvalid,
            steamworks::AuthSessionValidateError::PublisherIssuedBan => Self::PublisherIssuedBan,
            steamworks::AuthSessionValidateError::AuthTicketNetworkIdentityFailure => {
                Self::AuthTicketNetworkIdentityFailure
            }
        }
    }
}

fn process_user_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksUserState>,
    mut commands: ResMut<Messages<SteamworksUserCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksUserResult>,
) {
    process_user_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksUserError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks user command failed"
            );
            results.write(SteamworksUserResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_user_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks user command"
                );
                results.write(SteamworksUserResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks user command failed"
                );
                results.write(SteamworksUserResult::Err { command, error });
            }
        }
    }
}

fn process_user_steam_events(
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

fn handle_user_command(
    client: &SteamworksClient,
    command: &SteamworksUserCommand,
) -> Result<SteamworksUserOperation, SteamworksUserError> {
    validate_command(command)?;

    let user = client.user();
    Ok(match command {
        SteamworksUserCommand::GetCurrentUserInfo => SteamworksUserOperation::CurrentUserInfoRead {
            info: snapshot_current_user(client),
        },
        SteamworksUserCommand::GetSteamId => SteamworksUserOperation::SteamIdRead {
            steam_id: user.steam_id(),
        },
        SteamworksUserCommand::GetLevel => SteamworksUserOperation::LevelRead {
            level: user.level(),
        },
        SteamworksUserCommand::IsLoggedOn => SteamworksUserOperation::LoggedOnRead {
            logged_on: user.logged_on(),
        },
        SteamworksUserCommand::GetAuthenticationSessionTicket { steam_id } => {
            let (ticket, ticket_bytes) =
                user.authentication_session_ticket_with_steam_id(*steam_id);
            SteamworksUserOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id: *steam_id,
            }
        }
        SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi { identity } => {
            let ticket = user.authentication_session_ticket_for_webapi(identity);
            SteamworksUserOperation::WebApiAuthenticationTicketRequested {
                ticket,
                identity: identity.clone(),
            }
        }
        SteamworksUserCommand::CancelAuthenticationTicket { ticket } => {
            user.cancel_authentication_ticket(*ticket);
            SteamworksUserOperation::AuthenticationTicketCancelled { ticket: *ticket }
        }
        SteamworksUserCommand::BeginAuthenticationSession {
            user: steam_id,
            ticket,
        } => {
            user.begin_authentication_session(*steam_id, ticket)
                .map_err(SteamworksUserError::auth_session)?;
            SteamworksUserOperation::AuthenticationSessionStarted { user: *steam_id }
        }
        SteamworksUserCommand::EndAuthenticationSession { user: steam_id } => {
            user.end_authentication_session(*steam_id);
            SteamworksUserOperation::AuthenticationSessionEnded { user: *steam_id }
        }
        SteamworksUserCommand::UserHasLicenseForApp {
            user: steam_id,
            app_id,
        } => SteamworksUserOperation::UserLicenseForAppRead {
            user: *steam_id,
            app_id: *app_id,
            license: user.user_has_license_for_app(*steam_id, *app_id),
        },
    })
}

fn snapshot_current_user(client: &SteamworksClient) -> SteamworksUserInfo {
    let user = client.user();
    SteamworksUserInfo {
        steam_id: user.steam_id(),
        level: user.level(),
        logged_on: user.logged_on(),
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

fn validate_command(command: &SteamworksUserCommand) -> Result<(), SteamworksUserError> {
    match command {
        SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi { identity } => {
            validate_steam_string("identity", identity)
        }
        SteamworksUserCommand::BeginAuthenticationSession { ticket, .. } => {
            if ticket.is_empty() {
                Err(SteamworksUserError::EmptyTicket)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksUserError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksUserError::invalid_string(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn user_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksUserPlugin::new());

        assert!(app.world().contains_resource::<SteamworksUserState>());
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUserCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksUserResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksUserPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksUserCommand>>()
            .write(SteamworksUserCommand::GetCurrentUserInfo);

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksUserResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksUserResult::Err {
                command: SteamworksUserCommand::GetCurrentUserInfo,
                error: SteamworksUserError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksUserState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksUserError::ClientUnavailable)
        );
    }

    #[test]
    fn validation_rejects_interior_nul_for_web_api_identity() {
        let command =
            SteamworksUserCommand::get_authentication_session_ticket_for_web_api("web\0bad");

        assert_eq!(
            validate_command(&command),
            Err(SteamworksUserError::InvalidString { field: "identity" })
        );
    }

    #[test]
    fn validation_rejects_empty_auth_ticket() {
        let command = SteamworksUserCommand::begin_authentication_session(
            steamworks::SteamId::from_raw(1),
            Vec::new(),
        );

        assert_eq!(
            validate_command(&command),
            Err(SteamworksUserError::EmptyTicket)
        );
    }

    #[test]
    fn command_debug_redacts_authentication_ticket_bytes() {
        let command = SteamworksUserCommand::begin_authentication_session(
            steamworks::SteamId::from_raw(1),
            vec![1, 2, 3, 4],
        );
        let debug = format!("{command:?}");

        assert!(debug.contains("ticket_len: 4"));
        assert!(!debug.contains("[1, 2, 3, 4]"));
    }

    #[test]
    fn auth_session_errors_are_cloneable_and_comparable() {
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::InvalidTicket),
            SteamworksAuthSessionError::InvalidTicket
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::DuplicateRequest),
            SteamworksAuthSessionError::DuplicateRequest
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::InvalidVersion),
            SteamworksAuthSessionError::InvalidVersion
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::GameMismatch),
            SteamworksAuthSessionError::GameMismatch
        );
        assert_eq!(
            SteamworksAuthSessionError::from(steamworks::AuthSessionError::ExpiredTicket),
            SteamworksAuthSessionError::ExpiredTicket
        );
    }

    #[test]
    fn auth_session_validate_errors_are_cloneable_and_comparable() {
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::UserNotConnectedToSteam,
            ),
            SteamworksAuthSessionValidateError::UserNotConnectedToSteam
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::NoLicenseOrExpired,
            ),
            SteamworksAuthSessionValidateError::NoLicenseOrExpired
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::VACBanned,
            ),
            SteamworksAuthSessionValidateError::VacBanned
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::LoggedInElseWhere,
            ),
            SteamworksAuthSessionValidateError::LoggedInElseWhere
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::VACCheckTimedOut,
            ),
            SteamworksAuthSessionValidateError::VacCheckTimedOut
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::AuthTicketCancelled,
            ),
            SteamworksAuthSessionValidateError::AuthTicketCancelled
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::AuthTicketInvalidAlreadyUsed,
            ),
            SteamworksAuthSessionValidateError::AuthTicketInvalidAlreadyUsed
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::AuthTicketInvalid,
            ),
            SteamworksAuthSessionValidateError::AuthTicketInvalid
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::PublisherIssuedBan,
            ),
            SteamworksAuthSessionValidateError::PublisherIssuedBan
        );
        assert_eq!(
            SteamworksAuthSessionValidateError::from(
                steamworks::AuthSessionValidateError::AuthTicketNetworkIdentityFailure,
            ),
            SteamworksAuthSessionValidateError::AuthTicketNetworkIdentityFailure
        );
    }

    #[test]
    fn auth_validation_callbacks_are_bridged_without_client() {
        let mut app = App::new();
        let user = steamworks::SteamId::from_raw(1);
        let owner = steamworks::SteamId::from_raw(2);

        app.add_plugins(SteamworksUserPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ValidateAuthTicketResponse(
                steamworks::ValidateAuthTicketResponse {
                    steam_id: user,
                    owner_steam_id: owner,
                    response: Ok(()),
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::ValidateAuthTicketResponse(
                steamworks::ValidateAuthTicketResponse {
                    steam_id: user,
                    owner_steam_id: owner,
                    response: Err(steamworks::AuthSessionValidateError::VACBanned),
                },
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksUserResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        assert_eq!(
            drained,
            vec![
                SteamworksUserResult::Ok(
                    SteamworksUserOperation::AuthenticationTicketValidationReceived {
                        validation: SteamworksAuthTicketValidation {
                            steam_id: user,
                            owner_steam_id: owner,
                            response: Ok(()),
                        },
                    },
                ),
                SteamworksUserResult::Ok(
                    SteamworksUserOperation::AuthenticationTicketValidationReceived {
                        validation: SteamworksAuthTicketValidation {
                            steam_id: user,
                            owner_steam_id: owner,
                            response: Err(SteamworksAuthSessionValidateError::VacBanned),
                        },
                    },
                ),
            ]
        );

        let state = app.world().resource::<SteamworksUserState>();
        assert_eq!(
            state.last_auth_ticket_validation(),
            Some(&SteamworksAuthTicketValidation {
                steam_id: user,
                owner_steam_id: owner,
                response: Err(SteamworksAuthSessionValidateError::VacBanned),
            })
        );
        assert!(state.authenticated_users().is_empty());
        assert_eq!(state.last_error(), None);
    }

    #[test]
    fn connection_and_microtxn_callbacks_are_bridged_without_client() {
        let mut app = App::new();
        let app_id = steamworks::AppId(480);

        app.add_plugins(SteamworksUserPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::SteamServersConnected(
                steamworks::SteamServersConnected,
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::SteamServersDisconnected(
                steamworks::SteamServersDisconnected {
                    reason: steamworks::SteamError::NoConnection,
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::SteamServerConnectFailure(
                steamworks::SteamServerConnectFailure {
                    reason: steamworks::SteamError::Timeout,
                    still_retrying: true,
                },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::MicroTxnAuthorizationResponse(
                steamworks::MicroTxnAuthorizationResponse {
                    app_id,
                    order_id: 99,
                    authorized: true,
                },
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksUserResult>>();
        let drained = results.drain().collect::<Vec<_>>();
        let disconnected = SteamworksSteamServerConnectionEvent::Disconnected {
            reason: steamworks::SteamError::NoConnection,
        };
        let failed = SteamworksSteamServerConnectionEvent::ConnectFailure {
            reason: steamworks::SteamError::Timeout,
            still_retrying: true,
        };
        let micro_txn = SteamworksMicroTxnAuthorizationResponse {
            app_id,
            order_id: 99,
            authorized: true,
        };

        assert_eq!(
            drained,
            vec![
                SteamworksUserResult::Ok(
                    SteamworksUserOperation::SteamServerConnectionEventReceived {
                        event: SteamworksSteamServerConnectionEvent::Connected,
                    },
                ),
                SteamworksUserResult::Ok(
                    SteamworksUserOperation::SteamServerConnectionEventReceived {
                        event: disconnected,
                    },
                ),
                SteamworksUserResult::Ok(
                    SteamworksUserOperation::SteamServerConnectionEventReceived {
                        event: failed.clone(),
                    },
                ),
                SteamworksUserResult::Ok(
                    SteamworksUserOperation::MicroTxnAuthorizationResponseReceived {
                        response: micro_txn.clone(),
                    },
                ),
            ]
        );

        let state = app.world().resource::<SteamworksUserState>();
        assert_eq!(state.steam_server_connected(), Some(false));
        assert_eq!(state.last_steam_server_connection_event(), Some(&failed));
        assert_eq!(
            state.last_micro_txn_authorization_response(),
            Some(&micro_txn)
        );
        assert_eq!(state.last_error(), None);
    }

    #[test]
    fn state_updates_cached_logged_on_from_connection_operations() {
        let mut state = SteamworksUserState::default();
        let steam_id = steamworks::SteamId::from_raw(1);

        state.record_operation(&SteamworksUserOperation::CurrentUserInfoRead {
            info: SteamworksUserInfo {
                steam_id,
                level: 7,
                logged_on: false,
            },
        });
        state.record_operation(
            &SteamworksUserOperation::SteamServerConnectionEventReceived {
                event: SteamworksSteamServerConnectionEvent::Connected,
            },
        );

        assert_eq!(state.steam_server_connected(), Some(true));
        assert_eq!(
            state.current_user(),
            Some(&SteamworksUserInfo {
                steam_id,
                level: 7,
                logged_on: true,
            })
        );

        state.record_operation(&SteamworksUserOperation::LoggedOnRead { logged_on: false });

        assert_eq!(state.steam_server_connected(), Some(false));
        assert_eq!(
            state.current_user(),
            Some(&SteamworksUserInfo {
                steam_id,
                level: 7,
                logged_on: false,
            })
        );
    }

    #[test]
    fn state_records_user_operations_without_unbounded_history() {
        let mut state = SteamworksUserState::default();
        let first_user = steamworks::SteamId::from_raw(1);
        let second_user = steamworks::SteamId::from_raw(2);
        let app_id = steamworks::AppId(480);

        state.record_operation(&SteamworksUserOperation::CurrentUserInfoRead {
            info: SteamworksUserInfo {
                steam_id: first_user,
                level: 7,
                logged_on: false,
            },
        });
        state.record_operation(&SteamworksUserOperation::SteamIdRead {
            steam_id: second_user,
        });
        state.record_operation(&SteamworksUserOperation::LevelRead { level: 9 });
        state.record_operation(&SteamworksUserOperation::AuthenticationSessionStarted {
            user: first_user,
        });
        state.record_operation(&SteamworksUserOperation::AuthenticationSessionStarted {
            user: first_user,
        });
        state.record_operation(&SteamworksUserOperation::UserLicenseForAppRead {
            user: first_user,
            app_id,
            license: steamworks::UserHasLicense::HasLicense,
        });
        state.record_operation(&SteamworksUserOperation::AuthenticationSessionEnded {
            user: first_user,
        });

        assert_eq!(
            state.current_user(),
            Some(&SteamworksUserInfo {
                steam_id: second_user,
                level: 9,
                logged_on: false,
            })
        );
        assert_eq!(state.last_steam_id(), Some(second_user));
        assert_eq!(state.last_level(), Some(9));
        assert!(state.active_auth_tickets().is_empty());
        assert!(state.last_auth_session_ticket().is_none());
        assert_eq!(state.auth_session_ticket_issue_count(), 0);
        assert!(state.last_web_api_ticket_request().is_none());
        assert_eq!(state.web_api_ticket_request_count(), 0);
        assert!(state.last_cancelled_auth_ticket().is_none());
        assert_eq!(state.auth_ticket_cancel_count(), 0);
        assert!(state.authenticated_users().is_empty());
        assert_eq!(
            state.last_started_authentication_session(),
            Some(first_user)
        );
        assert_eq!(state.authentication_session_start_count(), 2);
        assert_eq!(state.last_ended_authentication_session(), Some(first_user));
        assert_eq!(state.authentication_session_end_count(), 1);
        assert_eq!(
            state.last_user_license_for_app(),
            Some(&SteamworksUserLicenseForApp {
                user: first_user,
                app_id,
                license: steamworks::UserHasLicense::HasLicense,
            })
        );
        assert_eq!(state.user_license_check_count(), 1);
    }

    #[test]
    fn validation_callbacks_do_not_create_sessions_but_failures_remove_known_sessions() {
        let mut state = SteamworksUserState::default();
        let user = steamworks::SteamId::from_raw(1);
        let owner = steamworks::SteamId::from_raw(2);

        state.record_operation(
            &SteamworksUserOperation::AuthenticationTicketValidationReceived {
                validation: SteamworksAuthTicketValidation {
                    steam_id: user,
                    owner_steam_id: owner,
                    response: Ok(()),
                },
            },
        );

        assert!(state.authenticated_users().is_empty());
        assert_eq!(
            state.last_auth_ticket_validation(),
            Some(&SteamworksAuthTicketValidation {
                steam_id: user,
                owner_steam_id: owner,
                response: Ok(()),
            })
        );

        state.record_operation(&SteamworksUserOperation::AuthenticationSessionStarted { user });
        assert_eq!(state.authenticated_users(), &[user]);

        state.record_operation(
            &SteamworksUserOperation::AuthenticationTicketValidationReceived {
                validation: SteamworksAuthTicketValidation {
                    steam_id: user,
                    owner_steam_id: owner,
                    response: Err(SteamworksAuthSessionValidateError::AuthTicketCancelled),
                },
            },
        );

        assert!(state.authenticated_users().is_empty());
        assert_eq!(
            state.last_auth_ticket_validation(),
            Some(&SteamworksAuthTicketValidation {
                steam_id: user,
                owner_steam_id: owner,
                response: Err(SteamworksAuthSessionValidateError::AuthTicketCancelled),
            })
        );
    }
}
