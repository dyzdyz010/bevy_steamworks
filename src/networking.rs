//! High-level Bevy ECS integration for Steam's legacy P2P Networking API.
//!
//! This module builds on top of the upstream [`steamworks::Networking`] API. It
//! exists for older Steam P2P workflows; new projects should prefer
//! [`crate::SteamworksNetworkingMessagesPlugin`].

use std::net::Ipv4Addr;

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{Message, MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

/// Maximum unreliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES: usize = 1_200;

/// Maximum reliable legacy P2P packet size accepted by Steam.
pub const STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES: usize = 1_048_576;

/// Maximum receive buffer this command layer will allocate in one frame.
pub const STEAMWORKS_P2P_MAX_READ_PACKET_BYTES: usize = STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES;

/// Bevy plugin for high-level legacy Steam P2P Networking commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksNetworkingCommand`] and [`SteamworksNetworkingResult`] messages,
/// observes legacy P2P callbacks from [`crate::SteamworksEvent`], and processes
/// commands in [`bevy_app::First`] after Steam callbacks.
#[derive(Clone, Debug, Default)]
pub struct SteamworksNetworkingPlugin;

impl SteamworksNetworkingPlugin {
    /// Creates a legacy P2P Networking plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksNetworkingState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksNetworkingCommand>()
            .add_message::<SteamworksNetworkingResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessNetworkingCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_networking_commands.in_set(SteamworksSystem::ProcessNetworkingCommands),
            );
    }
}

/// Runtime state for [`SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Default, Resource)]
pub struct SteamworksNetworkingState {
    last_error: Option<SteamworksNetworkingError>,
    last_accepted_session: Option<steamworks::SteamId>,
    last_closed_session: Option<steamworks::SteamId>,
    last_session_state: Option<SteamworksP2pSessionStateResult>,
    last_packet_availability: Option<SteamworksP2pPacketAvailability>,
    last_sent_packet: Option<SteamworksP2pPacketSent>,
    last_packet: Option<SteamworksP2pPacket>,
    sent_count: u64,
    received_count: u64,
    empty_read_count: u64,
    last_empty_read_channel: Option<u32>,
    session_request_count: u64,
    last_session_request: Option<steamworks::SteamId>,
    session_connect_failure_count: u64,
    last_session_connect_failure: Option<SteamworksP2pSessionConnectFailure>,
}

impl SteamworksNetworkingState {
    /// Returns the most recent synchronous command error observed by the plugin.
    pub fn last_error(&self) -> Option<&SteamworksNetworkingError> {
        self.last_error.as_ref()
    }

    /// Returns the most recent accepted legacy P2P session remote.
    pub fn last_accepted_session(&self) -> Option<steamworks::SteamId> {
        self.last_accepted_session
    }

    /// Returns the most recent closed legacy P2P session remote.
    pub fn last_closed_session(&self) -> Option<steamworks::SteamId> {
        self.last_closed_session
    }

    /// Returns the most recent P2P session state read through the plugin.
    pub fn last_session_state(&self) -> Option<&SteamworksP2pSessionStateResult> {
        self.last_session_state.as_ref()
    }

    /// Returns the most recent packet availability read through the plugin.
    pub fn last_packet_availability(&self) -> Option<&SteamworksP2pPacketAvailability> {
        self.last_packet_availability.as_ref()
    }

    /// Returns the most recent packet send submitted through the plugin.
    pub fn last_sent_packet(&self) -> Option<SteamworksP2pPacketSent> {
        self.last_sent_packet
    }

    /// Returns the most recent packet read through the plugin.
    pub fn last_packet(&self) -> Option<&SteamworksP2pPacket> {
        self.last_packet.as_ref()
    }

    /// Returns the number of successful send commands observed through the plugin.
    pub fn sent_count(&self) -> u64 {
        self.sent_count
    }

    /// Returns the number of packets read through the plugin.
    pub fn received_count(&self) -> u64 {
        self.received_count
    }

    /// Returns the number of read commands that found no queued packet.
    pub fn empty_read_count(&self) -> u64 {
        self.empty_read_count
    }

