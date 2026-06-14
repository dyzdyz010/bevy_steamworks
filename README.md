# bevy_steamworks

Lightweight Bevy plugin for integrating the Steamworks SDK through [`steamworks`](https://crates.io/crates/steamworks).

## Version Support

| Bevy | bevy_steamworks | steamworks |
|:-----|:----------------|:-----------|
| 0.18 | 0.1             | 0.13.1     |

## Install

```toml
[dependencies]
bevy = "0.18"
bevy_steamworks = { path = "." }
```

## Basic Usage

```rust,no_run
use bevy::prelude::*;
use bevy_steamworks::prelude::*;

fn main() {
    App::new()
        .add_plugins(SteamworksPlugins::app_id(480))
        .add_plugins(DefaultPlugins)
        .run();
}
```

`480` is Valve's Spacewar sample app id. Real games should use the app id assigned by Valve.

`SteamworksPlugins` inserts `SteamworksClient` as a Bevy resource, automatically runs Steam callbacks in `SteamworksSystem::RunCallbacks` during Bevy's `First` schedule, and installs every default client-side high-level feature plugin: apps, friends, input, matchmaking, server browser queries, legacy P2P networking, networking messages, networking sockets, networking utils, Remote Play, Remote Storage, screenshots, stats, timeline, UGC, user, and utils.

For module settings, use Bevy's plugin group customization APIs:

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
App::new()
    .add_plugins(
        SteamworksPlugins::app_id(480)
            .set(SteamworksStatsPlugin::new().auto_store(false))
            .disable::<SteamworksNetworkingPlugin>(),
    );
```

For lower-level control, use `SteamworksPlugin` for the client lifecycle and add selected feature plugins yourself. `SteamworksClientPlugins` installs the default client-side feature plugin group without initializing Steamworks; use it with `SteamworksPlugin` when you want the full feature set but need to configure the core plugin separately. Dedicated server builds should use `SteamworksServerPlugin` explicitly.

Most upstream `steamworks` types are re-exported at the crate root, so app code can use common items directly:

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn friends(steam: Res<SteamworksClient>) {
    for friend in steam.friends().get_friends(FriendFlags::IMMEDIATE) {
        info!("{} ({:?})", friend.name(), friend.state());
    }
}
```

## Development Without Steam

By default, initialization failures panic so games do not silently run without Steam. For local development or CI, opt into explicit degraded mode:

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
App::new()
    .add_plugins(SteamworksPlugins::app_id(480).log_and_continue())
    .add_systems(Startup, |error: Option<Res<SteamworksUnavailable>>| {
        if let Some(error) = error {
            warn!("Steamworks unavailable: {}", &*error);
        }
    });
```

In this mode the plugin writes a structured `tracing` error and inserts `SteamworksUnavailable`.

## Reading Typed Callbacks

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn steam_callbacks(mut callbacks: MessageReader<SteamworksEvent>) {
    for event in callbacks.read() {
        match event {
            SteamworksEvent::GameOverlayActivated(event) => {
                info!("Steam overlay active: {}", event.active);
            }
            SteamworksEvent::UserStatsReceived(event) => {
                info!("user stats received: {event:?}");
            }
            other => info!("{other:?}"),
        }
    }
}
```

`SteamworksEvent` has typed variants for every callback covered by upstream `steamworks::CallbackResult`, and can be converted back into `CallbackResult` if a lower-level handler needs it.

You can also register typed callbacks through the underlying `steamworks::Client`; keep the returned handles alive with `SteamworksCallbackRegistry`.

## Dedicated Game Servers

`SteamworksServerPlugin` initializes the upstream `steamworks::Server`, inserts `SteamworksServer` as a Bevy resource, registers `SteamworksServerCommand` / `SteamworksServerResult`, and pumps Steam Game Server callbacks into the shared `SteamworksEvent` message stream. Dedicated server initialization is separate from `SteamworksPlugin`; use one lifecycle for the process unless you have a specific reason to initialize both.

```rust,no_run
# use std::net::Ipv4Addr;
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn configure_server(mut server: MessageWriter<SteamworksServerCommand>) {
    server.write(SteamworksServerCommand::set_product("480"));
    server.write(SteamworksServerCommand::set_game_description("Spacewar"));
    server.write(SteamworksServerCommand::set_dedicated_server(true));
    server.write(SteamworksServerCommand::set_server_name("Spacewar Arena"));
    server.write(SteamworksServerCommand::set_max_players(16));
    server.write(SteamworksServerCommand::LogOnAnonymous);
    server.write(SteamworksServerCommand::set_advertise_server_active(true));
}

fn read_server(mut results: MessageReader<SteamworksServerResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn read_server_callbacks(mut events: MessageReader<SteamworksEvent>) {
    for event in events.read() {
        if let SteamworksEvent::GSClientApprove(event) = event {
            info!("approved client: {event:?}");
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            SteamworksServerPlugin::new(SteamworksServerConfig::new(
                Ipv4Addr::UNSPECIFIED,
                27015,
                27016,
                steamworks::ServerMode::Authentication,
                env!("CARGO_PKG_VERSION"),
            ))
            .log_and_continue(),
        )
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, configure_server)
        .add_systems(Update, (read_server, read_server_callbacks))
        .run();
}
```

The server command layer covers server identity reads, anonymous or token logon, metadata, advertisement flags, auth tickets, remote auth sessions, key/value rules, and shared-query incoming/outgoing packet handling. Login tokens use `SteamworksServerLoginToken`, whose `Debug` output is redacted so command tracing does not leak secrets. `SteamworksServerCommand::DrainOutgoingPackets` returns `SteamworksServerOutgoingPacket` values that the app should send through its game server socket; it mirrors upstream's drain-all behavior for packets currently queued by Steam. `SteamworksServerState` caches bounded snapshots for latest auth/session activity, metadata, incoming packet context, outgoing packet drains, and callback results. Packet and auth-ticket `Debug` output reports byte lengths instead of raw bytes. The layer validates strings, game tag lengths, and documented pre-logon-only metadata before calling upstream `steamworks`, so common C string conversion panics and logon-order mistakes become `SteamworksServerError` values. The server plugin also registers `SteamworksServerCallbackRegistry` for lower-level typed server callbacks and mirrors auth ticket, validation, connection, `GSClientApprove`, `GSClientDeny`, `GSClientKick`, and `GSClientGroupStatus` callbacks through both `SteamworksEvent` and `SteamworksServerResult`. Use the `SteamworksServer` resource directly for upstream safe APIs not yet wrapped by commands, such as game-server UGC, utils, networking, networking sockets, and apps accessors.

Run the dedicated server example with:

```powershell
cargo run --example server
```

## Friends, Rich Presence, and Overlay

`SteamworksFriendsPlugin` adds a Bevy-native command/result layer for Steam persona, friends, Rich Presence, overlay, and invite workflows.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_social(mut friends: MessageWriter<SteamworksFriendsCommand>) {
    friends.write(SteamworksFriendsCommand::GetPersonaName);
    friends.write(SteamworksFriendsCommand::list_friends(FriendFlags::IMMEDIATE));
    friends.write(SteamworksFriendsCommand::set_rich_presence("status", "In Match"));
}

fn read_social(mut results: MessageReader<SteamworksFriendsResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksFriendsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_social)
        .add_systems(Update, read_social)
        .run();
}
```

Commands validate strings before calling upstream `steamworks` methods, so interior NUL bytes are reported as `SteamworksFriendsError::InvalidString` instead of panicking inside a Steam C string conversion. Friend list results use snapshot types such as `SteamworksFriendInfo`, which are safe to store in ECS resources or messages. `SteamworksFriendsState` keeps the latest friend list plus a merged per-user cache; systems can query `friend`, `coplay_friend`, `friend_rich_presence`, `friend_avatar`, `has_friend`, and the latest overlay/invite actions without retaining result history. Rich Presence and avatar accessors use an outer `Option` for unread data and an inner `Option` for a completed read where Steam had no value or image available yet. Overlay activation, persona state changes, lobby join requests, and Rich Presence join requests are still available through `SteamworksEvent`, and are also mirrored as `SteamworksFriendsResult` messages for module-local systems.

Run the social example with:

```powershell
cargo run --example social
$env:BEVY_STEAMWORKS_RICH_PRESENCE_STATUS = "Testing bevy_steamworks"
cargo run --example social
```

## Matchmaking and Lobbies

`SteamworksMatchmakingPlugin` adds a Bevy-native command/result layer for common lobby workflows: lobby search, create/join/leave, metadata, members, joinability, lobby chat, and lobby game server data.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_lobbies(mut matchmaking: MessageWriter<SteamworksMatchmakingCommand>) {
    let filter = SteamworksLobbyListFilter::new()
        .with_distance(DistanceFilter::Default)
        .with_max_results(10);

    matchmaking.write(SteamworksMatchmakingCommand::request_lobby_list(filter));
    matchmaking.write(SteamworksMatchmakingCommand::create_lobby(LobbyType::Private, 4));
}

fn read_matchmaking(mut results: MessageReader<SteamworksMatchmakingResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksMatchmakingPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_lobbies)
        .add_systems(Update, read_matchmaking)
        .run();
}
```

