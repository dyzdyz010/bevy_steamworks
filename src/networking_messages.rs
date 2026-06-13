//! High-level Bevy ECS integration for Steam Networking Messages.
//!
//! This module builds on top of the upstream
//! [`steamworks::networking_messages::NetworkingMessages`] API. It exposes the
//! UDP-like Steam P2P message interface through Bevy commands/results while
//! copying received payloads into owned `Vec<u8>` values that are safe to keep
//! in ECS state.

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksSystem};

/// Maximum number of messages one receive command will pull in a single frame.
///
/// Steam's upstream wrapper allocates a temporary pointer buffer with the
/// requested batch size before calling the C API. Keeping this bounded prevents
/// one malformed command from forcing a huge frame-loop allocation.
pub const STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE: usize = 1024;

/// Bevy plugin for high-level Steam Networking Messages commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingMessagesCommand`] and
/// [`SteamworksNetworkingMessagesResult`] messages, installs the upstream
/// session callbacks once a [`SteamworksClient`] exists, and processes commands
/// in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug)]
pub struct SteamworksNetworkingMessagesPlugin {
    auto_accept_session_requests: bool,
}

impl Default for SteamworksNetworkingMessagesPlugin {
    fn default() -> Self {
        Self {
            auto_accept_session_requests: true,
        }
    }
}

impl SteamworksNetworkingMessagesPlugin {
    /// Creates a Networking Messages plugin with default behavior.
    ///
    /// Incoming session requests are accepted by default. Use
    /// [`Self::auto_accept_session_requests`] to opt out.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether incoming session requests are accepted in the Steam callback.
    ///
    /// The upstream safe API only allows accepting a session while handling the
    /// callback; it cannot defer the accept/reject decision to a later ECS frame.
    pub fn auto_accept_session_requests(mut self, enabled: bool) -> Self {
        self.auto_accept_session_requests = enabled;
        self
    }
}

impl Plugin for SteamworksNetworkingMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SteamworksNetworkingMessagesState::new(
            self.auto_accept_session_requests,
        ))
        .add_message::<SteamworksNetworkingMessagesCommand>()
        .add_message::<SteamworksNetworkingMessagesResult>()
        .configure_sets(
            First,
            SteamworksSystem::ProcessNetworkingMessagesCommands
                .after(SteamworksSystem::RunCallbacks)
                .before(bevy_ecs::message::MessageUpdateSystems),
        )
        .add_systems(
            First,
            (
                ensure_networking_messages_callbacks,
                apply_networking_messages_policy_commands,
            )
                .chain()
                .before(SteamworksSystem::RunCallbacks),
        )
        .add_systems(
            First,
            process_networking_messages_commands
                .in_set(SteamworksSystem::ProcessNetworkingMessagesCommands),
        );
    }
}

/// Runtime state for [`SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Resource)]
pub struct SteamworksNetworkingMessagesState {
    last_error: Option<SteamworksNetworkingMessagesError>,
    received_messages: Vec<SteamworksNetworkingMessage>,
    last_connection_info: Option<SteamworksNetworkingMessagesConnectionInfo>,
    last_session_request: Option<SteamworksNetworkingMessagesSessionRequestInfo>,
    last_session_failure: Option<SteamworksNetworkingMessagesConnectionInfo>,
    sent_count: u64,
    received_count: u64,
    session_request_count: u64,
    session_failure_count: u64,
    callbacks_registered: bool,
    auto_accept_session_requests: Arc<Mutex<bool>>,
    callback_results: Arc<Mutex<Vec<SteamworksNetworkingMessagesResult>>>,
}

impl Default for SteamworksNetworkingMessagesState {
    fn default() -> Self {
        Self::new(true)
    }
}

