#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Bevy integration for the Steamworks SDK via [`steamworks`].
//!
//! The plugin initializes a Steamworks client, stores it as a Bevy resource,
//! and pumps Steam callbacks every frame.

use std::{ops::Deref, sync::Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageWriter},
    prelude::Resource,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::Res,
};
use thiserror::Error;

/// Re-export of the upstream Steamworks Rust bindings.
pub use steamworks;

/// Re-export of the upstream Steamworks API surface for ergonomic `use bevy_steamworks::*`.
pub use steamworks::*;
pub use steamworks::{
    networking_messages::{NetworkingMessagesSessionFailed, NetworkingMessagesSessionRequest},
    networking_types::NetConnectionStatusChanged,
    networking_utils::RelayNetworkStatusCallback,
    screenshots::{ScreenshotReady, ScreenshotRequested},
};

pub mod apps;
pub mod friends;
pub mod game_server;
pub mod input;
pub mod matchmaking;
pub mod matchmaking_servers;
pub mod networking;
pub mod networking_messages;
pub mod networking_sockets;
pub mod networking_utils;
pub mod remote_play;
pub mod remote_storage;
pub mod screenshots;
pub mod timeline;
pub mod ugc;
pub mod user;
pub mod user_stats;
pub mod utils;
pub use apps::*;
pub use friends::*;
pub use game_server::*;
pub use input::*;
pub use matchmaking::*;
pub use matchmaking_servers::*;
pub use networking::*;
pub use networking_messages::*;
pub use networking_sockets::*;
pub use networking_utils::*;
pub use remote_play::*;
pub use remote_storage::*;
pub use screenshots::*;
pub use timeline::*;
pub use ugc::*;
pub use user::*;
pub use user_stats::*;
pub use utils::*;

