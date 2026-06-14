//! Bevy ECS integration for Steam Game Server initialization and callbacks.
//!
//! This module builds on top of the upstream [`steamworks::Server`] API. It
//! inserts a Bevy resource for the initialized game server, pumps Steam Game
//! Server callbacks into the shared [`crate::SteamworksEvent`] stream, and
//! mirrors relevant callback confirmations into [`SteamworksServerResult`].

use std::{cell::RefCell, net::Ipv4Addr, ops::Deref, sync::Mutex};

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut, Resource},
    schedule::IntoScheduleConfigs,
};
use thiserror::Error;

use crate::{
    user::{
        SteamworksAuthSessionTicketResponse, SteamworksAuthTicketValidation,
        SteamworksSteamServerConnectionEvent,
    },
    SteamworksEvent, SteamworksFailurePolicy, SteamworksSystem,
};

mod messages;
mod registry;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use registry::SteamworksServerCallbackRegistry;
pub use state::SteamworksServerState;
pub use types::*;

/// Required buffer size for Steam Game Server shared-query outgoing packets.
pub const STEAMWORKS_SERVER_QUERY_PACKET_BUFFER_BYTES: usize = 16 * 1024;

/// Configuration used to initialize [`steamworks::Server`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksServerConfig {
    /// IPv4 address the Steam Game Server API should bind to.
    pub ip: Ipv4Addr,
    /// Game traffic port.
    pub game_port: u16,
    /// Server browser query port.
    ///
    /// Use [`steamworks::QUERY_PORT_SHARED`] when game and query traffic share
    /// the same socket.
    pub query_port: u16,
    /// Upstream Steam server mode.
    pub server_mode: steamworks::ServerMode,
    /// Version string reported to Steam.
    pub version: String,
}

impl SteamworksServerConfig {
    /// Creates a Steam Game Server initialization config.
    pub fn new(
        ip: Ipv4Addr,
        game_port: u16,
        query_port: u16,
        server_mode: steamworks::ServerMode,
        version: impl Into<String>,
    ) -> Self {
        Self {
            ip,
            game_port,
            query_port,
            server_mode,
            version: version.into(),
        }
    }
}

/// How [`SteamworksServerPlugin`] should create or locate the Steamworks server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamworksServerInitMode {
    /// Initialize a Steam Game Server from the supplied config.
    Config(SteamworksServerConfig),
    /// Do not initialize Steamworks.
    ///
    /// This is useful when another layer inserts [`SteamworksServer`] manually,
    /// or for tests that only need plugin schedules and messages.
    Manual,
}

/// Resource inserted when Steam Game Server initialization is explicitly allowed to fail.
#[derive(Clone, Debug, Error, PartialEq, Eq, Resource)]
pub enum SteamworksServerUnavailable {
    /// Manual mode was selected, but no [`SteamworksServer`] resource was present.
    #[error(
        "manual Steam Game Server initialization was selected, but no SteamworksServer resource exists"
    )]
    ManualServerMissing,
    /// A string passed to Steam contains an interior NUL byte.
    #[error("Steam Game Server config field {field} contains an interior NUL byte")]
    InvalidString {
        /// Field that contained the invalid string.
        field: &'static str,
    },
    /// The upstream Steam Game Server initialization call returned an error.
    #[error("Steam Game Server initialization failed with {config:?}: {source}")]
    InitFailed {
        /// Initialization config used for the failed attempt.
        config: SteamworksServerConfig,
        /// Error returned by `steamworks`.
        source: steamworks::SteamAPIInitError,
    },
}

impl SteamworksServerUnavailable {
    fn invalid_string(field: &'static str) -> Self {
        Self::InvalidString { field }
    }

    fn init_failed(config: SteamworksServerConfig, source: steamworks::SteamAPIInitError) -> Self {
        Self::InitFailed { config, source }
    }
}

/// A Bevy resource wrapping [`steamworks::Server`].
#[derive(Clone, Resource)]
pub struct SteamworksServer(steamworks::Server);

