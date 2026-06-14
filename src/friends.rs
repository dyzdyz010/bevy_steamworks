//! High-level Bevy ECS integration for Steam friends, Rich Presence, overlays,
//! and invites.
//!
//! This module builds on top of the upstream [`steamworks::Friends`] API. Games
//! can keep using the raw Steamworks API through [`SteamworksClient`], while this
//! plugin provides a message-driven layer for common Bevy workflows.

use bevy_app::{App, First, Plugin};
use bevy_ecs::{
    message::{MessageReader, MessageWriter, Messages},
    prelude::{Res, ResMut},
    schedule::IntoScheduleConfigs,
};

use crate::{SteamworksClient, SteamworksEvent, SteamworksSystem};

mod messages;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use messages::*;
pub use state::SteamworksFriendsState;
pub use types::*;

/// Bevy plugin for high-level Steam friends, Rich Presence, overlay, and invite commands.
///
/// Add this plugin after [`crate::SteamworksPlugin`]. It registers
/// [`SteamworksFriendsCommand`] and [`SteamworksFriendsResult`] messages and
/// runs its command processor in [`bevy_app::First`] after Steam callbacks. It
/// also mirrors common friends, overlay, and invite callbacks into friends results.
#[derive(Clone, Debug, Default)]
pub struct SteamworksFriendsPlugin;

impl SteamworksFriendsPlugin {
    /// Creates a friends plugin with default behavior.
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SteamworksFriendsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamworksFriendsState>()
            .add_message::<SteamworksEvent>()
            .add_message::<SteamworksFriendsCommand>()
            .add_message::<SteamworksFriendsResult>()
            .configure_sets(
                First,
                SteamworksSystem::ProcessFriendsCommands
                    .after(SteamworksSystem::RunCallbacks)
                    .before(bevy_ecs::message::MessageUpdateSystems),
            )
            .add_systems(
                First,
                process_friends_commands.in_set(SteamworksSystem::ProcessFriendsCommands),
            );
    }
}

fn process_friends_commands(
    client: Option<Res<SteamworksClient>>,
    mut state: ResMut<SteamworksFriendsState>,
    mut commands: ResMut<Messages<SteamworksFriendsCommand>>,
    mut steam_events: MessageReader<SteamworksEvent>,
    mut results: MessageWriter<SteamworksFriendsResult>,
) {
    process_friends_steam_events(&mut state, &mut steam_events, &mut results);

    let Some(client) = client else {
        let error = SteamworksFriendsError::ClientUnavailable;
        for command in commands.drain() {
            state.record_error(error.clone());
            results.write(SteamworksFriendsResult::Err {
                command,
                error: error.clone(),
            });
        }
        return;
    };

    for command in commands.drain() {
        match handle_friends_command(&client, &command) {
            Ok(operation) => {
                state.record_operation(&operation);
                tracing::debug!(
                    target: "bevy_steamworks",
                    operation = ?operation,
                    "processed Steamworks friends command"
                );
                results.write(SteamworksFriendsResult::Ok(operation));
            }
            Err(error) => {
                state.record_error(error.clone());
                tracing::error!(
                    target: "bevy_steamworks",
                    command = ?command,
                    error = %error,
                    "Steamworks friends command failed"
                );
                results.write(SteamworksFriendsResult::Err { command, error });
            }
        }
    }
}

fn process_friends_steam_events(
    state: &mut SteamworksFriendsState,
    steam_events: &mut MessageReader<SteamworksEvent>,
    results: &mut MessageWriter<SteamworksFriendsResult>,
) {
    for event in steam_events.read() {
        let operation = match event {
            SteamworksEvent::GameOverlayActivated(event) => {
                SteamworksFriendsOperation::GameOverlayActivationChanged {
                    active: event.active,
                }
            }
            SteamworksEvent::PersonaStateChange(event) => {
                SteamworksFriendsOperation::PersonaStateChanged {
                    change: SteamworksPersonaStateChange {
                        steam_id: event.steam_id,
                        flags: event.flags,
                    },
                }
            }
            SteamworksEvent::GameLobbyJoinRequested(event) => {
                SteamworksFriendsOperation::GameLobbyJoinRequested {
                    request: SteamworksLobbyJoinRequest {
                        lobby: event.lobby_steam_id,
                        friend_steam_id: event.friend_steam_id,
                    },
                }
            }
            SteamworksEvent::GameRichPresenceJoinRequested(event) => {
                SteamworksFriendsOperation::GameRichPresenceJoinRequested {
                    request: SteamworksRichPresenceJoinRequest {
                        friend_steam_id: event.friend_steam_id,
                        connect: event.connect.clone(),
                    },
                }
            }
            _ => continue,
        };

        state.record_operation(&operation);
        tracing::debug!(
            target: "bevy_steamworks",
            operation = ?operation,
            "processed Steamworks friends callback"
        );
        results.write(SteamworksFriendsResult::Ok(operation));
    }
}