    /// Returns the most recent channel where a read command found no queued packet.
    pub fn last_empty_read_channel(&self) -> Option<u32> {
        self.last_empty_read_channel
    }

    /// Returns the number of incoming legacy P2P session requests observed.
    pub fn session_request_count(&self) -> u64 {
        self.session_request_count
    }

    /// Returns the most recent incoming legacy P2P session request remote.
    pub fn last_session_request(&self) -> Option<steamworks::SteamId> {
        self.last_session_request
    }

    /// Returns the number of legacy P2P session connection failures observed.
    pub fn session_connect_failure_count(&self) -> u64 {
        self.session_connect_failure_count
    }

    /// Returns the most recent legacy P2P session connection failure callback snapshot.
    pub fn last_session_connect_failure(&self) -> Option<SteamworksP2pSessionConnectFailure> {
        self.last_session_connect_failure
    }

    fn record_error(&mut self, error: SteamworksNetworkingError) {
        self.last_error = Some(error);
    }

    fn record_operation(&mut self, operation: &SteamworksNetworkingOperation) {
        match operation {
            SteamworksNetworkingOperation::SessionAccepted { user } => {
                self.last_accepted_session = Some(*user);
            }
            SteamworksNetworkingOperation::SessionClosed { user } => {
                self.last_closed_session = Some(*user);
                if self
                    .last_session_state
                    .as_ref()
                    .is_some_and(|state| state.user == *user)
                {
                    self.last_session_state = None;
                }
            }
            SteamworksNetworkingOperation::SessionStateRead { state } => {
                self.last_session_state = Some(state.clone());
            }
            SteamworksNetworkingOperation::PacketSent {
                remote,
                send_type,
                channel,
                bytes,
            } => {
                self.sent_count = self.sent_count.saturating_add(1);
                self.last_sent_packet = Some(SteamworksP2pPacketSent {
                    remote: *remote,
                    send_type: *send_type,
                    channel: *channel,
                    bytes: *bytes,
                });
            }
            SteamworksNetworkingOperation::PacketRead {
                packet: Some(packet),
                ..
            } => {
                self.received_count = self.received_count.saturating_add(1);
                self.last_packet = Some(packet.clone());
            }
            SteamworksNetworkingOperation::PacketRead {
                channel,
                packet: None,
            } => {
                self.empty_read_count = self.empty_read_count.saturating_add(1);
                self.last_empty_read_channel = Some(*channel);
            }
            SteamworksNetworkingOperation::PacketAvailabilityRead { availability } => {
                self.last_packet_availability = Some(availability.clone());
            }
            SteamworksNetworkingOperation::SessionRequestReceived { remote } => {
                self.session_request_count = self.session_request_count.saturating_add(1);
                self.last_session_request = Some(*remote);
            }
            SteamworksNetworkingOperation::SessionConnectFailed { remote, error } => {
                self.session_connect_failure_count =
                    self.session_connect_failure_count.saturating_add(1);
                self.last_session_connect_failure = Some(SteamworksP2pSessionConnectFailure {
                    remote: *remote,
                    error: *error,
                });
            }
        }
    }
}

/// Delivery mode for a legacy P2P packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SteamworksP2pSendType {
    /// Send directly over UDP. Payloads must be at most 1200 bytes.
    Unreliable,
    /// Like [`Self::Unreliable`], but does not buffer packets before connection start.
    UnreliableNoDelay,
    /// Reliable packet sending. Payloads must be at most 1 MiB.
    Reliable,
    /// Reliable sending with Steam's buffering behavior.
    ReliableWithBuffering,
}

impl SteamworksP2pSendType {
    fn to_steam(self) -> steamworks::SendType {
        match self {
            Self::Unreliable => steamworks::SendType::Unreliable,
            Self::UnreliableNoDelay => steamworks::SendType::UnreliableNoDelay,
            Self::Reliable => steamworks::SendType::Reliable,
            Self::ReliableWithBuffering => steamworks::SendType::ReliableWithBuffering,
        }
    }

    fn max_packet_bytes(self) -> usize {
        match self {
            Self::Unreliable | Self::UnreliableNoDelay => {
                STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES
            }
            Self::Reliable | Self::ReliableWithBuffering => {
                STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES
            }
        }
    }
}