impl SteamworksServer {
    /// Creates a Bevy resource from an initialized Steam Game Server.
    pub fn new(server: steamworks::Server) -> Self {
        Self(server)
    }

    /// Returns the underlying Steam Game Server handle.
    pub fn inner(&self) -> &steamworks::Server {
        &self.0
    }

    /// Clones the underlying Steam Game Server handle.
    pub fn clone_inner(&self) -> steamworks::Server {
        self.0.clone()
    }
}

impl From<steamworks::Server> for SteamworksServer {
    fn from(server: steamworks::Server) -> Self {
        Self::new(server)
    }
}

impl Deref for SteamworksServer {
    type Target = steamworks::Server;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

/// A Bevy plugin that integrates Steam Game Server callbacks into an app.
pub struct SteamworksServerPlugin {
    mode: SteamworksServerInitMode,
    failure_policy: SteamworksFailurePolicy,
    run_callbacks: bool,
    server: Mutex<Option<steamworks::Server>>,
}

impl SteamworksServerPlugin {
    /// Creates a plugin that initializes Steam Game Server from a config.
    pub fn new(config: SteamworksServerConfig) -> Self {
        Self::with_mode(SteamworksServerInitMode::Config(config))
    }

    /// Creates a plugin that does not initialize Steam Game Server.
    ///
    /// Use this when you insert [`SteamworksServer`] yourself, or when tests only
    /// need the plugin's schedule and message setup.
    pub fn manual() -> Self {
        Self::with_mode(SteamworksServerInitMode::Manual)
    }

    /// Initializes Steam Game Server immediately and wraps it.
    pub fn init(config: SteamworksServerConfig) -> Result<Self, SteamworksServerUnavailable> {
        validate_server_config(&config)?;
        let (server, _server_client) = steamworks::Server::init(
            config.ip,
            config.game_port,
            config.query_port,
            config.server_mode,
            &config.version,
        )
        .map_err(|source| SteamworksServerUnavailable::init_failed(config, source))?;
        Ok(Self::from_server(server))
    }

    /// Creates a plugin from an already initialized Steam Game Server.
    pub fn from_server(server: steamworks::Server) -> Self {
        Self {
            mode: SteamworksServerInitMode::Manual,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            server: Mutex::new(Some(server)),
        }
    }

    /// Sets the initialization failure policy.
    pub fn failure_policy(mut self, policy: SteamworksFailurePolicy) -> Self {
        self.failure_policy = policy;
        self
    }

    /// Keeps the Bevy app running when Steam Game Server cannot be initialized.
    ///
    /// The plugin will insert [`SteamworksServerUnavailable`] and emit a
    /// structured `tracing` error.
    pub fn log_and_continue(self) -> Self {
        self.failure_policy(SteamworksFailurePolicy::LogAndContinue)
    }

    /// Sets whether the plugin should automatically run Steam Game Server callbacks.
    pub fn run_callbacks(mut self, run_callbacks: bool) -> Self {
        self.run_callbacks = run_callbacks;
        self
    }

    fn with_mode(mode: SteamworksServerInitMode) -> Self {
        Self {
            mode,
            failure_policy: SteamworksFailurePolicy::Panic,
            run_callbacks: true,
            server: Mutex::new(None),
        }
    }

    fn initialize_server(&self) -> Result<steamworks::Server, SteamworksServerUnavailable> {
        let injected = self
            .server
            .lock()
            .expect("SteamworksServerPlugin server mutex was poisoned")
            .take();

        if let Some(server) = injected {
            return Ok(server);
        }

        match &self.mode {
            SteamworksServerInitMode::Config(config) => {
                validate_server_config(config)?;
                let (server, _server_client) = steamworks::Server::init(
                    config.ip,
                    config.game_port,
                    config.query_port,
                    config.server_mode,
                    &config.version,
                )
                .map_err(|source| {
                    SteamworksServerUnavailable::init_failed(config.clone(), source)
                })?;
                Ok(server)
            }
            SteamworksServerInitMode::Manual => {
                Err(SteamworksServerUnavailable::ManualServerMissing)
            }
        }
    }