Async calls such as lobby list requests, lobby creation, and lobby joins first emit a submitted operation, then later emit the Steam call result after `SteamworksSystem::RunCallbacks` pumps Steam callbacks. These submitted/completed operations include a plugin-assigned `request_id` so identical in-flight requests can be correlated. Steam lobby callbacks such as `LobbyCreated`, `LobbyEnter`, `LobbyChatMsg`, `LobbyChatUpdate`, and `LobbyDataUpdate` arrive through both `SteamworksEvent` and `SteamworksMatchmakingOperation::*Received` snapshots.

`SteamworksMatchmakingOperation::LobbyChatMessageReceived` carries the lobby, sender, entry type, and chat entry ID, but it does not copy chat bytes. Steam's chat entry ID is callback-timing-sensitive, so register a lower-level callback through `SteamworksCallbackRegistry` if you need to copy lobby chat bytes immediately and reliably.

`SteamworksMatchmakingState` caches bounded lobby snapshots for latest request contexts, lobby list results, created/joined/left lobbies, metadata reads and mutations, member reads, joinability changes, chat sends and entry reads, lobby game-server reads, and lobby callbacks. Its joined lobby list tracks joins observed by this plugin and explicit leave commands; it is not a strong-consistency source for remote kicks, disconnects, or lobby shutdowns. Query data and chat entry bytes are kept only as last snapshots.

Commands validate lobby keys, strings, lobby size, and chat message size before calling upstream `steamworks` methods, so common invalid inputs become structured `SteamworksMatchmakingError` values instead of panicking in the Steam API wrapper.

Run the matchmaking example with:

```powershell
cargo run --example matchmaking
$env:BEVY_STEAMWORKS_CREATE_PRIVATE_LOBBY = "1"
cargo run --example matchmaking
```

## Matchmaking Servers

`SteamworksMatchmakingServersPlugin` adds a Bevy-native command/result layer for Steam's server browser APIs: LAN, Internet, favorites, history, and friends server lists. The plugin owns upstream request handles and exposes stable `SteamworksServerListRequestId` values for refresh, count/details reads, refreshing checks, and release.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_servers(mut servers: MessageWriter<SteamworksMatchmakingServersCommand>) {
    let filters = SteamworksServerListFilters::new().with("map", "arena");
    servers.write(SteamworksMatchmakingServersCommand::request_internet_server_list(
        480,
        filters,
    ));
}