/// Common imports for Bevy apps using this crate.
pub mod prelude {
    pub use crate::{
        steamworks, SteamworksAchievementDisplayAttribute, SteamworksAchievementGlobalPercentage,
        SteamworksAchievementIcon, SteamworksAchievementIconStatus, SteamworksAchievementInfo,
        SteamworksAppsCommand, SteamworksAppsError, SteamworksAppsOperation, SteamworksAppsPlugin,
        SteamworksAppsResult, SteamworksAppsState, SteamworksAuthSessionError,
        SteamworksAuthSessionTicketResponse, SteamworksAuthSessionValidateError,
        SteamworksAuthTicketValidation, SteamworksAvatar, SteamworksAvatarSize,
        SteamworksCallbackRegistry, SteamworksClient, SteamworksConnectionRequestPolicy,
        SteamworksCoplayFriendInfo, SteamworksCurrentAppInfo, SteamworksEvent,
        SteamworksFailurePolicy, SteamworksFloatingGamepadTextInputDismissed,
        SteamworksFriendAvatar, SteamworksFriendGameInfo, SteamworksFriendInfo,
        SteamworksFriendRichPresenceValue, SteamworksFriendsCommand, SteamworksFriendsError,
        SteamworksFriendsOperation, SteamworksFriendsPlugin, SteamworksFriendsResult,
        SteamworksFriendsState, SteamworksGameServerItem, SteamworksGamepadTextInputDismissed,
        SteamworksGlobalStatHistory, SteamworksGlobalStatValue, SteamworksHasFriendResult,
        SteamworksInitMode, SteamworksInputActionOrigin, SteamworksInputActionOriginInfo,
        SteamworksInputActionSetActivation, SteamworksInputActionSetHandle,
        SteamworksInputAnalogActionData, SteamworksInputAnalogActionHandle,
        SteamworksInputAnalogActionOriginsSnapshot, SteamworksInputAnalogActionSnapshot,
        SteamworksInputCommand, SteamworksInputControllerInfo, SteamworksInputDigitalActionData,
        SteamworksInputDigitalActionHandle, SteamworksInputDigitalActionOriginsSnapshot,
        SteamworksInputDigitalActionSnapshot, SteamworksInputError, SteamworksInputHandle,
        SteamworksInputMotionData, SteamworksInputMotionSnapshot,
        SteamworksInputNamedActionSetHandle, SteamworksInputNamedAnalogActionHandle,
        SteamworksInputNamedDigitalActionHandle, SteamworksInputOperation, SteamworksInputPlugin,
        SteamworksInputResult, SteamworksInputSourceMode, SteamworksInputState,
        SteamworksInputType, SteamworksLeaderboardDataRequest, SteamworksLeaderboardDisplayType,
        SteamworksLeaderboardEntry, SteamworksLeaderboardId, SteamworksLeaderboardInfo,
        SteamworksLeaderboardScoreUploaded, SteamworksLeaderboardSortMethod,
        SteamworksLeaderboardUploadScoreMethod, SteamworksListenSocketEventInfo,
        SteamworksListenSocketId, SteamworksLobbyChatMessage, SteamworksLobbyChatUpdate,
        SteamworksLobbyCreatedCallback, SteamworksLobbyDataUpdate, SteamworksLobbyEnterCallback,
        SteamworksLobbyGameServer, SteamworksLobbyJoinRequest, SteamworksLobbyListFilter,
        SteamworksLobbyNearFilter, SteamworksLobbyNumberFilter, SteamworksLobbyStringFilter,
        SteamworksMatchmakingCommand, SteamworksMatchmakingError, SteamworksMatchmakingOperation,
        SteamworksMatchmakingPlugin, SteamworksMatchmakingResult,
        SteamworksMatchmakingServersCommand, SteamworksMatchmakingServersError,
        SteamworksMatchmakingServersOperation, SteamworksMatchmakingServersPlugin,
        SteamworksMatchmakingServersResult, SteamworksMatchmakingServersState,
        SteamworksMatchmakingState, SteamworksMicroTxnAuthorizationResponse,
        SteamworksNetworkingCommand, SteamworksNetworkingError, SteamworksNetworkingMessage,
        SteamworksNetworkingMessagesCommand, SteamworksNetworkingMessagesConnectionInfo,
        SteamworksNetworkingMessagesError, SteamworksNetworkingMessagesOperation,
        SteamworksNetworkingMessagesPlugin, SteamworksNetworkingMessagesRealtimeInfo,
        SteamworksNetworkingMessagesResult, SteamworksNetworkingMessagesSessionRequestInfo,
        SteamworksNetworkingMessagesState, SteamworksNetworkingOperation, SteamworksNetworkingPeer,
        SteamworksNetworkingPlugin, SteamworksNetworkingResult, SteamworksNetworkingSocketsCommand,
        SteamworksNetworkingSocketsConnectionEventInfo, SteamworksNetworkingSocketsConnectionId,
        SteamworksNetworkingSocketsConnectionInfo, SteamworksNetworkingSocketsConnectionTarget,
        SteamworksNetworkingSocketsError, SteamworksNetworkingSocketsListenEndpoint,
        SteamworksNetworkingSocketsMessage, SteamworksNetworkingSocketsOperation,
        SteamworksNetworkingSocketsPlugin, SteamworksNetworkingSocketsPollGroupId,
        SteamworksNetworkingSocketsPollGroupMessage, SteamworksNetworkingSocketsRealtimeLaneStatus,
        SteamworksNetworkingSocketsRealtimeStatus, SteamworksNetworkingSocketsResult,
        SteamworksNetworkingSocketsState, SteamworksNetworkingState,
        SteamworksNetworkingUtilsCommand, SteamworksNetworkingUtilsError,
        SteamworksNetworkingUtilsOperation, SteamworksNetworkingUtilsPlugin,
        SteamworksNetworkingUtilsResult, SteamworksNetworkingUtilsState,
        SteamworksNotificationPosition, SteamworksOverlayStoreActivation,
        SteamworksOverlayToStoreAction, SteamworksOverlayUserActivation, SteamworksP2pPacket,
        SteamworksP2pPacketAvailability, SteamworksP2pSendType, SteamworksP2pSessionState,
        SteamworksP2pSessionStateResult, SteamworksPersonaStateChange, SteamworksPlugin,
        SteamworksRelayNetworkStatus, SteamworksRemotePlayCommand, SteamworksRemotePlayError,
        SteamworksRemotePlayInvite, SteamworksRemotePlayOperation, SteamworksRemotePlayPlugin,
        SteamworksRemotePlayResult, SteamworksRemotePlaySessionInfo,
        SteamworksRemotePlaySessionSnapshot, SteamworksRemotePlayState,
        SteamworksRemoteStorageCloudInfo, SteamworksRemoteStorageCommand,
        SteamworksRemoteStorageError, SteamworksRemoteStorageFileInfo,
        SteamworksRemoteStorageFileShareHandle, SteamworksRemoteStorageFileSummary,
        SteamworksRemoteStorageOperation, SteamworksRemoteStoragePlugin,
        SteamworksRemoteStorageResult, SteamworksRemoteStorageSharedFile,
        SteamworksRemoteStorageState, SteamworksRichPresenceChange,
        SteamworksRichPresenceJoinRequest, SteamworksScreenshotLibraryError,
        SteamworksScreenshotReady, SteamworksScreenshotReadyError, SteamworksScreenshotsCommand,
        SteamworksScreenshotsError, SteamworksScreenshotsOperation, SteamworksScreenshotsPlugin,
        SteamworksScreenshotsResult, SteamworksScreenshotsState, SteamworksServer,
        SteamworksServerCallbackRegistry, SteamworksServerClientApproval,
        SteamworksServerClientDenial, SteamworksServerClientGroupStatus,
        SteamworksServerClientKick, SteamworksServerCommand, SteamworksServerConfig,
        SteamworksServerError, SteamworksServerInitMode, SteamworksServerListFilters,
        SteamworksServerListKind, SteamworksServerListReleaseError, SteamworksServerListRequestId,
        SteamworksServerListResponse, SteamworksServerLoginToken, SteamworksServerOperation,
        SteamworksServerOutgoingPacket, SteamworksServerPlugin, SteamworksServerResult,
        SteamworksServerState, SteamworksServerUnavailable, SteamworksStatsCommand,
        SteamworksStatsError, SteamworksStatsOperation, SteamworksStatsPlugin,
        SteamworksStatsResult, SteamworksStatsSettings, SteamworksStatsState,
        SteamworksSteamServerConnectionEvent, SteamworksSubmittedScreenshot, SteamworksSystem,
        SteamworksTimelineCommand, SteamworksTimelineError, SteamworksTimelineEventClipPriority,
        SteamworksTimelineEventInfo, SteamworksTimelineGameMode, SteamworksTimelineOperation,
        SteamworksTimelinePlugin, SteamworksTimelineResult, SteamworksTimelineState,
        SteamworksTimelineStateDescription, SteamworksUgcCommand, SteamworksUgcContentDescriptor,
        SteamworksUgcDownloadItemResult, SteamworksUgcError, SteamworksUgcItemDetails,
        SteamworksUgcItemDownloadInfo, SteamworksUgcItemDownloadInfoResult,
        SteamworksUgcItemInstallInfo, SteamworksUgcItemInstallInfoResult,
        SteamworksUgcItemStateInfo, SteamworksUgcItemUpdate, SteamworksUgcItemUpdateProgress,
        SteamworksUgcItemUpdateTags, SteamworksUgcOperation, SteamworksUgcPlugin,
        SteamworksUgcQuery, SteamworksUgcQueryOptions, SteamworksUgcQueryResults,
        SteamworksUgcResult, SteamworksUgcState, SteamworksUgcStatistic, SteamworksUnavailable,
        SteamworksUserAchievementStored, SteamworksUserCommand, SteamworksUserError,
        SteamworksUserGameInvite, SteamworksUserInfo, SteamworksUserInformationRequest,
        SteamworksUserOperation, SteamworksUserPlugin, SteamworksUserResult, SteamworksUserState,
        SteamworksUserStatsReceived, SteamworksUserStatsStored, SteamworksUtilsCommand,
        SteamworksUtilsError, SteamworksUtilsInfo, SteamworksUtilsOperation, SteamworksUtilsPlugin,
        SteamworksUtilsResult, SteamworksUtilsState, SteamworksWebApiTicketResponse,
        STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND,
        STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND, STEAMWORKS_LEADERBOARD_MAX_DETAILS,
        STEAMWORKS_LEADERBOARD_MAX_ENTRIES_PER_COMMAND,
        STEAMWORKS_MATCHMAKING_SERVER_FILTER_MAX_BYTES,
        STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE,
        STEAMWORKS_NETWORKING_SOCKETS_MAX_CONFIGURED_LANES,
        STEAMWORKS_NETWORKING_SOCKETS_MAX_EVENTS_PER_COMMAND,
        STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGES_PER_COMMAND,
        STEAMWORKS_NETWORKING_SOCKETS_MAX_MESSAGE_BYTES,
        STEAMWORKS_NETWORKING_SOCKETS_MAX_REALTIME_LANES, STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
        STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES, STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES,
        STEAMWORKS_SERVER_QUERY_PACKET_BUFFER_BYTES, STEAMWORKS_UGC_MAX_ITEMS_PER_COMMAND,
        STEAMWORKS_UGC_MAX_UPDATE_DESCRIPTION_BYTES, STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAGS,
        STEAMWORKS_UGC_MAX_UPDATE_KEY_VALUE_TAG_REMOVALS, STEAMWORKS_UGC_MAX_UPDATE_METADATA_BYTES,
        STEAMWORKS_UGC_MAX_UPDATE_TAG_BYTES, STEAMWORKS_UGC_MAX_UPDATE_TITLE_BYTES,
    };
    pub use steamworks::*;
    pub use steamworks::{
        networking_messages::{NetworkingMessagesSessionFailed, NetworkingMessagesSessionRequest},
        networking_types::NetConnectionStatusChanged,
        networking_utils::RelayNetworkStatusCallback,
        screenshots::{ScreenshotReady, ScreenshotRequested},
    };
}