impl SteamworksNetworkingMessagesState {
    /// Creates state with the requested session request policy.
    pub fn new(auto_accept_session_requests: bool) -> Self {
        Self {
            last_error: None,
            received_messages: Vec::new(),
            last_connection_info: None,
            last_session_request: None,
            last_session_failure: None,
            sent_count: 0,
            received_count: 0,
            session_request_count: 0,
            session_failure_count: 0,
            callbacks_registered: false,
            auto_accept_session_requests: Arc::new(Mutex::new(auto_accept_session_requests)),
            callback_results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns the most recent synchronous command error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksNetworkingMessagesError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent batch of received messages.
    pub fn received_messages(&self) -> &[SteamworksNetworkingMessage] {
        &self.received_messages
    }

    /// Returns the most recent connection info snapshot read through the plugin.
    pub fn last_connection_info(&self) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.last_connection_info.as_ref()
    }

    /// Returns the most recent incoming session request observed by the callback.
    pub fn last_session_request(&self) -> Option<&SteamworksNetworkingMessagesSessionRequestInfo> {
        self.last_session_request.as_ref()
    }

    /// Returns the most recent session failure observed by the callback.
    pub fn last_session_failure(&self) -> Option<&SteamworksNetworkingMessagesConnectionInfo> {
        self.last_session_failure.as_ref()
    }

    /// Returns whether incoming session requests are currently auto-accepted.
    pub fn auto_accept_session_requests(&self) -> bool {
        *self
            .auto_accept_session_requests
            .lock()
            .expect("Steamworks Networking Messages policy mutex was poisoned")
    }

    /// Returns the number of successful send commands observed through the plugin.
    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    /// Returns the number of received messages observed through the plugin.
    pub fn received_count(&self) -> u64 {
        self.received_count
    }

    /// Returns the number of incoming session requests observed by the plugin.
    pub fn session_request_count(&self) -> u64 {
        self.session_request_count
    }

    /// Returns the number of session failures observed by the plugin.
    pub fn session_failure_count(&self) -> u64 {
        self.session_failure_count
    }

    fn record_error(&mut self, error: SteamworksNetworkingMessagesError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksNetworkingMessagesOperation) {
        match operation {
            SteamworksNetworkingMessagesOperation::MessageSent { .. } => {
                self.sent_count = self.sent_count.saturating_add(1);
            }
            SteamworksNetworkingMessagesOperation::MessagesReceived { messages, .. } => {
                self.received_count = self
                    .received_count
                    .saturating_add(messages.len().try_into().unwrap_or(u64::MAX));
                self.received_messages.clone_from(messages);
            }
            SteamworksNetworkingMessagesOperation::SessionConnectionInfoRead { info, .. } => {
                self.last_connection_info = Some(info.clone());
            }
            SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled } => {
                self.set_auto_accept_session_requests(*enabled);
            }
            SteamworksNetworkingMessagesOperation::SessionRequestReceived { request } => {
                self.session_request_count = self.session_request_count.saturating_add(1);
                self.last_session_request = Some(request.clone());
            }
            SteamworksNetworkingMessagesOperation::SessionFailed { info } => {
                self.session_failure_count = self.session_failure_count.saturating_add(1);
                self.last_session_failure = Some(info.clone());
            }
        }
    }

    fn set_auto_accept_session_requests(&self, enabled: bool) {
        *self
            .auto_accept_session_requests
            .lock()
            .expect("Steamworks Networking Messages policy mutex was poisoned") = enabled;
    }

    fn drain_callback_results(&self) -> Vec<SteamworksNetworkingMessagesResult> {
        self.callback_results
            .lock()
            .expect("Steamworks Networking Messages callback queue mutex was poisoned")
            .drain(..)
            .collect()
    }
}

/// Peer identity accepted by the high-level Networking Messages command layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingPeer {
    /// A Steam user identity.
    SteamId(steamworks::SteamId),
    /// An IP endpoint identity.
    Ip(SocketAddr),
    /// The local host identity.
    LocalHost,
    /// A prebuilt upstream Steam networking identity.
    Identity(steamworks::networking_types::NetworkingIdentity),
}