/// Owned snapshot of a legacy P2P session state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pSessionState {
    /// Whether Steam reports an active open connection.
    pub connection_active: bool,
    /// Whether Steam is currently trying to establish the connection.
    pub connecting: bool,
    /// Last session error reported by Steam.
    pub error: steamworks::P2PSessionError,
    /// Whether the session is routed through a Steam relay server.
    pub using_relay: bool,
    /// Bytes queued for sending.
    pub bytes_queued_for_send: i32,
    /// Packets queued for sending.
    pub packets_queued_for_send: i32,
    /// Potential remote IP address, when Steam exposes one.
    pub remote_ip: Option<Ipv4Addr>,
    /// Potential remote port, when Steam exposes one.
    pub remote_port: Option<u16>,
}

impl From<steamworks::P2PSessionState> for SteamworksP2pSessionState {
    fn from(state: steamworks::P2PSessionState) -> Self {
        Self {
            connection_active: state.connection_active,
            connecting: state.connecting,
            error: state.error,
            using_relay: state.using_relay,
            bytes_queued_for_send: state.bytes_queued_for_send,
            packets_queued_for_send: state.packets_queued_for_send,
            remote_ip: state.remote_ip,
            remote_port: state.remote_port,
        }
    }
}

/// Result of reading legacy P2P session state for one user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pSessionStateResult {
    /// Remote Steam user inspected.
    pub user: steamworks::SteamId,
    /// Session state, or `None` when Steam reports no session for the user.
    pub state: Option<SteamworksP2pSessionState>,
}

/// Owned snapshot of one received legacy P2P packet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pPacket {
    /// Remote Steam user that sent the packet.
    pub remote: steamworks::SteamId,
    /// Channel the packet was read from.
    pub channel: u32,
    /// Packet payload.
    pub data: Vec<u8>,
}

/// Snapshot of one submitted legacy P2P packet send.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksP2pPacketSent {
    /// Remote Steam user targeted by the packet.
    pub remote: steamworks::SteamId,
    /// Delivery mode used for the send.
    pub send_type: SteamworksP2pSendType,
    /// Channel sent on.
    pub channel: u32,
    /// Payload size in bytes.
    pub bytes: usize,
}

/// Packet availability snapshot for one legacy P2P channel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksP2pPacketAvailability {
    /// Channel that was inspected.
    pub channel: u32,
    /// Available packet size in bytes, if a packet is queued.
    pub bytes: Option<usize>,
}

/// Legacy P2P session connection failure callback snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamworksP2pSessionConnectFailure {
    /// Remote Steam user.
    pub remote: steamworks::SteamId,
    /// Session error decoded from Steam's callback.
    pub error: steamworks::P2PSessionError,
}