/// How [`SteamworksPlugin`] should create or locate the Steamworks client.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SteamworksInitMode {
    /// Use [`steamworks::Client::init`] and let Steam determine the app id.
    ///
    /// This requires launching through Steam or providing `steam_appid.txt`.
    #[default]
    Automatic,
    /// Force a specific Steam app id through [`steamworks::Client::init_app`].
    AppId(AppId),
    /// Do not initialize Steamworks.
    ///
    /// This is useful when another layer inserts [`SteamworksClient`] manually,
    /// or for tests that only need the plugin schedules and messages.
    Manual,
}

impl SteamworksInitMode {
    fn app_id(self) -> Option<u32> {
        match self {
            Self::Automatic | Self::Manual => None,
            Self::AppId(app_id) => Some(app_id.0),
        }
    }
}

/// How the plugin reacts when Steamworks cannot be initialized.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SteamworksFailurePolicy {
    /// Panic during plugin build.
    ///
    /// This is the default so a Steam-required game cannot silently continue
    /// without Steamworks.
    #[default]
    Panic,
    /// Log the error, insert [`SteamworksUnavailable`], and keep the app running.
    LogAndContinue,
}

/// Resource inserted when Steamworks is explicitly allowed to be unavailable.
#[derive(Clone, Debug, Error, PartialEq, Eq, Resource)]
pub enum SteamworksUnavailable {
    /// Manual mode was selected, but no [`SteamworksClient`] resource was present.
    #[error(
        "manual Steamworks initialization was selected, but no SteamworksClient resource exists"
    )]
    ManualClientMissing,
    /// The upstream Steamworks initialization call returned an error.
    #[error("Steamworks initialization failed in {mode:?}: {source}")]
    InitFailed {
        /// Initialization mode used for the failed attempt.
        mode: SteamworksInitMode,
        /// Error returned by `steamworks`.
        source: SteamAPIInitError,
    },
}