impl SteamworksNetworkingPeer {
    /// Creates a peer from a Steam ID.
    pub fn steam_id(id: steamworks::SteamId) -> Self {
        Self::SteamId(id)
    }

    /// Creates a peer from an IP socket address.
    pub fn ip(addr: SocketAddr) -> Self {
        Self::Ip(addr)
    }

    /// Creates a peer for the local host identity.
    pub fn local_host() -> Self {
        Self::LocalHost
    }

    /// Creates a peer from a prebuilt upstream identity.
    pub fn identity(identity: steamworks::networking_types::NetworkingIdentity) -> Self {
        Self::Identity(identity)
    }

    fn to_identity(&self) -> steamworks::networking_types::NetworkingIdentity {
        match self {
            Self::SteamId(id) => {
                steamworks::networking_types::NetworkingIdentity::new_steam_id(*id)
            }
            Self::Ip(addr) => steamworks::networking_types::NetworkingIdentity::new_ip(*addr),
            Self::LocalHost => {
                let mut identity = steamworks::networking_types::NetworkingIdentity::new();
                identity.set_local_host();
                identity
            }
            Self::Identity(identity) => identity.clone(),
        }
    }
}

/// Owned snapshot of one received Steam networking message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingMessage {
    /// Identity of the peer that sent the message.
    pub peer: steamworks::networking_types::NetworkingIdentity,
    /// Message payload copied from Steam's message handle.
    pub data: Vec<u8>,
    /// Channel carried by the message.
    pub channel: i32,
    /// Message flags reported by Steam.
    pub send_flags: steamworks::networking_types::SendFlags,
    /// Message number assigned by the sender.
    pub message_number: u64,
    /// Connection user data captured by Steam for this message.
    pub connection_user_data: i64,
}

/// Snapshot of one Networking Messages session request callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksNetworkingMessagesSessionRequestInfo {
    /// Remote peer requesting a session.
    pub remote: steamworks::networking_types::NetworkingIdentity,
    /// Whether the plugin accepted the request in the callback.
    pub accepted: bool,
}

/// Snapshot of one Networking Messages session connection state.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingMessagesConnectionInfo {
    /// High-level connection state reported by Steam.
    pub state: steamworks::networking_types::NetworkingConnectionState,
    /// Remote peer identity when Steam reports one.
    pub remote: Option<steamworks::networking_types::NetworkingIdentity>,
    /// Connection user data reported by Steam.
    pub user_data: Option<i64>,
    /// End reason reported by Steam when the session has ended.
    pub end_reason: Option<steamworks::networking_types::NetConnectionEnd>,
    /// Real-time status when Steam reports one for the session.
    pub realtime: Option<SteamworksNetworkingMessagesRealtimeInfo>,
}

/// Real-time Networking Messages session status.
#[derive(Clone, Debug, PartialEq)]
pub struct SteamworksNetworkingMessagesRealtimeInfo {
    /// Connection state in the real-time status block.
    pub connection_state: steamworks::networking_types::NetworkingConnectionState,
    /// Estimated ping in milliseconds.
    pub ping: i32,
    /// Local delivery quality between 0.0 and 1.0.
    pub connection_quality_local: f32,
    /// Remote delivery quality between 0.0 and 1.0.
    pub connection_quality_remote: f32,
    /// Outbound packet rate.
    pub out_packets_per_sec: f32,
    /// Outbound byte rate.
    pub out_bytes_per_sec: f32,
    /// Inbound packet rate.
    pub in_packets_per_sec: f32,
    /// Inbound byte rate.
    pub in_bytes_per_sec: f32,
    /// Estimated send capacity in bytes per second.
    pub send_rate_bytes_per_sec: i32,
    /// Pending unreliable bytes.
    pub pending_unreliable: i32,
    /// Pending reliable bytes.
    pub pending_reliable: i32,
}