    fn handle_unavailable(&self, app: &mut App, error: SteamworksServerUnavailable) {
        match self.failure_policy {
            SteamworksFailurePolicy::Panic => panic!("{error}"),
            SteamworksFailurePolicy::LogAndContinue => {
                tracing::error!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    error = %error,
                    "Steam Game Server unavailable"
                );
                app.insert_resource(error);
            }
        }
    }
}

impl From<steamworks::Server> for SteamworksServerPlugin {
    fn from(server: steamworks::Server) -> Self {
        Self::from_server(server)
    }
}

impl Plugin for SteamworksServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksServerState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksServerCommand>()
            .add_message::<SteamworksServerResult>()
            .init_resource::<SteamworksServerCallbackRegistry>();

        if self.run_callbacks {
            app.configure_sets(First, SteamworksSystem::RunCallbacks)
                .add_systems(
                    First,
                    run_steam_server_callbacks
                        .in_set(SteamworksSystem::RunCallbacks)
                        .before(bevy_ecs::message::MessageUpdateSystems),
                );
        }

        app.configure_sets(
            First,
            SteamworksSystem::ProcessServerCommands
                .after(SteamworksSystem::RunCallbacks)
                .before(bevy_ecs::message::MessageUpdateSystems),
        )
        .add_systems(
            First,
            process_server_commands.in_set(SteamworksSystem::ProcessServerCommands),
        );

        if app.world().contains_resource::<SteamworksServer>() {
            tracing::debug!(
                target: "bevy_steamworks",
                init_mode = ?self.mode,
                "using existing SteamworksServer resource"
            );
            return;
        }

        match self.initialize_server() {
            Ok(server) => {
                tracing::info!(
                    target: "bevy_steamworks",
                    init_mode = ?self.mode,
                    "Steam Game Server initialized"
                );
                app.insert_resource(SteamworksServer::new(server));
            }
            Err(error) => self.handle_unavailable(app, error),
        }
    }
}

fn process_server_commands(
    server: Option<Res<SteamworksServer>>,
    mut state: ResMut<SteamworksServerState>,
    mut commands: ResMut<Messages<SteamworksServerCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksServerResult>,
) {
    process_server_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(server) = server else {
        let error = SteamworksServerError::ServerUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            tracing::error!(
                target: "bevy_steamworks",
                command = ?command,
                error = %error,
                "Steam Game Server command failed"
            );
            results.write(SteamworksServerResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_server_command(&server, &state, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steam Game Server command"
                );
                results.write(SteamworksServerResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steam Game Server command failed"
                );
                results.write(SteamworksServerResult::Err { command, error });
            }
        }
    }
}