impl SteamworksUnavailable {
    fn init_failed(mode: SteamworksInitMode, source: SteamAPIInitError) -> Self {
        Self::InitFailed { mode, source }
    }
}

/// A Bevy resource wrapping [`steamworks::Client`].
#[derive(Clone, Resource)]
pub struct SteamworksClient(steamworks::Client);

impl SteamworksClient {
    /// Creates a Bevy resource from an initialized Steamworks client.
    pub fn new(client: steamworks::Client) -> Self {
        Self(client)
    }

    /// Returns the underlying Steamworks client.
    pub fn inner(&self) -> &steamworks::Client {
        &self.0
    }

    /// Clones the underlying Steamworks client handle.
    pub fn clone_inner(&self) -> steamworks::Client {
        self.0.clone()
    }
}

impl From<steamworks::Client> for SteamworksClient {
    fn from(client: steamworks::Client) -> Self {
        Self::new(client)
    }
}

impl Deref for SteamworksClient {
    type Target = steamworks::Client;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

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

/// Stores Steamworks callback handles so callbacks stay registered.
#[derive(Default, Resource)]
pub struct SteamworksCallbackRegistry {
    handles: Vec<CallbackHandle>,
}

impl SteamworksCallbackRegistry {
    /// Registers a typed Steamworks callback and stores its handle.
    pub fn register<C, F>(&mut self, client: &SteamworksClient, callback: F)
    where
        C: Callback,
        F: FnMut(C) + Send + 'static,
    {
        self.handles.push(client.register_callback(callback));
    }