/// A high-level command for Steam Networking Messages workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingMessagesCommand {
    /// Send one payload to a peer on a channel.
    SendMessage {
        /// Remote peer.
        peer: SteamworksNetworkingPeer,
        /// Delivery flags.
        send_flags: steamworks::networking_types::SendFlags,
        /// Routing channel.
        channel: u32,
        /// Payload to send.
        data: Vec<u8>,
    },
    /// Receive up to `batch_size` messages from one channel.
    ReceiveMessages {
        /// Routing channel.
        channel: u32,
        /// Maximum number of messages to receive.
        batch_size: usize,
    },
    /// Read connection information for one peer.
    GetSessionConnectionInfo {
        /// Peer to inspect.
        peer: SteamworksNetworkingPeer,
    },
    /// Set whether future incoming session requests are accepted in the callback.
    SetAutoAcceptSessionRequests {
        /// Whether session requests should be accepted.
        enabled: bool,
    },
}

impl SteamworksNetworkingMessagesCommand {
    /// Creates a [`SteamworksNetworkingMessagesCommand::SendMessage`] command.
    pub fn send_message(
        peer: SteamworksNetworkingPeer,
        send_flags: steamworks::networking_types::SendFlags,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendMessage {
            peer,
            send_flags,
            channel,
            data: data.into(),
        }
    }

    /// Creates a command that sends a payload to a Steam user.
    pub fn send_message_to_steam_id(
        steam_id: steamworks::SteamId,
        send_flags: steamworks::networking_types::SendFlags,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::send_message(
            SteamworksNetworkingPeer::steam_id(steam_id),
            send_flags,
            channel,
            data,
        )
    }

    /// Creates a [`SteamworksNetworkingMessagesCommand::ReceiveMessages`] command.
    pub fn receive_messages(channel: u32, batch_size: usize) -> Self {
        Self::ReceiveMessages {
            channel,
            batch_size,
        }
    }

    /// Creates a [`SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo`] command.
    pub fn get_session_connection_info(peer: SteamworksNetworkingPeer) -> Self {
        Self::GetSessionConnectionInfo { peer }
    }

    /// Creates a [`SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests`] command.
    pub fn set_auto_accept_session_requests(enabled: bool) -> Self {
        Self::SetAutoAcceptSessionRequests { enabled }
    }
}

/// A successfully submitted Steam Networking Messages operation or callback.
#[derive(Clone, Debug, PartialEq)]
pub enum SteamworksNetworkingMessagesOperation {
    /// A payload was sent to Steam.
    MessageSent {
        /// Remote peer.
        peer: SteamworksNetworkingPeer,
        /// Routing channel.
        channel: u32,
        /// Delivery flags.
        send_flags: steamworks::networking_types::SendFlags,
        /// Payload size in bytes.
        bytes: usize,
    },
    /// One receive command completed.
    MessagesReceived {
        /// Routing channel read.
        channel: u32,
        /// Owned message snapshots.
        messages: Vec<SteamworksNetworkingMessage>,
    },
    /// Connection info was read for a peer.
    SessionConnectionInfoRead {
        /// Peer inspected.
        peer: SteamworksNetworkingPeer,
        /// Connection snapshot.
        info: SteamworksNetworkingMessagesConnectionInfo,
    },
    /// The auto-accept session request policy was changed.
    AutoAcceptSessionRequestsSet {
        /// Whether future session requests are accepted.
        enabled: bool,
    },
    /// A session request callback was observed.
    SessionRequestReceived {
        /// Request snapshot.
        request: SteamworksNetworkingMessagesSessionRequestInfo,
    },
    /// A session failure callback was observed.
    SessionFailed {
        /// Failure connection snapshot.
        info: SteamworksNetworkingMessagesConnectionInfo,
    },
}

/// Result message emitted by [`SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Message, PartialEq)]
pub enum SteamworksNetworkingMessagesResult {
    /// The command, receive operation, or callback succeeded.
    Ok(SteamworksNetworkingMessagesOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingMessagesCommand,
        /// Failure reason.
        error: SteamworksNetworkingMessagesError,
    },
}