/// A high-level command for Steam's legacy P2P Networking workflows.
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingCommand {
    /// Accept a legacy P2P session from a remote Steam user.
    AcceptP2pSession {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Close a legacy P2P session with a remote Steam user.
    CloseP2pSession {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Read the legacy P2P session state for a remote Steam user.
    GetP2pSessionState {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Send a legacy P2P packet to a remote Steam user.
    SendP2pPacket {
        /// Remote Steam user.
        remote: steamworks::SteamId,
        /// Delivery mode.
        send_type: SteamworksP2pSendType,
        /// Channel to send on.
        channel: u32,
        /// Payload to send.
        data: Vec<u8>,
    },
    /// Read whether a legacy P2P packet is queued on one channel.
    GetAvailablePacketSize {
        /// Channel to inspect.
        channel: u32,
    },
    /// Read one legacy P2P packet from one channel.
    ReadP2pPacket {
        /// Channel to read from.
        channel: u32,
        /// Receive buffer size to allocate for this read.
        max_bytes: usize,
    },
}

impl SteamworksNetworkingCommand {
    /// Creates a [`SteamworksNetworkingCommand::AcceptP2pSession`] command.
    pub fn accept_p2p_session(user: steamworks::SteamId) -> Self {
        Self::AcceptP2pSession { user }
    }

    /// Creates a [`SteamworksNetworkingCommand::CloseP2pSession`] command.
    pub fn close_p2p_session(user: steamworks::SteamId) -> Self {
        Self::CloseP2pSession { user }
    }

    /// Creates a [`SteamworksNetworkingCommand::GetP2pSessionState`] command.
    pub fn get_p2p_session_state(user: steamworks::SteamId) -> Self {
        Self::GetP2pSessionState { user }
    }

    /// Creates a [`SteamworksNetworkingCommand::SendP2pPacket`] command.
    pub fn send_p2p_packet(
        remote: steamworks::SteamId,
        send_type: SteamworksP2pSendType,
        channel: u32,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self::SendP2pPacket {
            remote,
            send_type,
            channel,
            data: data.into(),
        }
    }

    /// Creates a [`SteamworksNetworkingCommand::GetAvailablePacketSize`] command.
    pub fn get_available_packet_size(channel: u32) -> Self {
        Self::GetAvailablePacketSize { channel }
    }

    /// Creates a [`SteamworksNetworkingCommand::ReadP2pPacket`] command.
    pub fn read_p2p_packet(channel: u32, max_bytes: usize) -> Self {
        Self::ReadP2pPacket { channel, max_bytes }
    }
}

/// A successfully submitted legacy P2P operation, read, or callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksNetworkingOperation {
    /// A remote legacy P2P session was accepted.
    SessionAccepted {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// A remote legacy P2P session was closed.
    SessionClosed {
        /// Remote Steam user.
        user: steamworks::SteamId,
    },
    /// Session state was read for one remote Steam user.
    SessionStateRead {
        /// Session state result.
        state: SteamworksP2pSessionStateResult,
    },
    /// A packet was submitted to Steam.
    PacketSent {
        /// Remote Steam user.
        remote: steamworks::SteamId,
        /// Delivery mode.
        send_type: SteamworksP2pSendType,
        /// Channel sent on.
        channel: u32,
        /// Payload size in bytes.
        bytes: usize,
    },
    /// Packet availability was read for one channel.
    PacketAvailabilityRead {
        /// Availability snapshot.
        availability: SteamworksP2pPacketAvailability,
    },
    /// One read command completed.
    PacketRead {
        /// Channel read from.
        channel: u32,
        /// Packet snapshot, or `None` when no packet was available.
        packet: Option<SteamworksP2pPacket>,
    },
    /// A legacy P2P session request callback was observed.
    SessionRequestReceived {
        /// Remote Steam user requesting a session.
        remote: steamworks::SteamId,
    },
    /// A legacy P2P session connection failure callback was observed.
    SessionConnectFailed {
        /// Remote Steam user.
        remote: steamworks::SteamId,
        /// Session error decoded from Steam's callback.
        error: steamworks::P2PSessionError,
    },
}

/// Result message emitted by [`SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum SteamworksNetworkingResult {
    /// The command, read operation, or callback succeeded.
    Ok(SteamworksNetworkingOperation),
    /// The command failed synchronously.
    Err {
        /// Command that failed.
        command: SteamworksNetworkingCommand,
        /// Failure reason.
        error: SteamworksNetworkingError,
    },
}

/// Synchronous command errors from [`SteamworksNetworkingPlugin`].
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SteamworksNetworkingError {
    /// No [`SteamworksClient`] resource exists.
    #[error("SteamworksClient resource is not available")]
    ClientUnavailable,
    /// A Steam ID was zero.
    #[error("Steam networking command requires a non-zero Steam ID")]
    InvalidSteamId,
    /// A channel exceeds Steam's signed 32-bit channel range.
    #[error("Steam networking channel {channel} exceeds i32::MAX")]
    InvalidChannel {
        /// Invalid channel.
        channel: u32,
    },
    /// A send payload exceeded Steam's limit for the selected send type.
    #[error("Steam networking packet size {bytes} exceeds max {max_bytes}")]
    PacketTooLarge {
        /// Requested packet size.
        bytes: usize,
        /// Maximum accepted packet size.
        max_bytes: usize,
    },
    /// A read command used a zero buffer size.
    #[error("Steam networking read buffer size must be greater than zero")]
    InvalidReadBufferSize,
    /// A read command exceeded the per-frame allocation cap.
    #[error("Steam networking read buffer size {max_bytes} exceeds max {max_supported}")]
    ReadBufferTooLarge {
        /// Requested read buffer size.
        max_bytes: usize,
        /// Maximum accepted read buffer size.
        max_supported: usize,
    },
    /// A queued packet is larger than the requested read buffer.
    #[error(
        "Steam networking queued packet size {available_bytes} exceeds read buffer {max_bytes}"
    )]
    PacketExceedsReadBuffer {
        /// Queued packet size reported by Steam.
        available_bytes: usize,
        /// Requested read buffer size.
        max_bytes: usize,
    },
    /// Steam returned `false` for a boolean operation.
    #[error("{operation} failed")]
    OperationFailed {
        /// Operation that failed.
        operation: &'static str,
    },
}

