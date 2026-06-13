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
        .add_plugins(SteamworksPlugin::app_id(480))
        .add_plugins(DefaultPlugins)
        .run();
}
```

`480` is Valve's Spacewar sample app id. Real games should use the app id assigned by Valve.

The plugin inserts `SteamworksClient` as a Bevy resource and automatically runs Steam callbacks in `SteamworksSystem::RunCallbacks` during Bevy's `First` schedule.

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
    .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
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

Commands validate strings before calling upstream `steamworks` methods, so interior NUL bytes are reported as `SteamworksFriendsError::InvalidString` instead of panicking inside a Steam C string conversion. Friend list results use snapshot types such as `SteamworksFriendInfo`, which are safe to store in ECS resources or messages.

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

Async calls such as lobby list requests, lobby creation, and lobby joins first emit a submitted operation, then later emit the Steam call result after `SteamworksSystem::RunCallbacks` pumps Steam callbacks. These submitted/completed operations include a plugin-assigned `request_id` so identical in-flight requests can be correlated. Steam lobby callbacks such as `LobbyCreated`, `LobbyEnter`, `LobbyChatMsg`, and `LobbyDataUpdate` still arrive through `SteamworksEvent`.

Commands validate lobby keys, strings, lobby size, and chat message size before calling upstream `steamworks` methods, so common invalid inputs become structured `SteamworksMatchmakingError` values instead of panicking in the Steam API wrapper.

Run the matchmaking example with:

```powershell
cargo run --example matchmaking
$env:BEVY_STEAMWORKS_CREATE_PRIVATE_LOBBY = "1"
cargo run --example matchmaking
```

## User Identity and Authentication

`SteamworksUserPlugin` adds command/result messages for current-user identity, Steam server connection state, auth session tickets, Web API auth tickets, remote ticket validation sessions, and license checks for authenticated users.

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

`SteamworksUserCommand::GetAuthenticationSessionTicket` immediately returns a ticket handle and bytes in `SteamworksUserResult`, then final Steam confirmation arrives through `SteamworksEvent::AuthSessionTicketResponse`. `SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi` returns the handle first; the Web API ticket bytes arrive through `SteamworksEvent::TicketForWebApiResponse`. Remote ticket invalidation still arrives through `SteamworksEvent::ValidateAuthTicketResponse`.

Call `SteamworksUserCommand::CancelAuthenticationTicket` when a locally issued ticket is no longer needed, and `SteamworksUserCommand::EndAuthenticationSession` when a remote authenticated session ends. The command layer tracks issued ticket handles and started sessions in `SteamworksUserState`.

Run the user/auth example with:

```powershell
cargo run --example user
$env:BEVY_STEAMWORKS_AUTH_TICKET = "1"
cargo run --example user
$env:BEVY_STEAMWORKS_WEBAPI_IDENTITY = "my-service"
cargo run --example user
```

## Utilities and Overlay Helpers

`SteamworksUtilsPlugin` adds command/result messages for Steam utility queries and lightweight overlay helpers: app id, IP country, Steam UI language, server real time, overlay availability, Big Picture mode, Steam Deck detection, and overlay notification position.

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

Gamepad text input callbacks are still available through `SteamworksEvent::GamepadTextInputDismissed` and `SteamworksEvent::FloatingGamepadTextInputDismissed`. For now, use the raw `steamworks::Utils` methods through `SteamworksClient` for text input flows that must read submitted text inside Steam's callback timing. Be aware that the upstream text input helpers register their own typed callbacks, so avoid also registering competing callbacks for the same dismissal types through `SteamworksCallbackRegistry`.

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

fn read_screenshot_callbacks(mut events: MessageReader<SteamworksEvent>) {
    for event in events.read() {
        match event {
            SteamworksEvent::ScreenshotRequested(event) => info!("{event:?}"),
            SteamworksEvent::ScreenshotReady(event) => info!("{event:?}"),
            _ => {}
        }
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksScreenshotsPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_screenshots)
        .add_systems(Update, (read_screenshots, read_screenshot_callbacks))
        .run();
}
```

`SteamworksScreenshotsCommand::AddScreenshotToLibrary` returns a screenshot handle when Steam accepts the request. Final save confirmation still arrives later through `SteamworksEvent::ScreenshotReady`. Width and height are validated before calling upstream `steamworks`, and path/save failures are reported as `SteamworksScreenshotsError::LibraryAddFailed`.

Only call `SteamworksScreenshotsCommand::hook_screenshots(true)` if your game will handle `SteamworksEvent::ScreenshotRequested` by capturing an image and submitting it to Steam; once hooked, Steam no longer handles the screenshot hotkey for you. `AddScreenshotToLibrary` canonicalizes local file paths through the upstream wrapper before submitting, so keep it low-frequency and avoid slow network paths in frame-critical flows.

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