/// Synchronous command errors from [`SteamworksNetworkingMessagesPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingMessagesError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A peer identity is invalid.
    #[error("Steam networking peer identity is invalid")]
    InvalidIdentity,
    /// A channel exceeds Steam's signed 32-bit channel range.
    #[error("Steam networking channel {channel} exceeds i32::MAX")]
    InvalidChannel {
        /// Invalid channel.
        channel: u32,
    },
    /// A receive command used a zero batch size.
    #[error("Steam networking receive batch size must be greater than zero")]
    InvalidBatchSize,
    /// A receive command exceeded the per-frame batch cap.
    #[error("Steam networking receive batch size {batch_size} exceeds max {max_batch_size}")]
    BatchSizeTooLarge {
        /// Requested batch size.
        batch_size: usize,
        /// Maximum accepted batch size.
        max_batch_size: usize,
    },
    /// Steam returned an operation error.
    #[error("{operation} failed: {source}")]
    SteamError {
        /// Operation that failed.
        operation: &'static str,
        /// Error returned by Steam.
        source: steamworks::SteamError,
    },
}

impl SteamworksNetworkingMessagesError {
    fn steam_error(operation: &'static str, source: steamworks::SteamError) -> Self {
        Self::SteamError { operation, source }
    }
}

fn ensure_networking_messages_callbacks(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingMessagesState>,
) {
    if state.callbacks_registered {
        return;
    }

    let Some(client) = client else {
        return;
    };

    let request_queue = state.callback_results.clone();
    let auto_accept = state.auto_accept_session_requests.clone();
    client
        .networking_messages()
        .session_request_callback(move |request| {
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

    let failure_queue = state.callback_results.clone();
    client
        .networking_messages()
        .session_failed_callback(move |info| {
            let result = SteamworksNetworkingMessagesResult::Ok(
                SteamworksNetworkingMessagesOperation::SessionFailed {
                    info: snapshot_session_connection_info(
                        info.state().unwrap_or(
                            steamworks::networking_types::NetworkingConnectionState::None,
                        ),
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

    state.callbacks_registered = true;
    tracing::debug!(
        target: "bevy_steamworks",
        auto_accept_session_requests = state.auto_accept_session_requests(),
        "registered Steamworks Networking Messages callbacks"
    );
}

fn apply_networking_messages_policy_commands(
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

fn process_networking_messages_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingMessagesState>,
    mut commands: ResMut<Messages<SteamworksNetworkingMessagesCommand>>,
    mut results: MessageWriter<SteamworksNetworkingMessagesResult>,
) {
    for result in state.drain_callback_results() {
        record_networking_messages_result(&mut state, &result);
        results.write(result);
    }

    for command in commands.drain() {
        if let Err(error) = validate_command(&command) {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Networking Messages command failed"
            );
            results.write(SteamworksNetworkingMessagesResult::Err { command, error });
            continue;
        }

        if let SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled } =
            command
        {
            let operation =
                SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled };
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks Networking Messages command"
            );
            results.write(SteamworksNetworkingMessagesResult::Ok(operation));
            continue;
        }

        let Some(client) = client.as_ref() else {
            let error = SteamworksNetworkingMessagesError::ClientUnavailable;
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks Networking Messages command failed"
            );
            results.write(SteamworksNetworkingMessagesResult::Err { command, error });
            continue;
        };

        match handle_networking_messages_command(client, command.clone()) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks Networking Messages command"
                );
                results.write(SteamworksNetworkingMessagesResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks Networking Messages command failed"
                );
                results.write(SteamworksNetworkingMessagesResult::Err { command, error });
            }
        }
    }
}

fn record_networking_messages_result(
    state: &mut SteamworksNetworkingMessagesState,
    result: &SteamworksNetworkingMessagesResult,
) {
    match result {
        SteamworksNetworkingMessagesResult::Ok(operation) => state.record_operation(operation),
        SteamworksNetworkingMessagesResult::Err { error, .. } => {
            state.record_error(error.clone());
        }
    }
}