impl SteamworksNetworkingError {
    fn operation_failed(operation: &'static str) -> Self {
        Self::OperationFailed { operation }
    }
}

fn process_networking_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksNetworkingState>,
    mut commands: ResMut<Messages<SteamworksNetworkingCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksNetworkingResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::P2PSessionRequest(event) => {
                Some(SteamworksNetworkingOperation::SessionRequestReceived {
                    remote: event.remote,
                })
            }
            SteamworksEvent::P2PSessionConnectFail(event) => {
                Some(SteamworksNetworkingOperation::SessionConnectFailed {
                    remote: event.remote,
                    error: steamworks::P2PSessionError::from(event.error),
                })
            }
            _ => None,
        };

        if let Some(operation) = operation {
            state.record_operation(&operation);
            tracing::debug!(
                target: "bevy_steamworks",
                operation = ?operation,
                "processed Steamworks legacy P2P callback"
            );
            results.write(SteamworksNetworkingResult::Ok(operation));
        }
    }

    let Some(client) = client else {
        let error = SteamworksNetworkingError::ClientUnavailable;
        state.record_error(error.clone());
        for command in commands.drain() {
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steamworks legacy networking command failed"
            );
            results.write(SteamworksNetworkingResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_networking_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks legacy networking command"
                );
                results.write(SteamworksNetworkingResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks legacy networking command failed"
                );
                results.write(SteamworksNetworkingResult::Err { command, error });
            }
        }
    }
}

fn handle_networking_command(
    client: &SteamworksClient,
    command: &SteamworksNetworkingCommand,
) -> Result<SteamworksNetworkingOperation, SteamworksNetworkingError> {
    validate_command(command)?;

    let networking = client.networking();
    Ok(match command {
        SteamworksNetworkingCommand::AcceptP2pSession { user } => {
            if !networking.accept_p2p_session(*user) {
                return Err(SteamworksNetworkingError::operation_failed(
                    "networking.accept_p2p_session",
                ));
            }
            SteamworksNetworkingOperation::SessionAccepted { user: *user }
        }
        SteamworksNetworkingCommand::CloseP2pSession { user } => {
            if !networking.close_p2p_session(*user) {
                return Err(SteamworksNetworkingError::operation_failed(
                    "networking.close_p2p_session",
                ));
            }
            SteamworksNetworkingOperation::SessionClosed { user: *user }
        }
        SteamworksNetworkingCommand::GetP2pSessionState { user } => {
            SteamworksNetworkingOperation::SessionStateRead {
                state: SteamworksP2pSessionStateResult {
                    user: *user,
                    state: networking
                        .get_p2p_session_state(*user)
                        .map(SteamworksP2pSessionState::from),
                },
            }
        }
        SteamworksNetworkingCommand::SendP2pPacket {
            remote,
            send_type,
            channel,
            data,
        } => {
            if !networking.send_p2p_packet_on_channel(
                *remote,
                send_type.to_steam(),
                data,
                *channel as i32,
            ) {
                return Err(SteamworksNetworkingError::operation_failed(
                    "networking.send_p2p_packet_on_channel",
                ));
            }
            SteamworksNetworkingOperation::PacketSent {
                remote: *remote,
                send_type: *send_type,
                channel: *channel,
                bytes: data.len(),
            }
        }
        SteamworksNetworkingCommand::GetAvailablePacketSize { channel } => {
            SteamworksNetworkingOperation::PacketAvailabilityRead {
                availability: SteamworksP2pPacketAvailability {
                    channel: *channel,
                    bytes: networking.is_p2p_packet_available_on_channel(*channel as i32),
                },
            }
        }
        SteamworksNetworkingCommand::ReadP2pPacket { channel, max_bytes } => {
            let Some(available_bytes) =
                networking.is_p2p_packet_available_on_channel(*channel as i32)
            else {
                return Ok(SteamworksNetworkingOperation::PacketRead {
                    channel: *channel,
                    packet: None,
                });
            };

            if available_bytes > *max_bytes {
                return Err(SteamworksNetworkingError::PacketExceedsReadBuffer {
                    available_bytes,
                    max_bytes: *max_bytes,
                });
            }

            let mut buffer = vec![0; *max_bytes];
            let packet = networking
                .read_p2p_packet_from_channel(&mut buffer, *channel as i32)
                .map(|(remote, bytes)| {
                    if bytes > buffer.len() {
                        return Err(SteamworksNetworkingError::PacketExceedsReadBuffer {
                            available_bytes: bytes,
                            max_bytes: buffer.len(),
                        });
                    }
                    buffer.truncate(bytes);
                    Ok(SteamworksP2pPacket {
                        remote,
                        channel: *channel,
                        data: buffer,
                    })
                })
                .transpose()?;
            SteamworksNetworkingOperation::PacketRead {
                channel: *channel,
                packet,
            }
        }
    })
}

