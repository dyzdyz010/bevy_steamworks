use bevy_ecs::message::Message;
use steamworks::{
    networking_messages::{NetworkingMessagesSessionFailed, NetworkingMessagesSessionRequest},
    networking_types::NetConnectionStatusChanged,
    networking_utils::RelayNetworkStatusCallback,
    screenshots::{ScreenshotReady, ScreenshotRequested},
    *,
};

/// Bevy message emitted for typed Steamworks callback processing.
#[derive(Debug, Message)]
#[allow(missing_docs)]
pub enum SteamworksEvent {
    AuthSessionTicketResponse(AuthSessionTicketResponse),
    DownloadItemResult(DownloadItemResult),
    FloatingGamepadTextInputDismissed(FloatingGamepadTextInputDismissed),
    GameLobbyJoinRequested(GameLobbyJoinRequested),
    GameOverlayActivated(GameOverlayActivated),
    GamepadTextInputDismissed(GamepadTextInputDismissed),
    GameRichPresenceJoinRequested(GameRichPresenceJoinRequested),
    LobbyChatMsg(LobbyChatMsg),
    LobbyChatUpdate(LobbyChatUpdate),
    LobbyCreated(LobbyCreated),
    LobbyDataUpdate(LobbyDataUpdate),
    LobbyEnter(LobbyEnter),
    MicroTxnAuthorizationResponse(MicroTxnAuthorizationResponse),
    NetConnectionStatusChanged(NetConnectionStatusChanged),
    NetworkingMessagesSessionFailed(NetworkingMessagesSessionFailed),
    NetworkingMessagesSessionRequest(NetworkingMessagesSessionRequest),
    P2PSessionConnectFail(P2PSessionConnectFail),
    P2PSessionRequest(P2PSessionRequest),
    PersonaStateChange(PersonaStateChange),
    RelayNetworkStatusCallback(RelayNetworkStatusCallback),
    RemotePlayConnected(RemotePlayConnected),
    RemotePlayDisconnected(RemotePlayDisconnected),
    ScreenshotRequested(ScreenshotRequested),
    ScreenshotReady(ScreenshotReady),
    SteamServerConnectFailure(SteamServerConnectFailure),
    SteamServersConnected(SteamServersConnected),
    SteamServersDisconnected(SteamServersDisconnected),
    TicketForWebApiResponse(TicketForWebApiResponse),
    UserAchievementStored(UserAchievementStored),
    UserAchievementIconFetched(UserAchievementIconFetched),
    UserStatsReceived(UserStatsReceived),
    UserStatsStored(UserStatsStored),
    ValidateAuthTicketResponse(ValidateAuthTicketResponse),
    GSClientApprove(GSClientApprove),
    GSClientDeny(GSClientDeny),
    GSClientKick(GSClientKick),
    GSClientGroupStatus(GSClientGroupStatus),
    NewUrlLaunchParameters(NewUrlLaunchParameters),
}