fn read_servers(
    mut results: MessageReader<SteamworksMatchmakingServersResult>,
    mut servers: MessageWriter<SteamworksMatchmakingServersCommand>,
) {
    for result in results.read() {
        if let SteamworksMatchmakingServersResult::Ok(
            SteamworksMatchmakingServersOperation::ServerListRefreshCompleted { request, .. },
        ) = result
        {
            servers.write(SteamworksMatchmakingServersCommand::get_server_list_count(*request));
            servers.write(SteamworksMatchmakingServersCommand::release_server_list(*request));
        }
    }
}
```

Server-list callbacks are converted into owned Bevy result messages: `ServerResponded`, `ServerFailedToRespond`, and `ServerListRefreshCompleted`. Server snapshots use `SteamworksGameServerItem`, which can be stored safely in ECS. LAN requests do not accept filters; non-LAN simple keyed filter names and values are validated before calling upstream Steamworks. The upstream wrapper models filters as a map, so repeated or order-sensitive boolean filter clauses are not represented by `SteamworksServerListFilters`. Release can fail while the upstream request is still refreshing, in which case the request remains owned by the plugin and can be released later.

`SteamworksMatchmakingServersState` caches bounded server-browser snapshots: active request count, latest submitted/released server-list request, latest refresh and single-server refresh submissions, latest count/refreshing reads, latest server response/failure/details contexts, latest server snapshot, and refresh completion counters. It keeps counters and last snapshots instead of retaining unbounded server or callback history.

Run the server browser example with:

```powershell
cargo run --example matchmaking_servers
$env:BEVY_STEAMWORKS_SERVER_LIST = "internet"
$env:BEVY_STEAMWORKS_SERVER_FILTER_KEY = "map"
$env:BEVY_STEAMWORKS_SERVER_FILTER_VALUE = "arena"
cargo run --example matchmaking_servers
```

## User Identity and Authentication

`SteamworksUserPlugin` adds command/result messages for current-user identity, Steam server connection state, auth session tickets, Web API auth tickets, remote ticket validation sessions, license checks for authenticated users, and microtransaction authorization callbacks.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_user(mut user: MessageWriter<SteamworksUserCommand>) {
    user.write(SteamworksUserCommand::GetCurrentUserInfo);
    user.write(SteamworksUserCommand::IsLoggedOn);
}

fn read_user(mut results: MessageReader<SteamworksUserResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksUserPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_user)
        .add_systems(Update, read_user)
        .run();
}
```

`SteamworksUserCommand::GetAuthenticationSessionTicket` immediately returns a ticket handle and bytes in `SteamworksUserResult`, then final Steam confirmation arrives through both `SteamworksEvent::AuthSessionTicketResponse` and `SteamworksUserOperation::AuthenticationSessionTicketResponse`. `SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi` returns the handle first; the Web API ticket bytes arrive through `SteamworksEvent::TicketForWebApiResponse` and `SteamworksUserOperation::WebApiAuthenticationTicketReceived`. Remote ticket validation callbacks are also mirrored as `SteamworksUserOperation::AuthenticationTicketValidationReceived` with an owned, comparable validation result.

Steam server connection callbacks are mirrored as `SteamworksUserOperation::SteamServerConnectionEventReceived` and update `SteamworksUserState::steam_server_connected()`. `SteamworksEvent::MicroTxnAuthorizationResponse` is mirrored as `SteamworksUserOperation::MicroTxnAuthorizationResponseReceived` with app ID, order ID, and authorization state.

Call `SteamworksUserCommand::CancelAuthenticationTicket` when a locally issued ticket is no longer needed, and `SteamworksUserCommand::EndAuthenticationSession` when a remote authenticated session ends. The command layer tracks issued ticket handles and sessions started through its own commands in `SteamworksUserState`; validation failure callbacks prune matching started sessions without creating new ones from unrelated global events.

`SteamworksUserState` also caches bounded query snapshots for the latest Steam ID and level reads, latest issued auth session ticket, latest Web API ticket request, latest cancelled ticket, latest started/ended remote authentication session, latest app-license check, and counters for those command-layer events. It keeps active ticket/session sets for lifecycle management, but only last snapshots for ticket bytes and license checks. Treat cached ticket bytes as credential material: cancel tickets promptly and avoid dumping state snapshots to logs.

Run the user/auth example with:

```powershell
cargo run --example user
$env:BEVY_STEAMWORKS_AUTH_TICKET = "1"
cargo run --example user
$env:BEVY_STEAMWORKS_WEBAPI_IDENTITY = "my-service"
cargo run --example user
```

## Utilities and Overlay Helpers

`SteamworksUtilsPlugin` adds command/result messages for Steam utility queries and lightweight overlay helpers: app id, IP country, Steam UI language, server real time, overlay availability, Big Picture mode, Steam Deck detection, overlay notification position, and gamepad text input dismissal callbacks.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_utils(mut utils: MessageWriter<SteamworksUtilsCommand>) {
    utils.write(SteamworksUtilsCommand::GetCurrentInfo);
    utils.write(SteamworksUtilsCommand::IsOverlayEnabled);
    utils.write(SteamworksUtilsCommand::set_overlay_notification_position(
        SteamworksNotificationPosition::BottomRight,
    ));
}

fn read_utils(mut results: MessageReader<SteamworksUtilsResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksUtilsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_utils)
        .add_systems(Update, read_utils)
        .run();
}
```

`SteamworksUtilsState` caches both `GetCurrentInfo` snapshots and the latest individual app id, IP country, overlay availability, UI language, server time, Big Picture mode, Steam Deck, and submitted overlay notification position. Gamepad text input dismissal callbacks arrive through both `SteamworksEvent` and `SteamworksUtilsOperation::{GamepadTextInputDismissed, FloatingGamepadTextInputDismissed}`. `GamepadTextInputDismissed` includes Steam's submitted text length when the user submitted text, but the text itself must still be read inside Steam's original callback timing through the raw `steamworks::Utils` helper. Be aware that the upstream text input helpers register their own typed callbacks, so avoid also registering competing callbacks for the same dismissal types through `SteamworksCallbackRegistry`.

Run the utils example with:

```powershell
cargo run --example utils
$env:BEVY_STEAMWORKS_OVERLAY_BOTTOM_RIGHT = "1"
cargo run --example utils
```

## Steam Input

`SteamworksInputPlugin` adds command/result messages for Steam Input initialization, controller listing, action set/action handle lookup, action data reads, motion data, action origin presentation strings, and the binding panel.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_input(mut input: MessageWriter<SteamworksInputCommand>) {
    input.write(SteamworksInputCommand::init(false));
    input.write(SteamworksInputCommand::RunFrame);
    input.write(SteamworksInputCommand::ListControllers);
    input.write(SteamworksInputCommand::get_action_set_handle("gameplay"));
    input.write(SteamworksInputCommand::get_digital_action_handle("jump"));
}

fn read_input(mut results: MessageReader<SteamworksInputResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksInputPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_input)
        .add_systems(Update, read_input)
        .run();
}
```

The plugin uses stable wrapper types such as `SteamworksInputHandle`, `SteamworksInputActionSetHandle`, `SteamworksInputDigitalActionHandle`, and `SteamworksInputAnalogActionHandle` instead of exposing raw `steamworks_sys` types. String inputs are validated before calling upstream `steamworks`, so interior NUL bytes become `SteamworksInputError::InvalidString` instead of panicking in a C string conversion. Steam Input handle lookups return `SteamworksInputError::InvalidHandleReturned` when Steam returns the zero invalid handle.