fn handle_networking_messages_command(
    client: &SteamworksClient,
    command: SteamworksNetworkingMessagesCommand,
) -> Result<SteamworksNetworkingMessagesOperation, SteamworksNetworkingMessagesError> {
    let networking_messages = client.networking_messages();
    Ok(match command {
        SteamworksNetworkingMessagesCommand::SendMessage {
            peer,
            send_flags,
            channel,
            data,
        } => {
            let bytes = data.len();
            networking_messages
                .send_message_to_user(peer.to_identity(), send_flags, &data, channel)
                .map_err(|source| {
                    SteamworksNetworkingMessagesError::steam_error(
                        "networking_messages.send_message_to_user",
                        source,
                    )
                })?;
            SteamworksNetworkingMessagesOperation::MessageSent {
                peer,
                channel,
                send_flags,
                bytes,
            }
        }
        SteamworksNetworkingMessagesCommand::ReceiveMessages {
            channel,
            batch_size,
        } => {
            let messages = networking_messages
                .receive_messages_on_channel(channel, batch_size)
                .into_iter()
                .map(snapshot_networking_message)
                .collect();
            SteamworksNetworkingMessagesOperation::MessagesReceived { channel, messages }
        }
        SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo { peer } => {
            let (state, info, realtime) =
                networking_messages.get_session_connection_info(&peer.to_identity());
            SteamworksNetworkingMessagesOperation::SessionConnectionInfoRead {
                peer,
                info: snapshot_session_connection_info(state, info.as_ref(), realtime.as_ref()),
            }
        }
        SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { enabled } => {
            SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled }
        }
    })
}

fn snapshot_networking_message(
    message: steamworks::networking_types::NetworkingMessage,
) -> SteamworksNetworkingMessage {
    SteamworksNetworkingMessage {
        peer: message.identity_peer(),
        data: message.data().to_vec(),
        channel: message.channel(),
        send_flags: message.send_flags(),
        message_number: u64::from(message.message_number()),
        connection_user_data: message.connection_user_data(),
    }
}

fn snapshot_session_connection_info(
    state: steamworks::networking_types::NetworkingConnectionState,
    info: Option<&steamworks::networking_types::NetConnectionInfo>,
    realtime: Option<&steamworks::networking_types::NetConnectionRealTimeInfo>,
) -> SteamworksNetworkingMessagesConnectionInfo {
    SteamworksNetworkingMessagesConnectionInfo {
        state,
        remote: info.and_then(|info| info.identity_remote()),
        user_data: info.map(|info| info.user_data()),
        end_reason: info.and_then(|info| info.end_reason()),
        realtime: realtime.map(snapshot_realtime_info),
    }
}

fn snapshot_realtime_info(
    info: &steamworks::networking_types::NetConnectionRealTimeInfo,
) -> SteamworksNetworkingMessagesRealtimeInfo {
    SteamworksNetworkingMessagesRealtimeInfo {
        connection_state: info
            .connection_state()
            .unwrap_or(steamworks::networking_types::NetworkingConnectionState::None),
        ping: info.ping(),
        connection_quality_local: info.connection_quality_local(),
        connection_quality_remote: info.connection_quality_remote(),
        out_packets_per_sec: info.out_packets_per_sec(),
        out_bytes_per_sec: info.out_bytes_per_sec(),
        in_packets_per_sec: info.in_packets_per_sec(),
        in_bytes_per_sec: info.in_bytes_per_sec(),
        send_rate_bytes_per_sec: info.send_rate_bytes_per_sec(),
        pending_unreliable: info.pending_unreliable(),
        pending_reliable: info.pending_reliable(),
    }
}

