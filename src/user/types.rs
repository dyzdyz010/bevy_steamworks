use thiserror::Error;

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

/// Auth session ticket issued for a networking identity through this command layer.
#[derive(Clone, PartialEq, Eq)]
pub struct SteamworksIssuedAuthSessionTicketForIdentity {
    /// Ticket handle that should be cancelled when no longer needed.
    pub ticket: steamworks::AuthTicket,
    /// Raw ticket bytes returned by Steam.
    ///
    /// Treat this as credential material; avoid logging it or storing it longer than needed.
    pub ticket_bytes: Vec<u8>,
    /// Networking identity used as the verifier.
    pub identity: steamworks::networking_types::NetworkingIdentity,
}

impl std::fmt::Debug for SteamworksIssuedAuthSessionTicketForIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamworksIssuedAuthSessionTicketForIdentity")
            .field("ticket", &self.ticket)
            .field("ticket_bytes_len", &self.ticket_bytes.len())
            .field("identity", &self.identity)
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