`SteamworksInputState` caches known controller snapshots, named action set/digital/analog handles, the latest action set activation, digital/analog action data, digital/analog origin snapshots, motion data, binding panel controller, and successful `RunFrame` count. Systems can query the cached state with accessors such as `controller`, `action_set_handle`, `digital_action_handle`, `analog_action_handle`, `last_digital_action_origins`, and `last_analog_action_origins`. `Init`, `Shutdown`, and `SetActionManifestFilePath` clear stale action data so reads from a previous manifest are not silently reused.

Steam Input is synchronized by `SteamAPI_RunCallbacks` when initialized with `SteamworksInputCommand::init(false)`, which matches the default callback pump in `SteamworksSystem::RunCallbacks`. Input commands run after `RunCallbacks`; if you initialize and read input in the same frame, send `SteamworksInputCommand::RunFrame` between `Init` and the read commands, or wait until a later frame. If you initialize with `init(true)`, explicitly send `RunFrame` before reads each frame.

Run the Steam Input example with:

```powershell
cargo run --example input
$env:BEVY_STEAMWORKS_INPUT_MANIFEST = "C:\path\to\game_actions.vdf"
$env:BEVY_STEAMWORKS_INPUT_ACTION_SET = "gameplay"
$env:BEVY_STEAMWORKS_INPUT_DIGITAL_ACTION = "jump"
cargo run --example input
```

## Legacy P2P Networking

`SteamworksNetworkingPlugin` adds command/result messages for Steam's older P2P networking API: accept and close P2P sessions, send packets, poll packet availability, read owned packet snapshots, inspect session state, and mirror `P2PSessionRequest` / `P2PSessionConnectFail` callbacks as Bevy results.

New projects should prefer `SteamworksNetworkingMessagesPlugin`; this legacy layer exists for older Steam networking flows and migration work.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn send_legacy_packet(mut networking: MessageWriter<SteamworksNetworkingCommand>) {
    networking.write(SteamworksNetworkingCommand::send_p2p_packet(
        SteamId::from_raw(76561198000000000),
        SteamworksP2pSendType::Reliable,
        0,
        b"ping".to_vec(),
    ));
}

fn read_legacy_packets(mut networking: MessageWriter<SteamworksNetworkingCommand>) {
    networking.write(SteamworksNetworkingCommand::get_available_packet_size(0));
    networking.write(SteamworksNetworkingCommand::read_p2p_packet(
        0,
        STEAMWORKS_P2P_MAX_READ_PACKET_BYTES,
    ));
}