`SteamworksUgcPlugin` adds command/result messages for common Steam Workshop workflows: query item details, search Workshop pages, list subscriptions, read item state/download/install info, submit downloads, subscribe/unsubscribe/delete items, create a new Workshop item, and start/stop playtime tracking.

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
}

fn read_ugc(mut results: MessageReader<SteamworksUgcResult>) {
    for result in results.read() {
        info!("{result:?}");
    }
}

fn read_ugc_callbacks(mut events: MessageReader<SteamworksEvent>) {
    for event in events.read() {
        if let SteamworksEvent::DownloadItemResult(event) = event {
            info!("Workshop download finished: {event:?}");
        }
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksUgcPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_ugc)
        .add_systems(Update, (read_ugc, read_ugc_callbacks))
        .run();
}
```

All async UGC commands emit an immediate `*Requested` operation with a plugin-assigned `request_id`, then emit the completion or async error after `SteamworksSystem::RunCallbacks` pumps Steam call results. Query results are copied into owned `SteamworksUgcQueryResults` snapshots, so callers do not need to hold upstream query handles or lifetimes.

String query options are validated before calling upstream `steamworks`, so interior NUL bytes become `SteamworksUgcError::InvalidString` instead of panicking in a C string conversion. `DownloadItem` only confirms that Steam accepted the download request; final completion still arrives through `SteamworksEvent::DownloadItemResult`.

The current UGC layer covers the runtime Workshop workflows most games need. Full item update submission with content and preview paths will be added as a separate command layer extension because it has a longer multi-step handle lifecycle.

Run the UGC example with:

```powershell
cargo run --example ugc
$env:BEVY_STEAMWORKS_UGC_ITEM = "123456"
cargo run --example ugc
$env:BEVY_STEAMWORKS_UGC_SEARCH = "levels"
cargo run --example ugc
$env:BEVY_STEAMWORKS_UGC_DOWNLOAD = "1"
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

fn read_remote_play_callbacks(mut events: MessageReader<SteamworksEvent>) {
    for event in events.read() {
        match event {
            SteamworksEvent::RemotePlayConnected(event) => {
                info!("Remote Play connected: {event:?}");
            }
            SteamworksEvent::RemotePlayDisconnected(event) => {
                info!("Remote Play disconnected: {event:?}");
            }
            _ => {}
        }
    }
}

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::app_id(480).log_and_continue())
        .add_plugins(SteamworksRemotePlayPlugin::new())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, request_remote_play)
        .add_systems(Update, (read_remote_play, read_remote_play_callbacks))
        .run();
}
```

`SteamworksRemotePlayCommand::ListSessions` returns session snapshots with user, client name, form factor, and resolution. The upstream bulk listing API does not expose session IDs, so use `SteamworksEvent::RemotePlayConnected` to capture a `RemotePlaySessionId`, then call `SteamworksRemotePlayCommand::GetSession` for ID-based session reads.

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

`SteamworksAppsCommand::GetCurrentAppInfo` combines the most commonly needed app checks into one `SteamworksCurrentAppInfo` snapshot. Launch query keys are validated before calling upstream `steamworks`, so interior NUL bytes become `SteamworksAppsError::InvalidString` instead of panicking.

Run the app info example with:

```powershell
cargo run --example apps
$env:BEVY_STEAMWORKS_LAUNCH_PARAM = "connect"
cargo run --example apps
```

## Achievements and Stats

`SteamworksStatsPlugin` adds a Bevy-native command/result layer for common user stats and achievement workflows. It is optional; you can still call the raw `steamworks` API through `SteamworksClient`.

```rust,no_run
# use bevy::prelude::*;
# use bevy_steamworks::prelude::*;
fn unlock_win(mut stats: MessageWriter<SteamworksStatsCommand>) {
    stats.write(SteamworksStatsCommand::unlock_achievement("ACH_WIN_ONE_GAME"));
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

By default, `SteamworksStatsPlugin` requests stats for the current Steam user once the `SteamworksClient` resource exists. Successful stat/achievement writes are coalesced into one `store_stats()` call per frame. `SteamworksStatsResult::Ok(SteamworksStatsOperation::StatsStoreSubmitted)` only means the store request was submitted; final Steam confirmation still arrives through `SteamworksEvent::UserStatsStored`, and unlocked achievements may also emit `SteamworksEvent::UserAchievementStored`.

For read-only tools or examples, disable automatic storage:

```rust,no_run
# use bevy_steamworks::prelude::*;
SteamworksStatsPlugin::new().auto_store(false);
```

Run the read-only stats example with:

```powershell
cargo run --example stats
$env:BEVY_STEAMWORKS_STAT_I32 = "your_stat_api_name"
$env:BEVY_STEAMWORKS_ACHIEVEMENT = "your_achievement_api_name"
cargo run --example stats
```

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