fn validate_command(
    command: &SteamworksNetworkingCommand,
) -> Result<(), SteamworksNetworkingError> {
    match command {
        SteamworksNetworkingCommand::AcceptP2pSession { user }
        | SteamworksNetworkingCommand::CloseP2pSession { user }
        | SteamworksNetworkingCommand::GetP2pSessionState { user } => validate_steam_id(*user),
        SteamworksNetworkingCommand::SendP2pPacket {
            remote,
            send_type,
            channel,
            data,
        } => {
            validate_steam_id(*remote)?;
            validate_channel(*channel)?;
            let max_bytes = send_type.max_packet_bytes();
            if data.len() > max_bytes {
                return Err(SteamworksNetworkingError::PacketTooLarge {
                    bytes: data.len(),
                    max_bytes,
                });
            }
            Ok(())
        }
        SteamworksNetworkingCommand::GetAvailablePacketSize { channel } => {
            validate_channel(*channel)
        }
        SteamworksNetworkingCommand::ReadP2pPacket { channel, max_bytes } => {
            validate_channel(*channel)?;
            if *max_bytes == 0 {
                return Err(SteamworksNetworkingError::InvalidReadBufferSize);
            }
            if *max_bytes > STEAMWORKS_P2P_MAX_READ_PACKET_BYTES {
                return Err(SteamworksNetworkingError::ReadBufferTooLarge {
                    max_bytes: *max_bytes,
                    max_supported: STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
                });
            }
            Ok(())
        }
    }
}

fn validate_steam_id(user: steamworks::SteamId) -> Result<(), SteamworksNetworkingError> {
    if user.raw() == 0 {
        return Err(SteamworksNetworkingError::InvalidSteamId);
    }
    Ok(())
}

