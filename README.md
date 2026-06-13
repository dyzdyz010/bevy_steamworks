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