fn validate_command(
    command: &SteamworksNetworkingMessagesCommand,
) -> Result<(), SteamworksNetworkingMessagesError> {
    match command {
        SteamworksNetworkingMessagesCommand::SendMessage { peer, channel, .. } => {
            validate_peer(peer)?;
            validate_channel(*channel)
        }
        SteamworksNetworkingMessagesCommand::ReceiveMessages {
            channel,
            batch_size,
        } => {
            validate_channel(*channel)?;
            if *batch_size == 0 {
                return Err(SteamworksNetworkingMessagesError::InvalidBatchSize);
            }
            if *batch_size > STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE {
                return Err(SteamworksNetworkingMessagesError::BatchSizeTooLarge {
                    batch_size: *batch_size,
                    max_batch_size: STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE,
                });
            }
            Ok(())
        }
        SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo { peer } => {
            validate_peer(peer)
        }
        SteamworksNetworkingMessagesCommand::SetAutoAcceptSessionRequests { .. } => Ok(()),
    }
}

fn validate_peer(peer: &SteamworksNetworkingPeer) -> Result<(), SteamworksNetworkingMessagesError> {
    match peer {
        SteamworksNetworkingPeer::SteamId(id) if id.is_invalid() => {
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        }
        SteamworksNetworkingPeer::Identity(identity) if identity.is_invalid() => {
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        }
        _ => Ok(()),
    }
}