fn validate_channel(channel: u32) -> Result<(), SteamworksNetworkingError> {
    if channel > i32::MAX as u32 {
        return Err(SteamworksNetworkingError::InvalidChannel { channel });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::message::Messages;

    use super::*;

    fn user() -> steamworks::SteamId {
        steamworks::SteamId::from_raw(42)
    }

    fn packet(data: &[u8]) -> SteamworksP2pPacket {
        SteamworksP2pPacket {
            remote: user(),
            channel: 0,
            data: data.to_vec(),
        }
    }

    #[test]
    fn networking_plugin_registers_resources_and_messages() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingPlugin::new());

        assert!(app.world().contains_resource::<SteamworksNetworkingState>());
        assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingCommand>>());
        assert!(app
            .world()
            .contains_resource::<Messages<SteamworksNetworkingResult>>());
    }

    #[test]
    fn commands_fail_when_client_is_unavailable() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksNetworkingCommand>>()
            .write(SteamworksNetworkingCommand::get_available_packet_size(0));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksNetworkingResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![SteamworksNetworkingResult::Err {
                command: SteamworksNetworkingCommand::get_available_packet_size(0),
                error: SteamworksNetworkingError::ClientUnavailable,
            }]
        );

        let state = app.world().resource::<SteamworksNetworkingState>();
        assert_eq!(
            state.last_error(),
            Some(&SteamworksNetworkingError::ClientUnavailable)
        );
    }

    #[test]
    fn p2p_callback_events_are_bridged_without_client() {
        let mut app = App::new();

        app.add_plugins(SteamworksNetworkingPlugin::new());
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::P2PSessionRequest(
                steamworks::P2PSessionRequest { remote: user() },
            ));
        app.world_mut()
            .resource_mut::<Messages<SteamworksEvent>>()
            .write(SteamworksEvent::P2PSessionConnectFail(
                steamworks::P2PSessionConnectFail {
                    remote: user(),
                    error: 4,
                },
            ));

        app.update();

        let mut results = app
            .world_mut()
            .resource_mut::<Messages<SteamworksNetworkingResult>>();
        let drained = results.drain().collect::<Vec<_>>();

        assert_eq!(
            drained,
            vec![
                SteamworksNetworkingResult::Ok(
                    SteamworksNetworkingOperation::SessionRequestReceived { remote: user() },
                ),
                SteamworksNetworkingResult::Ok(
                    SteamworksNetworkingOperation::SessionConnectFailed {
                        remote: user(),
                        error: steamworks::P2PSessionError::Timeout,
                    },
                ),
            ]
        );

        let state = app.world().resource::<SteamworksNetworkingState>();
        assert_eq!(state.session_request_count(), 1);
        assert_eq!(state.last_session_request(), Some(user()));
        assert_eq!(state.session_connect_failure_count(), 1);
        assert_eq!(
            state.last_session_connect_failure(),
            Some(SteamworksP2pSessionConnectFailure {
                remote: user(),
                error: steamworks::P2PSessionError::Timeout,
            })
        );
    }

    #[test]
    fn validation_rejects_invalid_inputs() {
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::accept_p2p_session(
                steamworks::SteamId::from_raw(0),
            )),
            Err(SteamworksNetworkingError::InvalidSteamId)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::get_available_packet_size(
                i32::MAX as u32 + 1,
            )),
            Err(SteamworksNetworkingError::InvalidChannel {
                channel: i32::MAX as u32 + 1,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::send_p2p_packet(
                user(),
                SteamworksP2pSendType::Unreliable,
                0,
                vec![0; STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES + 1],
            )),
            Err(SteamworksNetworkingError::PacketTooLarge {
                bytes: STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES + 1,
                max_bytes: STEAMWORKS_P2P_MAX_UNRELIABLE_PACKET_BYTES,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::send_p2p_packet(
                user(),
                SteamworksP2pSendType::Reliable,
                0,
                vec![0; STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES + 1],
            )),
            Err(SteamworksNetworkingError::PacketTooLarge {
                bytes: STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES + 1,
                max_bytes: STEAMWORKS_P2P_MAX_RELIABLE_PACKET_BYTES,
            })
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::read_p2p_packet(0, 0)),
            Err(SteamworksNetworkingError::InvalidReadBufferSize)
        );
        assert_eq!(
            validate_command(&SteamworksNetworkingCommand::read_p2p_packet(
                0,
                STEAMWORKS_P2P_MAX_READ_PACKET_BYTES + 1,
            )),
            Err(SteamworksNetworkingError::ReadBufferTooLarge {
                max_bytes: STEAMWORKS_P2P_MAX_READ_PACKET_BYTES + 1,
                max_supported: STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
            })
        );
    }

    #[test]
    fn packet_exceeds_read_buffer_error_is_cloneable_and_comparable() {
        assert_eq!(
            SteamworksNetworkingError::PacketExceedsReadBuffer {
                available_bytes: 4097,
                max_bytes: 4096,
            }
            .clone(),
            SteamworksNetworkingError::PacketExceedsReadBuffer {
                available_bytes: 4097,
                max_bytes: 4096,
            }
        );
    }

    #[test]
    fn constructors_preserve_inputs() {
        assert_eq!(
            SteamworksNetworkingCommand::send_p2p_packet(
                user(),
                SteamworksP2pSendType::ReliableWithBuffering,
                7,
                vec![1, 2, 3],
            ),
            SteamworksNetworkingCommand::SendP2pPacket {
                remote: user(),
                send_type: SteamworksP2pSendType::ReliableWithBuffering,
                channel: 7,
                data: vec![1, 2, 3],
            }
        );
        assert_eq!(
            SteamworksNetworkingCommand::read_p2p_packet(3, 1024),
            SteamworksNetworkingCommand::ReadP2pPacket {
                channel: 3,
                max_bytes: 1024,
            }
        );
    }

    #[test]
    fn state_records_operations_without_unbounded_packet_history() {
        let mut state = SteamworksNetworkingState::default();
        let first = packet(&[1]);
        let second = packet(&[2, 3]);
        let session_state = SteamworksP2pSessionStateResult {
            user: user(),
            state: None,
        };

        state.record_operation(&SteamworksNetworkingOperation::SessionAccepted { user: user() });
        state.record_operation(&SteamworksNetworkingOperation::SessionStateRead {
            state: session_state.clone(),
        });
        state.record_operation(&SteamworksNetworkingOperation::PacketRead {
            channel: 0,
            packet: Some(first),
        });
        state.record_operation(&SteamworksNetworkingOperation::PacketRead {
            channel: 0,
            packet: Some(second.clone()),
        });
        state.record_operation(&SteamworksNetworkingOperation::PacketRead {
            channel: 7,
            packet: None,
        });
        state.record_operation(&SteamworksNetworkingOperation::PacketSent {
            remote: user(),
            send_type: SteamworksP2pSendType::Reliable,
            channel: 0,
            bytes: 3,
        });
        state.record_operation(&SteamworksNetworkingOperation::PacketAvailabilityRead {
            availability: SteamworksP2pPacketAvailability {
                channel: 0,
                bytes: Some(2),
            },
        });
        state.record_operation(&SteamworksNetworkingOperation::SessionRequestReceived {
            remote: user(),
        });
        state.record_operation(&SteamworksNetworkingOperation::SessionConnectFailed {
            remote: user(),
            error: steamworks::P2PSessionError::NoRightsToApp,
        });

        assert_eq!(state.last_accepted_session(), Some(user()));
        assert_eq!(state.last_session_state(), Some(&session_state));
        assert_eq!(state.received_count(), 2);
        assert_eq!(state.sent_count(), 1);
        assert_eq!(
            state.last_sent_packet(),
            Some(SteamworksP2pPacketSent {
                remote: user(),
                send_type: SteamworksP2pSendType::Reliable,
                channel: 0,
                bytes: 3,
            })
        );
        assert_eq!(state.empty_read_count(), 1);
        assert_eq!(state.last_empty_read_channel(), Some(7));
        assert_eq!(state.last_packet(), Some(&second));
        assert_eq!(
            state.last_packet_availability(),
            Some(&SteamworksP2pPacketAvailability {
                channel: 0,
                bytes: Some(2),
            })
        );
        assert_eq!(state.session_request_count(), 1);
        assert_eq!(state.last_session_request(), Some(user()));
        assert_eq!(state.session_connect_failure_count(), 1);
        assert_eq!(
            state.last_session_connect_failure(),
            Some(SteamworksP2pSessionConnectFailure {
                remote: user(),
                error: steamworks::P2PSessionError::NoRightsToApp,
            })
        );

        state.record_operation(&SteamworksNetworkingOperation::SessionClosed { user: user() });

        assert_eq!(state.last_closed_session(), Some(user()));
        assert!(state.last_session_state().is_none());
    }
}