fn read_legacy_results(
    mut results: MessageReader<SteamworksNetworkingResult>,
    mut networking: MessageWriter<SteamworksNetworkingCommand>,
) {
    for result in results.read() {
        info!("{result:?}");
        if let SteamworksNetworkingResult::Ok(
            SteamworksNetworkingOperation::SessionRequestReceived { remote },
        ) = result
        {
            networking.write(SteamworksNetworkingCommand::accept_p2p_session(*remote));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksNetworkingPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, send_legacy_packet)
        .add_systems(Update, (read_legacy_packets, read_legacy_results))
        .run();
}
```

The command layer validates Steam IDs, channel ranges, send-size limits, and per-frame receive buffer sizes before calling upstream `steamworks`. `ReadP2pPacket` checks the queued packet size before reading, so too-small buffers return `SteamworksNetworkingError::PacketExceedsReadBuffer` instead of silently truncating payloads. Received packets are copied into `SteamworksP2pPacket { data: Vec<u8>, .. }`, so they are safe to store in ECS resources.

`SteamworksNetworkingState` caches the latest accepted/closed session remote, latest session state read, latest packet availability, latest sent packet summary, latest received packet, empty-read count/channel, and the latest session request/failure callback snapshots. It keeps counters instead of unbounded packet or callback history, so gameplay systems can poll state resources without growing memory over time.

`SteamworksNetworkingCommand::AcceptP2pSession` should be sent in response to a `SessionRequestReceived` result, matching Steam's `P2PSessionRequest_t` timing requirement.

Run the legacy P2P example with:

```powershell
cargo run --example networking
$env:BEVY_STEAMWORKS_P2P_PEER = "76561198000000000"
$env:BEVY_STEAMWORKS_P2P_MESSAGE = "hello"
cargo run --example networking
```

## Networking Messages

`SteamworksNetworkingMessagesPlugin` adds command/result messages for Steam's UDP-like P2P message API: send payloads to Steam IDs, IP endpoints, local host, or prebuilt `NetworkingIdentity` values; receive owned message snapshots by channel; read session connection state; and handle session request/failure callbacks.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn send_ping(mut messages: MessageWriter<SteamworksNetworkingMessagesCommand>) {
    messages.write(SteamworksNetworkingMessagesCommand::send_message_to_steam_id(
        SteamId::from_raw(76561198000000000),
        steamworks::networking_types::SendFlags::RELIABLE_NO_NAGLE,
        0,
        b"ping".to_vec(),
    ));
}

fn receive_messages(mut messages: MessageWriter<SteamworksNetworkingMessagesCommand>) {
    messages.write(SteamworksNetworkingMessagesCommand::receive_messages(0, 32));
}

fn read_networking_results(mut results: MessageReader<SteamworksNetworkingMessagesResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksNetworkingMessagesPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, send_ping)
        .add_systems(Update, (receive_messages, read_networking_results))
        .run();
}
```

Session requests are accepted by default because the upstream safe API only allows accepting or rejecting while the Steam callback is running; it cannot defer the decision to a later ECS frame. Use `SteamworksNetworkingMessagesPlugin::new().auto_accept_session_requests(false)` or `SteamworksNetworkingMessagesCommand::set_auto_accept_session_requests(false)` to reject future incoming requests. The policy command is pre-read before `SteamworksSystem::RunCallbacks`, then processed normally later so you still receive a result message. Every session request and session failure is emitted as a `SteamworksNetworkingMessagesResult`.

Received messages are copied into `SteamworksNetworkingMessage { data: Vec<u8>, .. }`, so they can safely be stored in Bevy resources after Steam releases the original message handle. Channel values are validated before the upstream API call to avoid signed integer wrapping, and one receive command is capped by `STEAMWORKS_NETWORKING_MESSAGES_MAX_BATCH_SIZE` to prevent unbounded frame-loop allocation.

Run the Networking Messages example with:

```powershell
cargo run --example networking_messages
$env:BEVY_STEAMWORKS_NETWORKING_PEER = "76561198000000000"
$env:BEVY_STEAMWORKS_NETWORKING_MESSAGE = "hello"
cargo run --example networking_messages
```

## Networking Utils and Relay Status

`SteamworksNetworkingUtilsPlugin` adds command/result messages for Steam Datagram Relay diagnostics: initialize relay network access early, read summary relay availability, read detailed relay status, and receive relay status callbacks as Bevy messages.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_relay_status(mut utils: MessageWriter<SteamworksNetworkingUtilsCommand>) {
    utils.write(SteamworksNetworkingUtilsCommand::init_relay_network_access());
    utils.write(SteamworksNetworkingUtilsCommand::GetDetailedRelayNetworkStatus);
}

fn read_relay_status(mut results: MessageReader<SteamworksNetworkingUtilsResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksNetworkingUtilsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_relay_status)
        .add_systems(Update, read_relay_status)
        .run();
}
```

Relay status callbacks arrive through the main `SteamworksEvent::RelayNetworkStatusCallback` stream. This plugin observes those events after `SteamworksSystem::RunCallbacks`, reads the current detailed relay status, and emits an owned `SteamworksNetworkingUtilsResult::Ok` snapshot. Detailed status is copied into `SteamworksRelayNetworkStatus`, including Steam's diagnostic debug string, so the snapshot can be stored in ECS state safely.

This layer intentionally leaves `NetworkingUtils::allocate_message` to the future `networking_sockets` command layer, because allocated message handles are part of low-level socket send workflows rather than relay diagnostics.

Run the Networking Utils example with:

```powershell
cargo run --example networking_utils
$env:BEVY_STEAMWORKS_RELAY_INIT = "0"
cargo run --example networking_utils
```

## Networking Sockets

`SteamworksNetworkingSocketsPlugin` adds command/result messages for Steam's modern connection-oriented Networking Sockets API. It can initialize networking authentication, create IP or P2P listen sockets, connect to IP or Steam identity peers, poll listen-socket and connection events, send and receive owned message snapshots, create poll groups, configure connection lanes, read connection status, set user data, flush, and close handles.

The plugin owns upstream `ListenSocket` and `NetConnection` handles in a private resource and exposes stable IDs such as `SteamworksListenSocketId` and `SteamworksNetworkingSocketsConnectionId`. This prevents accidental handle drops from closing sockets outside the command layer.

Accepted listen-socket connections are tracked against their parent listen socket. A listen-socket disconnect event removes the matching connection ID when it can be identified unambiguously, and `CloseListenSocket` removes accepted child connections before dropping the listen socket. Independent connection polls report `connection_removed: true` when a terminal connection event caused the plugin to free the handle.

Poll group messages are returned as `SteamworksNetworkingSocketsPollGroupMessage`. The upstream safe wrapper does not expose the raw connection handle carried by those messages, so the poll-group snapshot includes Steam's `connection_user_data` instead of a plugin connection ID. If you need to map poll-group messages back to game state, set unique connection user data through `SteamworksNetworkingSocketsCommand::set_connection_user_data`.

`SteamworksNetworkingSocketsState` caches bounded snapshots for the latest created and closed handles, polled event batches, connection info, realtime status, sent message context, received message batches, flushes, poll-group assignments, lane configuration, and user-data updates. It keeps only the most recent received message batches; message and command `Debug` output reports payload lengths instead of raw bytes so tracing does not dump packet contents.

```rust,no_run
# use std::net::{Ipv4Addr, SocketAddr};
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn open_socket(mut sockets: MessageWriter<SteamworksNetworkingSocketsCommand>) {
    sockets.write(SteamworksNetworkingSocketsCommand::InitAuthentication);
    sockets.write(SteamworksNetworkingSocketsCommand::create_listen_socket_ip(
        SocketAddr::from((Ipv4Addr::UNSPECIFIED, 27015)),
    ));
}

fn poll_socket(
    listen_socket: Res<MyListenSocket>,
    mut sockets: MessageWriter<SteamworksNetworkingSocketsCommand>,
) {
    sockets.write(SteamworksNetworkingSocketsCommand::poll_listen_socket_events(
        listen_socket.0,
        32,
        SteamworksConnectionRequestPolicy::Accept,
    ));
}

#[derive(Resource)]
struct MyListenSocket(SteamworksListenSocketId);

fn read_socket_results(mut results: MessageReader<SteamworksNetworkingSocketsResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksNetworkingSocketsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, open_socket)
        .add_systems(Update, read_socket_results)
        .run();
}
```

Listen socket connection requests must be answered immediately. `PollListenSocketEvents` therefore takes a `SteamworksConnectionRequestPolicy` and accepts or rejects each incoming request in the same frame instead of exposing a cross-frame pending request handle.

This command layer covers the safe handle-oriented Networking Sockets workflow. Low-level configuration entries and zero-copy allocated messages remain accessible through `SteamworksClient::networking_sockets()` for specialized engines and can be promoted into typed commands in later layers.

Run the Networking Sockets example with:

```powershell
cargo run --example networking_sockets
$env:BEVY_STEAMWORKS_SOCKETS_LISTEN_IP = "0.0.0.0:27015"
$env:BEVY_STEAMWORKS_SOCKETS_ACCEPT = "1"
cargo run --example networking_sockets
$env:BEVY_STEAMWORKS_SOCKETS_CONNECT_IP = "127.0.0.1:27015"
$env:BEVY_STEAMWORKS_SOCKETS_MESSAGE = "hello"
cargo run --example networking_sockets
$env:BEVY_STEAMWORKS_SOCKETS_POLL_GROUP = "1"
cargo run --example networking_sockets
```

## Screenshots

`SteamworksScreenshotsPlugin` adds command/result messages for Steam screenshot workflows: hook screenshot hotkeys, read hook state, trigger a screenshot, and add an existing image file to the user's Steam screenshot library.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_screenshots(mut screenshots: MessageWriter<SteamworksScreenshotsCommand>) {
    screenshots.write(SteamworksScreenshotsCommand::IsScreenshotsHooked);
}

fn read_screenshots(mut results: MessageReader<SteamworksScreenshotsResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksScreenshotsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_screenshots)
        .add_systems(Update, read_screenshots)
        .run();
}
```