fn validate_channel(channel: u32) -> Result<(), SteamworksNetworkingMessagesError> {
    if channel > i32::MAX as u32 {
        return Err(SteamworksNetworkingMessagesError::InvalidChannel { channel });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use bevy_app::{App, First};
    use bevy_ecs::message::Messages;

    use super::*;

    #[test]
    fn networking_messages_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingMessagesPlugin::new());

        assert!(app
            .world()
            .contains_resource::<SteamworksNetworkingMessagesState>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingMessagesCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingMessagesResult>>());
        assert!(app
            .world()
            .resource::<SteamworksNetworkingMessagesState>()
            .auto_accept_session_requests());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingMessagesPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksNetworkingMessagesCommand>>()
            .write(SteamworksNetworkingMessagesCommand::receive_messages(0, 1));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksNetworkingMessagesResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksNetworkingMessagesResult::Err {
                command: SteamworksNetworkingMessagesCommand::receive_messages(0, 1),
                error: SteamworksNetworkingMessagesError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksNetworkingMessagesState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksNetworkingMessagesError::ClientUnavailable)
        );
    }

    #[test]
    fn local_auto_accept_command_updates_without_client() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingMessagesPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksNetworkingMessagesCommand>>()
            .write(SteamworksNetworkingMessagesCommand::set_auto_accept_session_requests(false));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksNetworkingMessagesResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksNetworkingMessagesResult::Ok(
                SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet {
                    enabled: false
                }
            )]
        );
        assert!(!app
            .world()
            .resource::<SteamworksNetworkingMessagesState>()
            .auto_accept_session_requests());
    }

    #[derive(Default, Resource)]
    struct ObservedAutoAccept(bool);

    fn observe_auto_accept_policy(
        state: Res<SteamworksNetworkingMessagesState>,
        mut observed: ResMut<ObservedAutoAccept>,
    ) {
        observed.0 = state.auto_accept_session_requests();
    }

    #[test]
    fn auto_accept_command_applies_before_run_callbacks_set() {
        let mut app = App::new();

        app.insert_resource(ObservedAutoAccept(true));
        app.add_plugins(SteamworksNetworkingMessagesPlugin::new());
        app.add_systems(
            First,
            observe_auto_accept_policy
                .after(SteamworksSystem::RunCallbacks)
                .before(SteamworksSystem::ProcessNetworkingMessagesCommands),
        );
        app.world_mut()
            .resource_mut::<Messages<SteamworksNetworkingMessagesCommand>>()
            .write(SteamworksNetworkingMessagesCommand::set_auto_accept_session_requests(false));

        app.update();

        assert!(!app.world().resource::<ObservedAutoAccept>().0);
    }

    #[test]
    fn validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(
                &SteamworksNetworkingMessagesCommand::send_message_to_steam_id(
                    steamworks::SteamId::from_raw(0),
                    steamworks::networking_types::SendFlags::RELIABLE,
                    0,
                    vec![1],
                )
            ),
            Err(SteamworksNetworkingMessagesError::InvalidIdentity)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingMessagesCommand::receive_messages(
                i32::MAX as u32 + 1,
                1,
            )),
            Err(SteamworksNetworkingMessagesError::InvalidChannel {
                channel: i32::MAX as u32 + 1,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingMessagesCommand::receive_messages(0, 0)),
            Err(SteamworksNetworkingMessagesError::InvalidBatchSize)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingMessagesCommand::receive_messages(
                0,
                STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE + 1,
            )),
            Err(SteamworksNetworkingMessagesError::BatchSizeTooLarge {
                batch_size: STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE + 1,
                max_batch_size: STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE,
            })
        );
    }

    #[test]
    fn constructors_preserve_inputs() {
        let peer = SteamworksNetworkingPeer::steam_id(steamworks::SteamId::from_raw(42));

        assert_eq!(
            SteamworksNetworkingMessagesCommand::send_message(
                peer.clone(),
                steamworks::networking_types::SendFlags::UNRELIABLE,
                7,
                [1, 2, 3],
            ),
            SteamworksNetworkingMessagesCommand::SendMessage {
                peer: peer.clone(),
                send_flags: steamworks::networking_types::SendFlags::UNRELIABLE,
                channel: 7,
                data: vec![1, 2, 3],
            }
        );
        assert_eq!(
            SteamworksNetworkingMessagesCommand::get_session_connection_info(peer.clone()),
            SteamworksNetworkingMessagesCommand::GetSessionConnectionInfo { peer }
        );
    }

    #[test]
    fn state_records_operations_without_unbounded_message_history() {
        let mut state = SteamworksNetworkingMessagesState::new(true);
        let peer = steamworks::networking_types::NetworkingIdentity::new_steam_id(
            steamworks::SteamId::from_raw(42),
        );
        let first = SteamworksNetworkingMessage {
            peer: peer.clone(),
            data: vec![1],
            channel: 0,
            send_flags: steamworks::networking_types::SendFlags::RELIABLE,
            message_number: 1,
            connection_user_data: 0,
        };
        let second = SteamworksNetworkingMessage {
            peer: peer.clone(),
            data: vec![2],
            channel: 0,
            send_flags: steamworks::networking_types::SendFlags::RELIABLE,
            message_number: 2,
            connection_user_data: 0,
        };

        state.record_operation(&SteamworksNetworkingMessagesOperation::MessagesReceived {
            channel: 0,
            messages: vec![first],
        });
        state.record_operation(&SteamworksNetworkingMessagesOperation::MessagesReceived {
            channel: 0,
            messages: vec![second.clone()],
        });
        state.record_operation(&SteamworksNetworkingMessagesOperation::MessageSent {
            peer: SteamworksNetworkingPeer::identity(peer.clone()),
            channel: 0,
            send_flags: steamworks::networking_types::SendFlags::RELIABLE,
            bytes: 3,
        });
        state.record_operation(
            &SteamworksNetworkingMessagesOperation::SessionRequestReceived {
                request: SteamworksNetworkingMessagesSessionRequestInfo {
                    remote: peer,
                    accepted: true,
                },
            },
        );
        state.record_operation(
            &SteamworksNetworkingMessagesOperation::AutoAcceptSessionRequestsSet { enabled: false },
        );

        assert_eq!(state.received_messages(), &[second]);
        assert_eq!(state.received_count(), 2);
        assert_eq!(state.sent_count(), 1);
        assert_eq!(state.session_request_count(), 1);
        assert!(!state.auto_accept_session_requests());
    }
}