fn handle_friends_command(
    client: &SteamworksClient,
    command: &SteamworksFriendsCommand,
) -> Result<SteamworksFriendsOperation, SteamworksFriendsError> {
    validate_command_strings(command)?;

    let friends = client.friends();
    match command {
        SteamworksFriendsCommand::GetPersonaName => {
            Ok(SteamworksFriendsOperation::PersonaNameRead {
                name: friends.name(),
            })
        }
        SteamworksFriendsCommand::ListFriends { flags } => {
            let listed = friends
                .get_friends(*flags)
                .iter()
                .map(snapshot_friend)
                .collect();
            Ok(SteamworksFriendsOperation::FriendsListed {
                flags: *flags,
                friends: listed,
            })
        }
        SteamworksFriendsCommand::ListCoplayFriends => {
            let listed = friends
                .get_coplay_friends()
                .iter()
                .map(|friend| SteamworksCoplayFriendInfo {
                    friend: snapshot_friend(friend),
                    coplay_app_id: friend.coplay_game_played(),
                    coplay_time: friend.coplay_time(),
                })
                .collect();
            Ok(SteamworksFriendsOperation::CoplayFriendsListed { friends: listed })
        }
        SteamworksFriendsCommand::GetFriend { steam_id } => {
            let friend = friends.get_friend(*steam_id);
            Ok(SteamworksFriendsOperation::FriendRead {
                friend: snapshot_friend(&friend),
            })
        }
        SteamworksFriendsCommand::RequestUserInformation {
            steam_id,
            name_only,
        } => Ok(SteamworksFriendsOperation::UserInformationRequested {
            steam_id: *steam_id,
            name_only: *name_only,
            requested: friends.request_user_information(*steam_id, *name_only),
        }),
        SteamworksFriendsCommand::SetRichPresence { key, value } => {
            let cleared = value.as_deref().unwrap_or_default().is_empty();
            if friends.set_rich_presence(key, value.as_deref()) {
                Ok(SteamworksFriendsOperation::RichPresenceSet {
                    key: key.clone(),
                    cleared,
                })
            } else {
                Err(SteamworksFriendsError::operation_failed(
                    "friends.set_rich_presence",
                ))
            }
        }
        SteamworksFriendsCommand::ClearRichPresence => {
            friends.clear_rich_presence();
            Ok(SteamworksFriendsOperation::RichPresenceCleared)
        }
        SteamworksFriendsCommand::GetFriendRichPresence { steam_id, key } => {
            let friend = friends.get_friend(*steam_id);
            Ok(SteamworksFriendsOperation::FriendRichPresenceRead {
                steam_id: *steam_id,
                key: key.clone(),
                value: friend.rich_presence(key),
            })
        }
        SteamworksFriendsCommand::ActivateGameOverlay { dialog } => {
            friends.activate_game_overlay(dialog);
            Ok(SteamworksFriendsOperation::GameOverlayActivated {
                dialog: dialog.clone(),
            })
        }
        SteamworksFriendsCommand::ActivateGameOverlayToWebPage { url } => {
            friends.activate_game_overlay_to_web_page(url);
            Ok(SteamworksFriendsOperation::GameOverlayToWebPageActivated { url: url.clone() })
        }
        SteamworksFriendsCommand::ActivateGameOverlayToStore { app_id, action } => {
            friends.activate_game_overlay_to_store(*app_id, action.to_steam());
            Ok(SteamworksFriendsOperation::GameOverlayToStoreActivated {
                app_id: *app_id,
                action: *action,
            })
        }
        SteamworksFriendsCommand::ActivateGameOverlayToUser { dialog, steam_id } => {
            friends.activate_game_overlay_to_user(dialog, *steam_id);
            Ok(SteamworksFriendsOperation::GameOverlayToUserActivated {
                dialog: dialog.clone(),
                steam_id: *steam_id,
            })
        }
        SteamworksFriendsCommand::ActivateInviteDialog { lobby } => {
            friends.activate_invite_dialog(*lobby);
            Ok(SteamworksFriendsOperation::InviteDialogActivated { lobby: *lobby })
        }
        SteamworksFriendsCommand::ActivateInviteDialogConnectString { connect } => {
            friends.activate_invite_dialog_connect_string(connect);
            Ok(
                SteamworksFriendsOperation::InviteDialogConnectStringActivated {
                    connect: connect.clone(),
                },
            )
        }
        SteamworksFriendsCommand::InviteUserToGame { steam_id, connect } => {
            let friend = friends.get_friend(*steam_id);
            friend.invite_user_to_game(connect);
            Ok(SteamworksFriendsOperation::UserInvitedToGame {
                steam_id: *steam_id,
                connect: connect.clone(),
            })
        }
        SteamworksFriendsCommand::SetPlayedWith { steam_id } => {
            let friend = friends.get_friend(*steam_id);
            friend.set_played_with();
            Ok(SteamworksFriendsOperation::PlayedWithSet {
                steam_id: *steam_id,
            })
        }
        SteamworksFriendsCommand::HasFriend { steam_id, flags } => {
            let friend = friends.get_friend(*steam_id);
            Ok(SteamworksFriendsOperation::HasFriendRead {
                steam_id: *steam_id,
                flags: *flags,
                has_friend: friend.has_friend(*flags),
            })
        }
        SteamworksFriendsCommand::GetFriendAvatar { steam_id, size } => {
            let friend = friends.get_friend(*steam_id);
            let rgba = match size {
                SteamworksAvatarSize::Small => friend.small_avatar(),
                SteamworksAvatarSize::Medium => friend.medium_avatar(),
                SteamworksAvatarSize::Large => friend.large_avatar(),
            };
            let avatar = rgba.map(|rgba| {
                let (width, height) = size.dimensions();
                SteamworksAvatar {
                    size: *size,
                    width,
                    height,
                    rgba,
                }
            });
            Ok(SteamworksFriendsOperation::FriendAvatarRead {
                steam_id: *steam_id,
                size: *size,
                avatar,
            })
        }
    }
}