`SteamworksScreenshotsCommand::AddScreenshotToLibrary` returns a screenshot handle when Steam accepts the request. Final save confirmation arrives later through both `SteamworksEvent::ScreenshotReady` and `SteamworksScreenshotsOperation::ScreenshotReady`. Width and height are validated before calling upstream `steamworks`, and path/save failures are reported as `SteamworksScreenshotsError::LibraryAddFailed`.

`SteamworksScreenshotsState` caches the latest hook state, successful trigger count, screenshot request callback count, accepted library submission metadata, and the latest screenshot ready callback. Systems can keep using `added_screenshots` for handle-only compatibility or query `submitted_screenshots`, `submitted_screenshot`, and `last_submitted_screenshot` when they need the submitted path and dimensions.

Only call `SteamworksScreenshotsCommand::hook_screenshots(true)` if your game will handle `SteamworksScreenshotsOperation::ScreenshotRequested` or `SteamworksEvent::ScreenshotRequested` by capturing an image and submitting it to Steam; once hooked, Steam no longer handles the screenshot hotkey for you. `AddScreenshotToLibrary` canonicalizes local file paths through the upstream wrapper before submitting, so keep it low-frequency and avoid slow network paths in frame-critical flows.

Run the screenshots example with:

```powershell
cargo run --example screenshots
$env:BEVY_STEAMWORKS_HOOK_SCREENSHOTS = "1"
$env:BEVY_STEAMWORKS_TRIGGER_SCREENSHOT = "1"
cargo run --example screenshots
$env:BEVY_STEAMWORKS_SCREENSHOT_FILE = "C:\absolute\path\to\screenshot.png"
$env:BEVY_STEAMWORKS_SCREENSHOT_WIDTH = "1920"
$env:BEVY_STEAMWORKS_SCREENSHOT_HEIGHT = "1080"
cargo run --example screenshots
```

## Remote Storage

`SteamworksRemoteStoragePlugin` adds command/result messages for Steam Cloud availability, file listing, file metadata, delete/forget, sync platforms, and asynchronous file sharing.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_storage(mut storage: MessageWriter<SteamworksRemoteStorageCommand>) {
    storage.write(SteamworksRemoteStorageCommand::GetCloudInfo);
    storage.write(SteamworksRemoteStorageCommand::ListFiles);
    storage.write(SteamworksRemoteStorageCommand::get_file_info("save.dat"));
}

fn read_storage(mut results: MessageReader<SteamworksRemoteStorageResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksRemoteStoragePlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_storage)
        .add_systems(Update, read_storage)
        .run();
}
```

This command layer intentionally does not wrap upstream `SteamFileReader` and `SteamFileWriter`, because the current upstream reader busy-waits for async file reads and file payload IO should not run in a Bevy frame loop. Use `SteamworksClient::remote_storage()` directly from your own background/file-IO layer for payload reads and writes.

`SteamworksRemoteStorageCommand::ShareFile` emits `FileShareRequested` immediately with a plugin-assigned `request_id`, then emits `FileShared` or an async error after `SteamworksSystem::RunCallbacks` pumps the Steam call result. File names are validated before calling upstream `steamworks`, so interior NUL bytes become `SteamworksRemoteStorageError::InvalidString` instead of panicking in a C string conversion.

Run the Remote Storage example with:

```powershell
cargo run --example remote_storage
$env:BEVY_STEAMWORKS_REMOTE_STORAGE_FILE = "save.dat"
cargo run --example remote_storage
$env:BEVY_STEAMWORKS_REMOTE_STORAGE_SHARE = "1"
cargo run --example remote_storage
```

## Workshop / UGC

`SteamworksUgcPlugin` adds command/result messages for common Steam Workshop workflows: query item details, search Workshop pages, list subscriptions, read item state/download/install info, submit downloads, subscribe/unsubscribe/delete items, create a new Workshop item, submit item updates, read update progress, and start/stop playtime tracking.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_ugc(mut ugc: MessageWriter<SteamworksUgcCommand>) {
    ugc.write(SteamworksUgcCommand::list_subscribed_items(false));
    ugc.write(SteamworksUgcCommand::query(
        SteamworksUgcQuery::item(PublishedFileId(123456)).with_options(
            SteamworksUgcQueryOptions::new()
                .with_metadata(true)
                .with_key_value_tags(true)
            .with_statistic(UGCStatisticType::Subscriptions),
        ),
    ));
    ugc.write(SteamworksUgcCommand::submit_item_update(
        AppId(480),
        PublishedFileId(123456),
        SteamworksUgcItemUpdate::new()
            .with_title("Updated title")
            .with_content_path("workshop_content")
            .with_preview_path("preview.png")
            .with_change_note("Updated Workshop content"),
    ));
}

fn read_ugc(mut results: MessageReader<SteamworksUgcResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksUgcPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_ugc)
        .add_systems(Update, read_ugc)
        .run();
}
```

All async UGC commands emit an immediate `*Requested` or `*Submitted` operation with a plugin-assigned `request_id`, then emit the completion or async error after `SteamworksSystem::RunCallbacks` pumps Steam call results. Query results are copied into owned `SteamworksUgcQueryResults` snapshots, so callers do not need to hold upstream query handles or lifetimes.

String query and item update options are validated before calling upstream `steamworks`, so interior NUL bytes become `SteamworksUgcError::InvalidString` instead of panicking in a C string conversion. Item update paths are canonicalized before submission, so paths that cannot be resolved become structured `SteamworksUgcError::InvalidPath` errors. Submitted item updates retain an internal progress watch handle until the Steam call result arrives; read it with `SteamworksUgcCommand::get_item_update_progress(request_id)`.

`DownloadItem` only confirms that Steam accepted the download request; final completion arrives later through both `SteamworksEvent::DownloadItemResult` and `SteamworksUgcOperation::DownloadItemResultReceived`.

Run the UGC example with:

```powershell
cargo run --example ugc
$env:BEVY_STEAMWORKS_UGC_ITEM = "123456"
cargo run --example ugc
$env:BEVY_STEAMWORKS_UGC_SEARCH = "levels"
cargo run --example ugc
$env:BEVY_STEAMWORKS_UGC_DOWNLOAD = "1"
cargo run --example ugc
$env:BEVY_STEAMWORKS_UGC_UPDATE = "1"
$env:BEVY_STEAMWORKS_UGC_UPDATE_TITLE = "Updated title"
$env:BEVY_STEAMWORKS_UGC_UPDATE_CONTENT_PATH = "C:\path\to\workshop_content"
$env:BEVY_STEAMWORKS_UGC_UPDATE_PREVIEW_PATH = "C:\path\to\preview.png"
$env:BEVY_STEAMWORKS_UGC_UPDATE_CHANGE_NOTE = "Updated Workshop content"
cargo run --example ugc
```

## Remote Play

`SteamworksRemotePlayPlugin` adds command/result messages for Steam Remote Play sessions and Remote Play Together invites.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_remote_play(mut remote_play: MessageWriter<SteamworksRemotePlayCommand>) {
    remote_play.write(SteamworksRemotePlayCommand::ListSessions);
}

fn read_remote_play(mut results: MessageReader<SteamworksRemotePlayResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksRemotePlayPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_remote_play)
        .add_systems(Update, read_remote_play)
        .run();
}
```

`SteamworksRemotePlayCommand::ListSessions` returns session snapshots with user, client name, form factor, and resolution. The upstream bulk listing API does not expose session IDs, so use `SteamworksRemotePlayOperation::SessionConnected` to capture a `RemotePlaySessionId`, then call `SteamworksRemotePlayCommand::GetSession` for ID-based session reads. Remote Play connect/disconnect callbacks are still available through `SteamworksEvent`, and are mirrored as `SteamworksRemotePlayResult` messages for module-local systems.

`SteamworksRemotePlayState` caches the latest bulk session list, ID-based known session snapshots, callback-observed connected session IDs, the latest submitted Remote Play Together invite, and successful invite count. Systems can query specific ID-based snapshots with `known_session`, check callback-observed connectivity with `is_session_observed_connected`, and inspect the last invite through `last_submitted_invite`.

The current upstream Rust wrapper exposes Remote Play Together invites through `steamworks::RemotePlaySession`, but the underlying invite result only confirms whether Steam accepted an invite for the friend. `SteamworksRemotePlayCommand::Invite` therefore treats the session ID as caller-provided context, not proof that Steam created a session-specific invite.

Run the Remote Play example with:

```powershell
cargo run --example remote_play
$env:BEVY_STEAMWORKS_REMOTE_PLAY_SESSION = "1"
$env:BEVY_STEAMWORKS_REMOTE_PLAY_FRIEND = "76561198000000000"
cargo run --example remote_play
```

## Timeline

`SteamworksTimelinePlugin` adds command/result messages for Steam Timeline game modes, state descriptions, and event markers.

```rust,no_run
# use std::time::Duration;
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_timeline(mut timeline: MessageWriter<SteamworksTimelineCommand>) {
    timeline.write(SteamworksTimelineCommand::set_game_mode(
        SteamworksTimelineGameMode::Playing,
    ));
    timeline.write(SteamworksTimelineCommand::set_state_description(
        "Boss fight",
        Duration::from_secs(3),
    ));
    timeline.write(SteamworksTimelineCommand::add_event(
        SteamworksTimelineEventInfo {
            icon: "skull".to_owned(),
            title: "Boss defeated".to_owned(),
            description: "The party won the encounter".to_owned(),
            priority: 10,
            start_offset_seconds: 0.0,
            duration: Duration::ZERO,
            clip_priority: SteamworksTimelineEventClipPriority::Featured,
        },
    ));
}

fn read_timeline(mut results: MessageReader<SteamworksTimelineResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksTimelinePlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_timeline)
        .add_systems(Update, read_timeline)
        .run();
}
```

Timeline commands validate strings before calling upstream `steamworks`, so interior NUL bytes become `SteamworksTimelineError::InvalidString` instead of panicking in a C string conversion. Event start offsets must be finite. The upstream Timeline wrapper no-ops when the Steam client API is too old for Timeline support; `SteamworksTimelineResult::Ok` means the command was accepted by this Bevy command layer and submitted to the wrapper.

Run the Timeline example with:

```powershell
cargo run --example timeline
$env:BEVY_STEAMWORKS_TIMELINE_EVENT = "1"
cargo run --example timeline
```

## App, Ownership, and Launch Parameters

`SteamworksAppsPlugin` adds command/result messages for application-level Steam checks: current app info, ownership/subscription state, DLC installation, language settings, beta branch name, build ID, install directories, and launch parameters.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn request_app_info(mut apps: MessageWriter<SteamworksAppsCommand>) {
    apps.write(SteamworksAppsCommand::GetCurrentAppInfo);
    apps.write(SteamworksAppsCommand::is_dlc_installed(123456));
    apps.write(SteamworksAppsCommand::get_launch_query_param("connect"));
}

fn read_app_info(mut results: MessageReader<SteamworksAppsResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksAppsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_app_info)
        .add_systems(Update, read_app_info)
        .run();
}
```

`SteamworksAppsCommand::GetCurrentAppInfo` combines the most commonly needed app checks into one `SteamworksCurrentAppInfo` snapshot. `SteamworksAppsState` also caches the latest individual subscription, install directory, language, beta branch, launch command line, and launch query parameter reads for systems that prefer resource lookups over retaining message history. Use `current_beta_name_result()` when you need to distinguish an unread beta branch from a completed read that reported no beta branch. Launch query keys are validated before calling upstream `steamworks`, so interior NUL bytes become `SteamworksAppsError::InvalidString` instead of panicking. If Steam delivers new URL launch parameters while the app is already running, `SteamworksEvent::NewUrlLaunchParameters` is also mirrored as `SteamworksAppsOperation::NewUrlLaunchParametersReceived`; send `GetLaunchCommandLine` or `GetLaunchQueryParam` afterwards to read the latest values.

Run the app info example with:

```powershell
cargo run --example apps
$env:BEVY_STEAMWORKS_LAUNCH_PARAM = "connect"
cargo run --example apps
```

## Achievements and Stats