fn process_server_steam_events(
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

fn handle_server_command(
    server: &SteamworksServer,
    state: &SteamworksServerState,
    command: &SteamworksServerCommand,
) -> Result<SteamworksServerOperation, SteamworksServerError> {
    validate_server_command_for_state(command, state)?;

    Ok(match command {
        SteamworksServerCommand::GetSteamId => SteamworksServerOperation::SteamIdRead {
            steam_id: server.steam_id(),
        },
        SteamworksServerCommand::GetAuthenticationSessionTicket { steam_id } => {
            let (ticket, ticket_bytes) =
                server.authentication_session_ticket_with_steam_id(*steam_id);
            SteamworksServerOperation::AuthenticationSessionTicketIssued {
                ticket,
                ticket_bytes,
                steam_id: *steam_id,
            }
        }
        SteamworksServerCommand::CancelAuthenticationTicket { ticket } => {
            server.cancel_authentication_ticket(*ticket);
            SteamworksServerOperation::AuthenticationTicketCancelled { ticket: *ticket }
        }
        SteamworksServerCommand::BeginAuthenticationSession { user, ticket } => {
            server
                .begin_authentication_session(*user, ticket)
                .map_err(SteamworksServerError::auth_session)?;
            SteamworksServerOperation::AuthenticationSessionStarted { user: *user }
        }
        SteamworksServerCommand::EndAuthenticationSession { user } => {
            server.end_authentication_session(*user);
            SteamworksServerOperation::AuthenticationSessionEnded { user: *user }
        }
        SteamworksServerCommand::HandleIncomingPacket { data, addr } => {
            SteamworksServerOperation::IncomingPacketHandled {
                addr: *addr,
                bytes: data.len(),
                accepted: server.handle_incoming_packet(data, *addr),
            }
        }
        SteamworksServerCommand::SetProduct { product } => {
            server.set_product(product);
            SteamworksServerOperation::ProductSet {
                product: product.clone(),
            }
        }
        SteamworksServerCommand::SetGameDescription { description } => {
            server.set_game_description(description);
            SteamworksServerOperation::GameDescriptionSet {
                description: description.clone(),
            }
        }
        SteamworksServerCommand::SetGameData { data } => {
            server.set_game_data(data);
            SteamworksServerOperation::GameDataSet { data: data.clone() }
        }
        SteamworksServerCommand::SetDedicatedServer { dedicated } => {
            server.set_dedicated_server(*dedicated);
            SteamworksServerOperation::DedicatedServerSet {
                dedicated: *dedicated,
            }
        }
        SteamworksServerCommand::LogOnAnonymous => {
            server.log_on_anonymous();
            SteamworksServerOperation::AnonymousLogonSubmitted
        }
        SteamworksServerCommand::LogOn { token } => {
            server.log_on(token.as_str());
            SteamworksServerOperation::TokenLogonSubmitted
        }
        SteamworksServerCommand::SetAdvertiseServerActive { active } => {
            server.set_advertise_server_active(*active);
            SteamworksServerOperation::AdvertiseServerActiveSet { active: *active }
        }
        SteamworksServerCommand::SetModDir { mod_dir } => {
            server.set_mod_dir(mod_dir);
            SteamworksServerOperation::ModDirSet {
                mod_dir: mod_dir.clone(),
            }
        }
        SteamworksServerCommand::SetMapName { map_name } => {
            server.set_map_name(map_name);
            SteamworksServerOperation::MapNameSet {
                map_name: map_name.clone(),
            }
        }
        SteamworksServerCommand::SetServerName { server_name } => {
            server.set_server_name(server_name);
            SteamworksServerOperation::ServerNameSet {
                server_name: server_name.clone(),
            }
        }
        SteamworksServerCommand::SetMaxPlayers { count } => {
            server.set_max_players(*count);
            SteamworksServerOperation::MaxPlayersSet { count: *count }
        }
        SteamworksServerCommand::SetGameTags { tags } => {
            server.set_game_tags(tags);
            SteamworksServerOperation::GameTagsSet { tags: tags.clone() }
        }
        SteamworksServerCommand::SetKeyValue { key, value } => {
            server.set_key_value(key, value);
            SteamworksServerOperation::KeyValueSet {
                key: key.clone(),
                value: value.clone(),
            }
        }
        SteamworksServerCommand::ClearAllKeyValues => {
            server.clear_all_key_values();
            SteamworksServerOperation::AllKeyValuesCleared
        }
        SteamworksServerCommand::SetPasswordProtected { protected } => {
            server.set_password_protected(*protected);
            SteamworksServerOperation::PasswordProtectedSet {
                protected: *protected,
            }
        }
        SteamworksServerCommand::SetBotPlayerCount { count } => {
            server.set_bot_player_count(*count);
            SteamworksServerOperation::BotPlayerCountSet { count: *count }
        }
        SteamworksServerCommand::DrainOutgoingPackets => {
            SteamworksServerOperation::OutgoingPacketsDrained {
                packets: drain_outgoing_packets(server),
            }
        }
    })
}

fn drain_outgoing_packets(server: &SteamworksServer) -> Vec<SteamworksServerOutgoingPacket> {
    let mut buffer = vec![0; STEAMWORKS_SERVER_QUERY_PACKET_BUFFER_BYTES];
    let packets = RefCell::new(Vec::new());

    server.get_next_outgoing_packet(&mut buffer, |addr, data| {
        packets.borrow_mut().push(SteamworksServerOutgoingPacket {
            addr,
            data: data.to_vec(),
        });
    });

    packets.into_inner()
}

fn run_steam_server_callbacks(
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

fn validate_server_command(command: &SteamworksServerCommand) -> Result<(), SteamworksServerError> {
    match command {
        SteamworksServerCommand::BeginAuthenticationSession { ticket, .. } => {
            if ticket.is_empty() {
                Err(SteamworksServerError::EmptyTicket)
            } else {
                Ok(())
            }
        }
        SteamworksServerCommand::SetProduct { product } => {
            validate_steam_string("product", product)
        }
        SteamworksServerCommand::SetGameDescription { description } => {
            validate_steam_string("description", description)
        }
        SteamworksServerCommand::SetGameData { data } => validate_steam_string("data", data),
        SteamworksServerCommand::LogOn { token } => {
            if token.as_str().is_empty() {
                Err(SteamworksServerError::EmptyLogonToken)
            } else {
                validate_steam_string("token", token.as_str())
            }
        }
        SteamworksServerCommand::SetModDir { mod_dir } => validate_steam_string("mod_dir", mod_dir),
        SteamworksServerCommand::SetMapName { map_name } => {
            validate_steam_string("map_name", map_name)
        }
        SteamworksServerCommand::SetServerName { server_name } => {
            validate_steam_string("server_name", server_name)
        }
        SteamworksServerCommand::SetGameTags { tags } => {
            validate_steam_string("tags", tags)?;
            if tags.is_empty() || tags.len() >= 128 {
                Err(SteamworksServerError::InvalidGameTags)
            } else {
                Ok(())
            }
        }
        SteamworksServerCommand::SetKeyValue { key, value } => {
            validate_steam_string("key", key)?;
            validate_steam_string("value", value)
        }
        SteamworksServerCommand::SetMaxPlayers { count } => {
            validate_non_negative_count("count", *count)
        }
        SteamworksServerCommand::SetBotPlayerCount { count } => {
            validate_non_negative_count("count", *count)
        }
        _ => Ok(()),
    }
}

fn validate_server_command_for_state(
    command: &SteamworksServerCommand,
    state: &SteamworksServerState,
) -> Result<(), SteamworksServerError> {
    validate_server_command(command)?;

    if state.logon_submitted() {
        if matches!(
            command,
            SteamworksServerCommand::LogOnAnonymous | SteamworksServerCommand::LogOn { .. }
        ) {
            return Err(SteamworksServerError::LogonAlreadySubmitted);
        }

        if let Some(command_name) = pre_logon_only_command_name(command) {
            return Err(SteamworksServerError::command_requires_pre_logon(
                command_name,
            ));
        }
    }

    Ok(())
}

fn pre_logon_only_command_name(command: &SteamworksServerCommand) -> Option<&'static str> {
    match command {
        SteamworksServerCommand::SetProduct { .. } => Some("SetProduct"),
        SteamworksServerCommand::SetGameDescription { .. } => Some("SetGameDescription"),
        _ => None,
    }
}

fn validate_server_config(
    config: &SteamworksServerConfig,
) -> Result<(), SteamworksServerUnavailable> {
    if config.version.as_bytes().contains(&0) {
        Err(SteamworksServerUnavailable::invalid_string("version"))
    } else {
        Ok(())
    }
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksServerError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksServerError::invalid_string(field))
    } else {
        Ok(())
    }
}

fn validate_non_negative_count(
    field: &'static str,
    value: i32,
) -> Result<(), SteamworksServerError> {
    if value < 0 {
        Err(SteamworksServerError::invalid_count(field, value))
    } else {
        Ok(())
    }
}