    /// Drops every registered callback handle.
    pub fn clear(&mut self) {
        self.handles.clear();
    }

    /// Number of callback handles currently held.
    pub fn len(&self) -> usize {
        self.handles.len()
    }

    /// Returns true when no callback handles are held.
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
}

/// A Bevy plugin that integrates Steamworks into an app.
pub struct SteamworksPlugin {
    mode: SteamworksInitMode,
    failure_policy: SteamworksFailurePolicy,
    run_callbacks: bool,
    client: Mutex<Option<steamworks::Client>>,
}

impl Default for SteamworksPlugin {
    fn default() -> Self {
        Self::automatic()
    }
}

impl SteamworksPlugin {
    /// Creates a plugin that initializes Steamworks from the environment.
    ///
    /// This uses [`steamworks::Client::init`].
    pub fn automatic() -> Self {
        Self::with_mode(SteamworksInitMode::Automatic)
    }

    /// Creates a plugin that initializes Steamworks with a specific app id.
    ///
    /// This uses [`steamworks::Client::init_app`] when the plugin is built.
    pub fn app_id(app_id: impl Into<AppId>) -> Self {
        Self::with_mode(SteamworksInitMode::AppId(app_id.into()))
    }

    /// Creates a plugin that does not initialize Steamworks.
    ///
    /// Use this when you insert [`SteamworksClient`] yourself, or when tests only
    /// need the plugin's schedule and message setup.
    pub fn manual() -> Self {
        Self::with_mode(SteamworksInitMode::Manual)
    }

    /// Initializes Steamworks immediately from the environment and wraps it.
    pub fn init() -> Result<Self, SteamAPIInitError> {
        steamworks::Client::init().map(Self::from_client)
    }

    /// Initializes Steamworks immediately with a specific app id and wraps it.
    pub fn init_app(app_id: impl Into<AppId>) -> Result<Self, SteamAPIInitError> {
        steamworks::Client::init_app(app_id.into()).map(Self::from_client)
    }

    /// Creates a plugin from an already initialized Steamworks client.
    pub fn from_client(client: steamworks::Client) -> Self {
        Self {
            mode: SteamworksInitMode::Manual,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            client: Mutex::new(Some(client)),
        }
    }

    /// Sets the initialization failure policy.
    pub fn failure_policy(mut self, policy: SteamworksFailurePolicy) -> Self {
        self.failure_policy = policy;
        self
    }

    /// Keeps the Bevy app running when Steamworks cannot be initialized.
    ///
    /// The plugin will insert [`SteamworksUnavailable`] and emit a structured
    /// `tracing` error.
    pub fn log_and_continue(self) -> Self {
        self.failure_policy(SteamworksFailurePolicy::LogAndContinue)
    }

    /// Sets whether the plugin should automatically run Steam callbacks.
    pub fn run_callbacks(mut self, run_callbacks: bool) -> Self {
        self.run_callbacks = run_callbacks;
        self
    }

    fn with_mode(mode: SteamworksInitMode) -> Self {
        Self {
            mode,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            client: Mutex::new(None),
        }
    }

    fn initialize_client(&self) -> Result<steamworks::Client, SteamworksUnavailable> {
        let injected = self
            .client
            .lock()
            .expect("SteamworksPlugin client mutex was poisoned")
            .take();

        if let Some(client) = injected {
            return Ok(client);
        }

        match self.mode {
            SteamworksInitMode::Automatic => steamworks::Client::init().map_err(|source| {
                SteamworksUnavailable::init_failed(SteamworksInitMode::Automatic, source)
            }),
            SteamworksInitMode::AppId(app_id) => {
                steamworks::Client::init_app(app_id).map_err(|source| {
                    SteamworksUnavailable::init_failed(SteamworksInitMode::AppId(app_id), source)
                })
            }
            SteamworksInitMode::Manual => Err(SteamworksUnavailable::ManualClientMissing),
        }
    }

    fn handle_unavailable(&self, app: &mut App, error: SteamworksUnavailable) {
        match self.failure_policy {
            SteamworksFailurePolicy::Panic => panic!("{error}"),
            SteamworksFailurePolicy::LogAndContinue => {
                tracing::error!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    app_id = ?self.mode.app_id(),
                    error = %error,
                    "Steamworks unavailable"
                );
                app.insert_resource(error);
            }
        }
    }
}