`SteamworksStatsPlugin` adds a Bevy-native command/result layer for common user stats, achievement, and leaderboard workflows. It is optional; you can still call the raw `steamworks` API through `SteamworksClient`.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn unlock_win(mut stats: MessageWriter<SteamworksStatsCommand>) {
    stats.write(SteamworksStatsCommand::list_achievements(true, true));
    stats.write(SteamworksStatsCommand::unlock_achievement("ACH_WIN_ONE_GAME"));
    stats.write(SteamworksStatsCommand::get_achievement_icon("ACH_WIN_ONE_GAME"));
    stats.write(SteamworksStatsCommand::set_stat_i32("total_wins", 1));
}

fn read_stats_results(mut results: MessageReader<SteamworksStatsResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksStatsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, unlock_win)
        .add_systems(Update, read_stats_results)
        .run();
}
```

By default, `SteamworksStatsPlugin` requests stats for the current Steam user once the `SteamworksClient` resource exists. Successful stat/achievement writes are coalesced into one `store_stats()` call per frame. `SteamworksStatsResult::Ok(SteamworksStatsOperation::StatsStoreSubmitted)` only means the store request was submitted; final Steam confirmation arrives through both `SteamworksEvent::UserStatsStored` and `SteamworksStatsOperation::UserStatsStored`. Stats load callbacks are mirrored as `SteamworksStatsOperation::UserStatsReceived`, and stored achievements are mirrored as `SteamworksStatsOperation::UserAchievementStored`.

`SteamworksStatsState` caches the latest local stat and achievement reads/writes by API name. Systems can query `stat_i32`, `stat_f32`, `achievement`, `achievement_unlocked`, `achievement_unlock_time`, `achievement_display_attribute`, and `achievement_global_percent` without retaining `SteamworksStatsResult` history. Achievement catalog commands can list API names or owned `SteamworksAchievementInfo` snapshots with optional display attributes and current-user unlock state. Catalog reads are paged: `list_achievements(...)` returns the first `STEAMWORKS_ACHIEVEMENT_DEFAULT_ITEMS_PER_COMMAND` items, and `list_achievements_page(..., offset, limit)` can pull later pages up to `STEAMWORKS_ACHIEVEMENT_MAX_ITEMS_PER_COMMAND` per command. The upstream safe wrapper still enumerates achievement names internally, so use catalog reads as startup/tooling work instead of sending them every frame.

`GetAchievementIcon` emits `SteamworksAchievementIconStatus::Available(SteamworksAchievementIcon)` when Steam returns a 64x64 RGBA icon. `SteamworksAchievementIconStatus::PendingOrUnavailable` means the upstream safe wrapper did not return bytes; Steam may still be fetching the icon, the icon may be missing, or the image read may have failed. A later `UserAchievementIconFetched` callback is converted into `SteamworksStatsOperation::AchievementIconFetched` and preserves Steam's callback `icon_handle`.

After `RequestGlobalAchievementPercentages` completes with `GlobalAchievementPercentagesReceived`, `list_achievement_global_percentages()` and `list_achievement_global_percentages_page(offset, limit)` return paged `SteamworksAchievementGlobalPercentage` snapshots. These commands also enumerate achievement names through the upstream safe wrapper, so treat them as startup/tooling reads rather than every-frame work.

Aggregated global stats use `request_global_stats(history_days)` first; after `GlobalStatsReceived`, read values with `get_global_stat_i64`, `get_global_stat_f64`, `get_global_stat_history_i64`, or `get_global_stat_history_f64`. `SteamworksStatsState` caches the latest game ID, value snapshots (`SteamworksGlobalStatValue<T>`), and history snapshots (`SteamworksGlobalStatHistory<T>`) so systems can inspect the last result without retaining message history. A new global stats request clears the previous cached global stat values until fresh reads complete.

Leaderboard find and find-or-create commands are asynchronous. Successful results insert the upstream leaderboard handle into the plugin and return a `SteamworksLeaderboardId`; later info reads, score uploads, entry downloads, and forget commands use that stable ID. `SteamworksStatsState` caches bounded leaderboard snapshots for the latest find/create requests and completions, info reads, score upload requests and completions, entries download requests and completions, and the latest forgotten leaderboard ID. Global downloads use absolute rank ranges, user-relative downloads accept signed offsets around the current user, and friends downloads do not take a range. Ranged downloads are capped per command to keep frame work bounded.

For read-only tools or examples, disable automatic storage:

```rust,no_run
# use bevy_steamworks::prelude::*;
SteamworksStatsPlugin::new().auto_store(false);
```

Run the stats example with:

```powershell
cargo run --example stats
$env:BEVY_STEAMWORKS_APP_ID = "your_app_id"
$env:BEVY_STEAMWORKS_STAT_I32 = "your_stat_api_name"
$env:BEVY_STEAMWORKS_ACHIEVEMENT = "your_achievement_api_name"
$env:BEVY_STEAMWORKS_ACHIEVEMENT_ICON = "1"
$env:BEVY_STEAMWORKS_ACHIEVEMENT_CATALOG = "1"
$env:BEVY_STEAMWORKS_GLOBAL_ACHIEVEMENT_PERCENTAGES = "1"
cargo run --example stats
$env:BEVY_STEAMWORKS_LEADERBOARD = "your_leaderboard_api_name"
$env:BEVY_STEAMWORKS_LEADERBOARD_CREATE = "1"
$env:BEVY_STEAMWORKS_LEADERBOARD_SCORE = "100"
cargo run --example stats
```

The stats example defaults to Spacewar AppId `480`, but catalog and achievement names need an app id whose Steamworks schema defines the requested achievements.

## Smoke Test With Spacewar

Valve's public Spacewar app id is `480`. With Steam running locally, run:

```powershell
cargo run --example friends
```

For an automated live smoke test:

```powershell
$env:BEVY_STEAMWORKS_LIVE = "1"
cargo test --test live_spacewar -- --nocapture
```

Expected result:

- If Steam initializes, the example prints the current Steam persona and immediate friends.
- If Steam is unavailable, the example keeps running long enough to print `SteamworksUnavailable` because it uses `log_and_continue()`.
- The automated test is skipped unless `BEVY_STEAMWORKS_LIVE=1` is set, so CI can run without a Steam client.

For your own game, replace `480` with the app id assigned by Valve and ensure the Steam redistributable libraries can be found by the OS loader.

## Steam Redistributables

`steamworks-rs` dynamically loads the Steamworks runtime libraries. Put the required Steam redistributables next to your game executable or somewhere the operating system loader can find them. See the upstream `steamworks` crate documentation for platform-specific details.
