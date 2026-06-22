# Feature Coverage

This crate exposes Steamworks through Bevy resources, commands, results, and callback messages while keeping the raw SDK boundary inside the upstream `steamworks` crate. The table below is a working support map for planning future work.

## Coverage Map

| Area | Status | Notes |
|:-----|:-------|:------|
| Client lifecycle | Implemented | `SteamworksPlugin`, `SteamworksPlugins`, fail-fast by default, `LogAndContinue` diagnostics, callback pump in `SteamworksSystem::RunCallbacks`. |
| Apps | Implemented | Subscription, install/DLC checks, ownership, languages, beta, launch command/query params, URL launch callback. |
| Friends | Implemented | Persona/friend snapshots, coplay, user info requests, rich presence, overlays, invites, played-with, relationship checks, avatars, callbacks. |
| User | Implemented | Steam ID, level, logged-on state, auth session tickets, Web API tickets, remote ticket validation, license checks, relevant callbacks. |
| User stats | Implemented | Current stats lifecycle, int/float stats, achievements, global stats/history, global achievement percentages, leaderboards, store/reset. |
| Matchmaking | Implemented | Lobby create/join/leave, lobby lists/filters, lobby data, member data, owner/limits/joinability, chat, game server assignment, callbacks. |
| Matchmaking servers | Implemented | Server list requests, refresh/release/detail reads, direct ping/player-details/rules queries, async result tracking. |
| Legacy networking | Implemented | Legacy P2P sessions, send/read packets, packet availability, session callbacks. |
| Networking messages | Implemented | Send/receive, session connection info, session request/failure callbacks, configurable auto-accept. |
| Networking sockets | Implemented | Listen/connect, accept/close, single/all event polling, receives, flushes, and teardown, messages, batch sends, poll groups, connection names/user data, lanes, auth helpers, realtime status, client/server ownership tracking. |
| Networking utils | Implemented | Relay initialization, relay status callbacks, detailed relay diagnostics, ping measurement/config/any-relay/debug-message reads. |
| Input | Implemented | Init/run/shutdown, controller and action handles, digital/analog data, origins/glyphs/names, cached origin presentation lookup, motion data, action set activation, binding panel. |
| Screenshots | Implemented | Hook state, trigger, library add, requested/ready callbacks. |
| Remote Storage | Implemented | Cloud state, files, metadata/existence/persisted/timestamps, read/write/delete/forget, sync platforms, file share. |
| UGC | Implemented | Subscriptions, item state/download/install info, item downloads, queries, total/ID-only query paths, item create/update/delete, update progress, playtime tracking, game-server workshop init. |
| Remote Play | Implemented | Session lists, ID-based session reads, invites, connect/disconnect callbacks. |
| Timeline | Implemented | Game mode, state descriptions, events. |
| Game server | Implemented | Server lifecycle plugin, auth tickets, remote ticket validation, shared query packets, product/description/data, logon, advertisement/heartbeats, server browser metadata. |

## Upstream-Safe Limits

The crate intentionally does not call raw Steamworks SDK functions directly unless there is a clear reason and a safe design. Known limits that are currently gated by upstream safe API support include:

- `ISteamNetworkingMessages` session/channel close commands are not exposed as safe methods by `steamworks` 0.13.1. The upstream crate only uses raw close internally for `SessionRequest::reject`.
- Dedicated-server `ISteamApps` access is not exposed as a safe method by `steamworks` 0.13.1; the upstream `Server::apps` wrapper is commented out as buggy.
- Some UGC query options, such as requesting additional previews, can be forwarded, but owned snapshots only include rows that the upstream `steamworks` crate exposes through safe accessors.
- Screenshot tagging and some lower-level Steamworks SDK surfaces are not wrapped until the upstream crate exposes safe APIs or this crate adds a reviewed safe abstraction.

When adding coverage, prefer wrapping upstream safe methods first. If a feature requires raw SDK access, keep the unsafe code isolated, document the invariant, and add focused tests around validation and failure behavior.

## Validation Policy

Feature batches should include the smallest test scope that proves the changed behavior:

- `cargo fmt --check`
- `git diff --check`
- module-specific `cargo test <module>::`
- `cargo test --test public_api <test_name>` when root/prelude exports change
- `cargo check --example <example>` when an example is touched

Full `cargo test` and `cargo clippy --all-targets -- -D warnings` remain release-quality gates, but day-to-day feature batches can use targeted checks to keep development moving.