impl From<steamworks::Client> for SteamworksPlugin {
    fn from(client: steamworks::Client) -> Self {
        Self::from_client(client)
    }
}

impl Plugin for SteamworksPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SteamworksEvent>()
            .init_resource::<SteamworksCallbackRegistry>();

        if self.run_callbacks {
            app.configure_sets(First, SteamworksSystem::RunCallbacks)
                .add_systems(
                    First,
                    run_steam_callbacks
                        .in_set(SteamworksSystem::RunCallbacks)
                        .before(bevy_ecs::message::MessageUpdateSystems),
                );
        }

        if app.world().contains_resource::<SteamworksClient>() {
            tracing::debug!(
                target: "bevy_steamworks",
                init_mode = ?self.mode,
                "using existing SteamworksClient resource"
            );
            return;
        }

        match self.initialize_client() {
            Ok(client) => {
                tracing::info!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    app_id = ?self.mode.app_id(),
                    "Steamworks initialized"
                );
                app.insert_resource(SteamworksClient::new(client));
            }
            Err(error) => self.handle_unavailable(app, error),
        }
    }
}

/// System sets used by [`SteamworksPlugin`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum SteamworksSystem {
    /// Runs Steamworks callbacks and writes [`SteamworksEvent`] messages.
    RunCallbacks,
    /// Processes high-level Steam user stats and achievements commands.
    ProcessStatsCommands,
    /// Processes high-level Steam friends, Rich Presence, overlay, and invite commands.
    ProcessFriendsCommands,
    /// Processes high-level Steam matchmaking and lobby commands.
    ProcessMatchmakingCommands,
    /// Processes high-level Steam Matchmaking Servers commands.
    ProcessMatchmakingServersCommands,
    /// Processes high-level Steam Game Server commands.
    ProcessServerCommands,
    /// Processes high-level Steam app and launch-parameter commands.
    ProcessAppsCommands,
    /// Processes high-level Steam user identity and authentication commands.
    ProcessUserCommands,
    /// Processes high-level Steam utility commands.
    ProcessUtilsCommands,
    /// Processes high-level Steam screenshots commands.
    ProcessScreenshotsCommands,
    /// Processes high-level Steam Remote Play commands.
    ProcessRemotePlayCommands,
    /// Processes high-level Steam Remote Storage commands.
    ProcessRemoteStorageCommands,
    /// Processes high-level Steam Timeline commands.
    ProcessTimelineCommands,
    /// Processes high-level Steam Input commands.
    ProcessInputCommands,
    /// Processes high-level legacy Steam P2P Networking commands.
    ProcessNetworkingCommands,
    /// Processes high-level Steam Networking Messages commands.
    ProcessNetworkingMessagesCommands,
    /// Processes high-level Steam Networking Sockets commands.
    ProcessNetworkingSocketsCommands,
    /// Processes high-level Steam Networking Utils commands.
    ProcessNetworkingUtilsCommands,
    /// Processes high-level Steam Workshop / UGC commands.
    ProcessUgcCommands,
}

fn run_steam_callbacks(
    client: Option<Res<SteamworksClient>>,
    mut output: MessageWriter<SteamworksEvent>,
) {
    let Some(client) = client else {
        return;
    };

    client.process_callbacks(|callback| {
        output.write(SteamworksEvent::from(callback));
    });
}

#[cfg(test)]
mod tests {
    use bevy_app::App;

    use super::*;

    #[test]
    fn manual_mode_can_continue_without_client() {
        let mut app = App::new();

        app.add_plugins(SteamworksPlugin::manual().log_and_continue());

        assert!(app.world().contains_resource::<SteamworksUnavailable>());
        assert!(app
            .world()
            .contains_resource::<SteamworksCallbackRegistry>());
        assert!(!app.world().contains_resource::<SteamworksClient>());

        app.update();
    }

    #[test]
    #[should_panic(expected = "manual Steamworks initialization was selected")]
    fn manual_mode_panics_by_default() {
        let mut app = App::new();

        app.add_plugins(SteamworksPlugin::manual());
    }

    #[test]
    fn callback_registry_tracks_handles() {
        let registry = SteamworksCallbackRegistry::default();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
}