fn snapshot_friend(friend: &steamworks::Friend) -> SteamworksFriendInfo {
    SteamworksFriendInfo {
        steam_id: friend.id(),
        name: friend.name(),
        nickname: friend.nick_name(),
        state: friend.state(),
        game: friend.game_played().map(Into::into),
    }
}

fn validate_command_strings(
    command: &SteamworksFriendsCommand,
) -> Result<(), SteamworksFriendsError> {
    match command {
        SteamworksFriendsCommand::SetRichPresence { key, value } => {
            validate_steam_string("key", key)?;
            if let Some(value) = value {
                validate_steam_string("value", value)?;
            }
        }
        SteamworksFriendsCommand::GetFriendRichPresence { key, .. } => {
            validate_steam_string("key", key)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlay { dialog } => {
            validate_steam_string("dialog", dialog)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlayToWebPage { url } => {
            validate_steam_string("url", url)?;
        }
        SteamworksFriendsCommand::ActivateGameOverlayToUser { dialog, .. } => {
            validate_steam_string("dialog", dialog)?;
        }
        SteamworksFriendsCommand::ActivateInviteDialogConnectString { connect }
        | SteamworksFriendsCommand::InviteUserToGame { connect, .. } => {
            validate_steam_string("connect", connect)?;
        }
        _ => {}
    }

    Ok(())
}

fn validate_steam_string(field: &'static str, value: &str) -> Result<(), SteamworksFriendsError> {
    if value.as_bytes().contains(&0) {
        Err(SteamworksFriendsError::invalid_string(field))
    } else {
        Ok(())
    }
}