impl From<CallbackResult> for SteamworksEvent {
    fn from(callback: CallbackResult) -> Self {
        match callback {
            CallbackResult::AuthSessionTicketResponse(callback) => {
                Self::AuthSessionTicketResponse(callback)
            }
            CallbackResult::DownloadItemResult(callback) => Self::DownloadItemResult(callback),
            CallbackResult::FloatingGamepadTextInputDismissed(callback) => {
                Self::FloatingGamepadTextInputDismissed(callback)
            }
            CallbackResult::GameLobbyJoinRequested(callback) => {
                Self::GameLobbyJoinRequested(callback)
            }
            CallbackResult::GameOverlayActivated(callback) => Self::GameOverlayActivated(callback),
            CallbackResult::GamepadTextInputDismissed(callback) => {
                Self::GamepadTextInputDismissed(callback)
            }
            CallbackResult::GameRichPresenceJoinRequested(callback) => {
                Self::GameRichPresenceJoinRequested(callback)
            }
            CallbackResult::LobbyChatMsg(callback) => Self::LobbyChatMsg(callback),
            CallbackResult::LobbyChatUpdate(callback) => Self::LobbyChatUpdate(callback),
            CallbackResult::LobbyCreated(callback) => Self::LobbyCreated(callback),
            CallbackResult::LobbyDataUpdate(callback) => Self::LobbyDataUpdate(callback),
            CallbackResult::LobbyEnter(callback) => Self::LobbyEnter(callback),
            CallbackResult::MicroTxnAuthorizationResponse(callback) => {
                Self::MicroTxnAuthorizationResponse(callback)
            }
            CallbackResult::NetConnectionStatusChanged(callback) => {
                Self::NetConnectionStatusChanged(callback)
            }
            CallbackResult::NetworkingMessagesSessionFailed(callback) => {
                Self::NetworkingMessagesSessionFailed(callback)
            }
            CallbackResult::NetworkingMessagesSessionRequest(callback) => {
                Self::NetworkingMessagesSessionRequest(callback)
            }
            CallbackResult::P2PSessionConnectFail(callback) => {
                Self::P2PSessionConnectFail(callback)
            }
            CallbackResult::P2PSessionRequest(callback) => Self::P2PSessionRequest(callback),
            CallbackResult::PersonaStateChange(callback) => Self::PersonaStateChange(callback),
            CallbackResult::RelayNetworkStatusCallback(callback) => {
                Self::RelayNetworkStatusCallback(callback)
            }
            CallbackResult::RemotePlayConnected(callback) => Self::RemotePlayConnected(callback),
            CallbackResult::RemotePlayDisconnected(callback) => {
                Self::RemotePlayDisconnected(callback)
            }
            CallbackResult::ScreenshotRequested(callback) => Self::ScreenshotRequested(callback),
            CallbackResult::ScreenshotReady(callback) => Self::ScreenshotReady(callback),
            CallbackResult::SteamServerConnectFailure(callback) => {
                Self::SteamServerConnectFailure(callback)
            }
            CallbackResult::SteamServersConnected(callback) => {
                Self::SteamServersConnected(callback)
            }
            CallbackResult::SteamServersDisconnected(callback) => {
                Self::SteamServersDisconnected(callback)
            }
            CallbackResult::TicketForWebApiResponse(callback) => {
                Self::TicketForWebApiResponse(callback)
            }
            CallbackResult::UserAchievementStored(callback) => {
                Self::UserAchievementStored(callback)
            }
            CallbackResult::UserAchievementIconFetched(callback) => {
                Self::UserAchievementIconFetched(callback)
            }
            CallbackResult::UserStatsReceived(callback) => Self::UserStatsReceived(callback),
            CallbackResult::UserStatsStored(callback) => Self::UserStatsStored(callback),
            CallbackResult::ValidateAuthTicketResponse(callback) => {
                Self::ValidateAuthTicketResponse(callback)
            }
            CallbackResult::GSClientApprove(callback) => Self::GSClientApprove(callback),
            CallbackResult::GSClientDeny(callback) => Self::GSClientDeny(callback),
            CallbackResult::GSClientKick(callback) => Self::GSClientKick(callback),
            CallbackResult::GSClientGroupStatus(callback) => Self::GSClientGroupStatus(callback),
            CallbackResult::NewUrlLaunchParameters(callback) => {
                Self::NewUrlLaunchParameters(callback)
            }
        }
    }
}

impl From<SteamworksEvent> for CallbackResult {
    fn from(event: SteamworksEvent) -> Self {
        match event {
            SteamworksEvent::AuthSessionTicketResponse(event) => {
                Self::AuthSessionTicketResponse(event)
            }
            SteamworksEvent::DownloadItemResult(event) => Self::DownloadItemResult(event),
            SteamworksEvent::FloatingGamepadTextInputDismissed(event) => {
                Self::FloatingGamepadTextInputDismissed(event)
            }
            SteamworksEvent::GameLobbyJoinRequested(event) => Self::GameLobbyJoinRequested(event),
            SteamworksEvent::GameOverlayActivated(event) => Self::GameOverlayActivated(event),
            SteamworksEvent::GamepadTextInputDismissed(event) => {
                Self::GamepadTextInputDismissed(event)
            }
            SteamworksEvent::GameRichPresenceJoinRequested(event) => {
                Self::GameRichPresenceJoinRequested(event)
            }
            SteamworksEvent::LobbyChatMsg(event) => Self::LobbyChatMsg(event),
            SteamworksEvent::LobbyChatUpdate(event) => Self::LobbyChatUpdate(event),
            SteamworksEvent::LobbyCreated(event) => Self::LobbyCreated(event),
            SteamworksEvent::LobbyDataUpdate(event) => Self::LobbyDataUpdate(event),
            SteamworksEvent::LobbyEnter(event) => Self::LobbyEnter(event),
            SteamworksEvent::MicroTxnAuthorizationResponse(event) => {
                Self::MicroTxnAuthorizationResponse(event)
            }
            SteamworksEvent::NetConnectionStatusChanged(event) => {
                Self::NetConnectionStatusChanged(event)
            }
            SteamworksEvent::NetworkingMessagesSessionFailed(event) => {
                Self::NetworkingMessagesSessionFailed(event)
            }
            SteamworksEvent::NetworkingMessagesSessionRequest(event) => {
                Self::NetworkingMessagesSessionRequest(event)
            }
            SteamworksEvent::P2PSessionConnectFail(event) => Self::P2PSessionConnectFail(event),
            SteamworksEvent::P2PSessionRequest(event) => Self::P2PSessionRequest(event),
            SteamworksEvent::PersonaStateChange(event) => Self::PersonaStateChange(event),
            SteamworksEvent::RelayNetworkStatusCallback(event) => {
                Self::RelayNetworkStatusCallback(event)
            }
            SteamworksEvent::RemotePlayConnected(event) => Self::RemotePlayConnected(event),
            SteamworksEvent::RemotePlayDisconnected(event) => Self::RemotePlayDisconnected(event),
            SteamworksEvent::ScreenshotRequested(event) => Self::ScreenshotRequested(event),
            SteamworksEvent::ScreenshotReady(event) => Self::ScreenshotReady(event),
            SteamworksEvent::SteamServerConnectFailure(event) => {
                Self::SteamServerConnectFailure(event)
            }
            SteamworksEvent::SteamServersConnected(event) => Self::SteamServersConnected(event),
            SteamworksEvent::SteamServersDisconnected(event) => {
                Self::SteamServersDisconnected(event)
            }
            SteamworksEvent::TicketForWebApiResponse(event) => Self::TicketForWebApiResponse(event),
            SteamworksEvent::UserAchievementStored(event) => Self::UserAchievementStored(event),
            SteamworksEvent::UserAchievementIconFetched(event) => {
                Self::UserAchievementIconFetched(event)
            }
            SteamworksEvent::UserStatsReceived(event) => Self::UserStatsReceived(event),
            SteamworksEvent::UserStatsStored(event) => Self::UserStatsStored(event),
            SteamworksEvent::ValidateAuthTicketResponse(event) => {
                Self::ValidateAuthTicketResponse(event)
            }
            SteamworksEvent::GSClientApprove(event) => Self::GSClientApprove(event),
            SteamworksEvent::GSClientDeny(event) => Self::GSClientDeny(event),
            SteamworksEvent::GSClientKick(event) => Self::GSClientKick(event),
            SteamworksEvent::GSClientGroupStatus(event) => Self::GSClientGroupStatus(event),
            SteamworksEvent::NewUrlLaunchParameters(event) => Self::NewUrlLaunchParameters(event),
        }
    }
}
