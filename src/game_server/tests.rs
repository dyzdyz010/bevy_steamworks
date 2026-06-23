use std::net::{Ipv4Addr, SocketAddrV4};

use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::{
    user::{SteamworksAuthTicketValidation, SteamworksSteamServerConnectionEvent},
    SteamworksEvent,
};

use super::validation::{validate_server_command, validate_server_command_for_state};
use super::*;

#[test]
fn plugin_name_matches_game_server_type_path_for_bevy_tracking() {
    let plugin = SteamworksServerPlugin::manual();

    assert_eq!(
        plugin.name(),
        std::any::type_name::<SteamworksServerPlugin>()
    );
    assert_eq!(
        plugin.name(),
        "bevy_steamworks::game_server::SteamworksServerPlugin"
    );
}

#[test]
fn configuration_accessors_expose_builder_settings() {
    let config = SteamworksServerConfig::new(
        Ipv4Addr::LOCALHOST,
        27015,
        27016,
        steamworks::ServerMode::Authentication,
        "1.0.0",
    );
    let plugin = SteamworksServerPlugin::new(config.clone())
        .failure_policy(SteamworksFailurePolicy::LogAndContinue)
        .run_callbacks(false);

    assert_eq!(
        plugin.init_mode(),
        &SteamworksServerInitMode::Config(config)
    );
    assert_eq!(
        plugin.failure_policy_setting(),
        SteamworksFailurePolicy::LogAndContinue
    );
    assert!(!plugin.runs_callbacks());

    let plugin = SteamworksServerPlugin::manual();
    assert_eq!(plugin.init_mode(), &SteamworksServerInitMode::Manual);
    assert_eq!(
        plugin.failure_policy_setting(),
        SteamworksFailurePolicy::Panic
    );
    assert!(plugin.runs_callbacks());
}

#[test]
fn unavailable_accessors_expose_structured_status() {
    let config = SteamworksServerConfig::new(
        Ipv4Addr::LOCALHOST,
        27015,
        27016,
        steamworks::ServerMode::Authentication,
        "1.0.0",
    );
    let source = steamworks::SteamAPIInitError::NoSteamClient("Steam is not running".to_string());
    let unavailable = SteamworksServerUnavailable::InitFailed {
        config: config.clone(),
        source: source.clone(),
    };

    assert!(!unavailable.is_manual_server_missing());
    assert!(!unavailable.is_invalid_string());
    assert!(unavailable.is_init_failed());
    assert_eq!(unavailable.invalid_string_field(), None);
    assert_eq!(unavailable.init_config(), Some(&config));
    assert_eq!(unavailable.init_error(), Some(&source));

    let unavailable = SteamworksServerUnavailable::InvalidString { field: "version" };
    assert!(!unavailable.is_manual_server_missing());
    assert!(unavailable.is_invalid_string());
    assert!(!unavailable.is_init_failed());
    assert_eq!(unavailable.invalid_string_field(), Some("version"));
    assert_eq!(unavailable.init_config(), None);
    assert_eq!(unavailable.init_error(), None);

    let unavailable = SteamworksServerUnavailable::ManualServerMissing;
    assert!(unavailable.is_manual_server_missing());
    assert!(!unavailable.is_invalid_string());
    assert!(!unavailable.is_init_failed());
    assert_eq!(unavailable.invalid_string_field(), None);
    assert_eq!(unavailable.init_config(), None);
    assert_eq!(unavailable.init_error(), None);
}

#[test]
fn manual_mode_can_continue_without_server() {
    let mut app = App::new();

    app.add_plugins(SteamworksServerPlugin::manual().log_and_continue());

    assert!(app
        .world()
        .contains_resource::<SteamworksServerUnavailable>());
    assert!(app.world().contains_resource::<SteamworksServerState>());
    assert!(app
        .world()
        .contains_resource::<SteamworksServerCallbackRegistry>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksServerCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksServerResult>>());
    assert!(!app.world().contains_resource::<SteamworksServer>());

    app.update();
}

#[test]
#[should_panic(expected = "manual Steam Game Server initialization was selected")]
fn manual_mode_panics_by_default() {
    let mut app = App::new();

    app.add_plugins(SteamworksServerPlugin::manual());
}

#[test]
fn invalid_version_can_continue_without_server() {
    let mut app = App::new();

    app.add_plugins(
        SteamworksServerPlugin::new(SteamworksServerConfig::new(
            Ipv4Addr::LOCALHOST,
            27015,
            27016,
            steamworks::ServerMode::Authentication,
            "bad\0version",
        ))
        .log_and_continue(),
    );

    assert_eq!(
        app.world().resource::<SteamworksServerUnavailable>(),
        &SteamworksServerUnavailable::InvalidString { field: "version" }
    );
    assert!(!app.world().contains_resource::<SteamworksServer>());
}

#[test]
fn commands_fail_when_server_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksServerPlugin::manual().log_and_continue());
    app.world_mut()
        .resource_mut::<Messages<SteamworksServerCommand>>()
        .write(SteamworksServerCommand::get_steam_id());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksServerResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksServerResult::Err {
            command: SteamworksServerCommand::GetSteamId,
            error: SteamworksServerError::ServerUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksServerState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksServerError::ServerUnavailable)
    );
}

#[test]
fn server_callbacks_are_bridged_without_server() {
    let mut app = App::new();
    let user = steamworks::SteamId::from_raw(7);
    let owner = steamworks::SteamId::from_raw(8);
    let group = steamworks::SteamId::from_raw(9);

    app.add_plugins(SteamworksServerPlugin::manual().log_and_continue());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::SteamServersConnected(
            steamworks::SteamServersConnected,
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::ValidateAuthTicketResponse(
            steamworks::ValidateAuthTicketResponse {
                steam_id: user,
                owner_steam_id: owner,
                response: Ok(()),
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GSClientApprove(
            steamworks::GSClientApprove { user, owner },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::ValidateAuthTicketResponse(
            steamworks::ValidateAuthTicketResponse {
                steam_id: user,
                owner_steam_id: owner,
                response: Err(steamworks::AuthSessionValidateError::VACBanned),
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GSClientDeny(steamworks::GSClientDeny {
            user,
            deny_reason: steamworks::DenyReason::NoLicense,
            optional_text: "no license".to_string(),
        }));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GSClientKick(steamworks::GSClientKick {
            user,
            deny_reason: steamworks::DenyReason::SteamConnectionLost,
        }));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::GSClientGroupStatus(
            steamworks::GSClientGroupStatus {
                user,
                group,
                member: true,
                officer: false,
            },
        ));

    app.update();

    let validation_failed = SteamworksAuthTicketValidation {
        steam_id: user,
        owner_steam_id: owner,
        response: Err(crate::SteamworksAuthSessionValidateError::VacBanned),
    };
    let approval = SteamworksServerClientApproval { user, owner };
    let denial = SteamworksServerClientDenial {
        user,
        deny_reason: steamworks::DenyReason::NoLicense,
        optional_text: "no license".to_string(),
    };
    let kick = SteamworksServerClientKick {
        user,
        deny_reason: steamworks::DenyReason::SteamConnectionLost,
    };
    let group_status = SteamworksServerClientGroupStatus {
        user,
        group,
        member: true,
        officer: false,
    };

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksServerResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    assert_eq!(
        drained,
        vec![
            SteamworksServerResult::Ok(
                SteamworksServerOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Connected,
                },
            ),
            SteamworksServerResult::Ok(
                SteamworksServerOperation::AuthenticationTicketValidationReceived {
                    validation: SteamworksAuthTicketValidation {
                        steam_id: user,
                        owner_steam_id: owner,
                        response: Ok(()),
                    },
                },
            ),
            SteamworksServerResult::Ok(SteamworksServerOperation::ClientApproved {
                approval: approval.clone(),
            }),
            SteamworksServerResult::Ok(
                SteamworksServerOperation::AuthenticationTicketValidationReceived {
                    validation: validation_failed.clone(),
                },
            ),
            SteamworksServerResult::Ok(SteamworksServerOperation::ClientDenied {
                denial: denial.clone(),
            }),
            SteamworksServerResult::Ok(SteamworksServerOperation::ClientKicked {
                kick: kick.clone(),
            }),
            SteamworksServerResult::Ok(SteamworksServerOperation::ClientGroupStatusReceived {
                status: group_status.clone(),
            }),
        ]
    );

    let state = app.world().resource::<SteamworksServerState>();
    assert_eq!(state.steam_server_connected(), Some(true));
    assert_eq!(
        state.last_steam_server_connection_event(),
        Some(&SteamworksSteamServerConnectionEvent::Connected)
    );
    assert_eq!(
        state.steam_server_connection_events(),
        &[SteamworksSteamServerConnectionEvent::Connected]
    );
    assert_eq!(
        state.last_auth_ticket_validation(),
        Some(&validation_failed)
    );
    assert_eq!(
        state.auth_ticket_validation(user),
        state.last_auth_ticket_validation()
    );
    assert_eq!(state.auth_ticket_validation_succeeded(user), Some(false));
    assert_eq!(state.auth_ticket_validations().len(), 1);
    assert_eq!(state.last_client_approval(), Some(&approval));
    assert_eq!(state.client_approval(user), Some(&approval));
    assert!(state.has_client_approval(user));
    assert_eq!(state.client_approval_owner(user), Some(owner));
    assert_eq!(state.client_approvals(), &[approval]);
    assert_eq!(state.last_client_denial(), Some(&denial));
    assert_eq!(state.client_denial(user), Some(&denial));
    assert!(state.has_client_denial(user));
    assert_eq!(
        state.client_denial_reason(user),
        Some(steamworks::DenyReason::NoLicense)
    );
    assert_eq!(state.client_denials(), &[denial]);
    assert_eq!(state.last_client_kick(), Some(&kick));
    assert_eq!(state.client_kick(user), Some(&kick));
    assert!(state.has_client_kick(user));
    assert_eq!(
        state.client_kick_reason(user),
        Some(steamworks::DenyReason::SteamConnectionLost)
    );
    assert_eq!(state.client_kicks(), &[kick]);
    assert_eq!(state.last_client_group_status(), Some(&group_status));
    assert_eq!(state.client_group_status(user, group), Some(&group_status));
    assert_eq!(state.client_group_member(user, group), Some(true));
    assert_eq!(state.client_group_officer(user, group), Some(false));
    assert_eq!(state.client_group_statuses(), &[group_status]);
    assert!(state.authenticated_users().is_empty());
    assert!(!state.is_user_authenticated(user));
    assert_eq!(state.last_error(), None);
}

#[test]
fn constructors_preserve_inputs() {
    let user = steamworks::SteamId::from_raw(7);
    let identity = steamworks::networking_types::NetworkingIdentity::new_ip(
        std::net::SocketAddr::from(([127, 0, 0, 1], 27015)),
    );
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 27015);

    assert_eq!(
        SteamworksServerCommand::get_steam_id(),
        SteamworksServerCommand::GetSteamId
    );
    assert_eq!(
        SteamworksServerCommand::get_authentication_session_ticket(user),
        SteamworksServerCommand::GetAuthenticationSessionTicket { steam_id: user }
    );
    assert_eq!(
        SteamworksServerCommand::get_authentication_session_ticket_for_identity(
            crate::SteamworksNetworkingPeer::from(identity.clone())
        ),
        SteamworksServerCommand::GetAuthenticationSessionTicketForIdentity { identity }
    );
    let _cancel_constructor: fn(steamworks::AuthTicket) -> SteamworksServerCommand =
        SteamworksServerCommand::cancel_authentication_ticket;
    assert_eq!(
        SteamworksServerCommand::begin_authentication_session(user, [1, 2, 3]),
        SteamworksServerCommand::BeginAuthenticationSession {
            user,
            ticket: vec![1, 2, 3],
        }
    );
    assert_eq!(
        SteamworksServerCommand::end_authentication_session(user),
        SteamworksServerCommand::EndAuthenticationSession { user }
    );
    assert_eq!(
        SteamworksServerCommand::handle_incoming_packet([255, 255, 255, 255], addr),
        SteamworksServerCommand::HandleIncomingPacket {
            data: vec![255, 255, 255, 255],
            addr,
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_product("480"),
        SteamworksServerCommand::SetProduct {
            product: "480".to_string(),
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_game_description("Spacewar"),
        SteamworksServerCommand::SetGameDescription {
            description: "Spacewar".to_string(),
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_game_data("mode=arena"),
        SteamworksServerCommand::SetGameData {
            data: "mode=arena".to_string(),
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_dedicated_server(true),
        SteamworksServerCommand::SetDedicatedServer { dedicated: true }
    );
    assert_eq!(
        SteamworksServerCommand::set_key_value("map", "arena"),
        SteamworksServerCommand::SetKeyValue {
            key: "map".to_string(),
            value: "arena".to_string(),
        }
    );
    assert_eq!(
        SteamworksServerCommand::log_on_anonymous(),
        SteamworksServerCommand::LogOnAnonymous
    );
    assert_eq!(
        SteamworksServerCommand::log_on("secret-token"),
        SteamworksServerCommand::LogOn {
            token: SteamworksServerLoginToken::new("secret-token"),
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_advertise_server_active(true),
        SteamworksServerCommand::SetAdvertiseServerActive { active: true }
    );
    assert_eq!(
        SteamworksServerCommand::enable_heartbeats(true),
        SteamworksServerCommand::EnableHeartbeats { active: true }
    );
    assert_eq!(
        SteamworksServerCommand::set_mod_dir("spacewar"),
        SteamworksServerCommand::SetModDir {
            mod_dir: "spacewar".to_string(),
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_map_name("arena"),
        SteamworksServerCommand::SetMapName {
            map_name: "arena".to_string(),
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_server_name("Test Server"),
        SteamworksServerCommand::SetServerName {
            server_name: "Test Server".to_string(),
        }
    );
    assert_eq!(
        SteamworksServerCommand::set_max_players(16),
        SteamworksServerCommand::SetMaxPlayers { count: 16 }
    );
    assert_eq!(
        SteamworksServerCommand::set_game_tags("arena,pvp"),
        SteamworksServerCommand::SetGameTags {
            tags: "arena,pvp".to_string(),
        }
    );
    assert_eq!(
        format!("{:?}", SteamworksServerCommand::log_on("secret-token")),
        "LogOn { token: SteamworksServerLoginToken(<redacted>) }"
    );
    assert_eq!(
        SteamworksServerCommand::clear_all_key_values(),
        SteamworksServerCommand::ClearAllKeyValues
    );
    assert_eq!(
        SteamworksServerCommand::set_password_protected(false),
        SteamworksServerCommand::SetPasswordProtected { protected: false }
    );
    assert_eq!(
        SteamworksServerCommand::set_bot_player_count(2),
        SteamworksServerCommand::SetBotPlayerCount { count: 2 }
    );
    assert_eq!(
        SteamworksServerCommand::drain_outgoing_packets(),
        SteamworksServerCommand::DrainOutgoingPackets
    );
}

#[test]
fn debug_redacts_auth_and_packet_bytes() {
    let user = steamworks::SteamId::from_raw(7);
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 27015);
    let auth_command = SteamworksServerCommand::begin_authentication_session(user, vec![1, 2, 3]);
    let packet_command = SteamworksServerCommand::handle_incoming_packet(vec![4, 5, 6], addr);
    let packet = SteamworksServerOutgoingPacket {
        addr,
        data: vec![7, 8, 9],
    };
    let operation = SteamworksServerOperation::OutgoingPacketsDrained {
        packets: vec![packet.clone()],
    };
    let result = SteamworksServerResult::Ok(operation.clone());

    let auth_command_debug = format!("{auth_command:?}");
    let packet_command_debug = format!("{packet_command:?}");
    let packet_debug = format!("{packet:?}");
    let operation_debug = format!("{operation:?}");
    let result_debug = format!("{result:?}");

    assert!(auth_command_debug.contains("ticket_len: 3"));
    assert!(!auth_command_debug.contains("[1, 2, 3]"));
    assert!(packet_command_debug.contains("data_len: 3"));
    assert!(!packet_command_debug.contains("[4, 5, 6]"));
    assert!(packet_debug.contains("data_len: 3"));
    assert!(!packet_debug.contains("[7, 8, 9]"));
    assert!(operation_debug.contains("data_len: 3"));
    assert!(!operation_debug.contains("[7, 8, 9]"));
    assert!(result_debug.contains("data_len: 3"));
    assert!(!result_debug.contains("[7, 8, 9]"));
}

#[test]
fn validation_rejects_inputs_that_would_panic_upstream() {
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::set_product("bad\0product")),
        Err(SteamworksServerError::InvalidString { field: "product" })
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::set_key_value("bad\0key", "arena")),
        Err(SteamworksServerError::InvalidString { field: "key" })
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::set_key_value("map", "bad\0value")),
        Err(SteamworksServerError::InvalidString { field: "value" })
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::set_game_tags("")),
        Err(SteamworksServerError::InvalidGameTags)
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::set_game_tags("a".repeat(128))),
        Err(SteamworksServerError::InvalidGameTags)
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::set_max_players(-1)),
        Err(SteamworksServerError::InvalidCount {
            field: "count",
            value: -1,
        })
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::set_bot_player_count(-1)),
        Err(SteamworksServerError::InvalidCount {
            field: "count",
            value: -1,
        })
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::begin_authentication_session(
            steamworks::SteamId::from_raw(1),
            Vec::new(),
        )),
        Err(SteamworksServerError::EmptyTicket)
    );
    assert_eq!(
        validate_server_command(
            &SteamworksServerCommand::get_authentication_session_ticket_for_identity(
                steamworks::networking_types::NetworkingIdentity::new(),
            )
        ),
        Err(SteamworksServerError::InvalidNetworkingIdentity)
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::log_on("")),
        Err(SteamworksServerError::EmptyLogonToken)
    );
    assert_eq!(
        validate_server_command(&SteamworksServerCommand::log_on("bad\0token")),
        Err(SteamworksServerError::InvalidString { field: "token" })
    );
}

#[test]
fn validation_rejects_pre_logon_only_commands_after_logon() {
    let mut state = SteamworksServerState::default();

    assert_eq!(
        validate_server_command_for_state(&SteamworksServerCommand::set_product("480"), &state),
        Ok(())
    );

    state.record_operation(&SteamworksServerOperation::AnonymousLogonSubmitted);

    assert_eq!(
        validate_server_command_for_state(&SteamworksServerCommand::set_product("480"), &state),
        Err(SteamworksServerError::CommandRequiresPreLogon {
            command: "SetProduct",
        })
    );
    assert_eq!(
        validate_server_command_for_state(
            &SteamworksServerCommand::set_game_description("Spacewar"),
            &state
        ),
        Err(SteamworksServerError::CommandRequiresPreLogon {
            command: "SetGameDescription",
        })
    );
    assert_eq!(
        validate_server_command_for_state(
            &SteamworksServerCommand::set_server_name("Arena"),
            &state
        ),
        Ok(())
    );
    assert_eq!(
        validate_server_command_for_state(&SteamworksServerCommand::log_on_anonymous(), &state),
        Err(SteamworksServerError::LogonAlreadySubmitted)
    );
    assert_eq!(
        validate_server_command_for_state(&SteamworksServerCommand::log_on("token"), &state),
        Err(SteamworksServerError::LogonAlreadySubmitted)
    );
}

#[test]
fn state_records_server_operations() {
    let mut state = SteamworksServerState::default();
    let steam_id = steamworks::SteamId::from_raw(1);
    let user = steamworks::SteamId::from_raw(2);
    let packet = SteamworksServerOutgoingPacket {
        addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 27015),
        data: vec![1, 2, 3],
    };

    state.record_operation(&SteamworksServerOperation::SteamIdRead { steam_id });
    state.record_operation(&SteamworksServerOperation::AuthenticationSessionStarted { user });
    state.record_operation(&SteamworksServerOperation::AuthenticationSessionStarted { user });
    state.record_operation(&SteamworksServerOperation::ProductSet {
        product: "480".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::GameDescriptionSet {
        description: "Spacewar".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::GameDataSet {
        data: "mode=arena".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::DedicatedServerSet { dedicated: true });
    state.record_operation(&SteamworksServerOperation::AnonymousLogonSubmitted);
    state.record_operation(&SteamworksServerOperation::TokenLogonSubmitted);
    state.record_operation(&SteamworksServerOperation::AdvertiseServerActiveSet { active: true });
    state.record_operation(&SteamworksServerOperation::HeartbeatsEnabled { active: true });
    state.record_operation(&SteamworksServerOperation::ModDirSet {
        mod_dir: "spacewar".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::MapNameSet {
        map_name: "arena".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::ServerNameSet {
        server_name: "Test Server".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::MaxPlayersSet { count: 16 });
    state.record_operation(&SteamworksServerOperation::GameTagsSet {
        tags: "arena,pvp".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::KeyValueSet {
        key: "map".to_string(),
        value: "arena".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::KeyValueSet {
        key: "map".to_string(),
        value: "arena2".to_string(),
    });
    state.record_operation(&SteamworksServerOperation::PasswordProtectedSet { protected: false });
    state.record_operation(&SteamworksServerOperation::BotPlayerCountSet { count: 2 });
    state.record_operation(&SteamworksServerOperation::IncomingPacketHandled {
        addr: packet.addr,
        bytes: packet.data.len(),
        accepted: true,
    });
    state.record_operation(&SteamworksServerOperation::OutgoingPacketsDrained {
        packets: vec![packet.clone()],
    });

    assert_eq!(state.steam_id(), Some(steam_id));
    assert_eq!(state.authenticated_users(), &[user]);
    assert!(state.is_user_authenticated(user));
    assert!(state.active_auth_tickets().is_empty());
    assert_eq!(state.active_auth_ticket_count(), 0);
    assert!(state.last_auth_session_ticket().is_none());
    assert!(state.auth_session_tickets().is_empty());
    assert!(state.last_auth_session_ticket_for_identity().is_none());
    assert!(state.auth_session_tickets_for_identity().is_empty());
    assert_eq!(state.auth_session_ticket_issue_count(), 0);
    assert!(state.last_cancelled_auth_ticket().is_none());
    assert_eq!(state.auth_ticket_cancel_count(), 0);
    assert_eq!(state.last_started_authentication_session(), Some(user));
    assert_eq!(state.authentication_session_start_count(), 2);
    assert_eq!(state.product(), Some("480"));
    assert_eq!(state.game_description(), Some("Spacewar"));
    assert_eq!(state.game_data(), Some("mode=arena"));
    assert_eq!(state.dedicated(), Some(true));
    assert!(state.anonymous_logon_submitted());
    assert!(state.token_logon_submitted());
    assert!(state.logon_submitted());
    assert_eq!(state.advertise_server_active(), Some(true));
    assert_eq!(state.heartbeats_active(), Some(true));
    assert_eq!(state.mod_dir(), Some("spacewar"));
    assert_eq!(state.map_name(), Some("arena"));
    assert_eq!(state.server_name(), Some("Test Server"));
    assert_eq!(state.max_players(), Some(16));
    assert_eq!(state.game_tags(), Some("arena,pvp"));
    assert_eq!(
        state.key_values(),
        &[("map".to_string(), "arena2".to_string())]
    );
    assert_eq!(state.key_value("map"), Some("arena2"));
    assert_eq!(state.key_value("mode"), None);
    assert_eq!(state.password_protected(), Some(false));
    assert_eq!(state.bot_player_count(), Some(2));
    assert_eq!(
        state.last_incoming_packet(),
        Some(&SteamworksServerIncomingPacket {
            addr: packet.addr,
            bytes: packet.data.len(),
            accepted: true,
        })
    );
    assert_eq!(state.incoming_packet_count(), 1);
    assert_eq!(state.last_outgoing_packets(), &[packet]);
    assert_eq!(state.outgoing_packet_drain_count(), 1);

    state.record_operation(&SteamworksServerOperation::AuthenticationSessionEnded { user });
    state.record_operation(&SteamworksServerOperation::AllKeyValuesCleared);

    assert!(state.authenticated_users().is_empty());
    assert!(!state.is_user_authenticated(user));
    assert_eq!(state.last_ended_authentication_session(), Some(user));
    assert_eq!(state.authentication_session_end_count(), 1);
    assert!(state.key_values().is_empty());
    assert_eq!(state.key_value("map"), None);
}

#[test]
fn server_state_callback_lookup_caches_are_bounded() {
    let mut state = SteamworksServerState::default();
    let limit = 1_024;

    for index in 0..(limit + 4) {
        let user = steamworks::SteamId::from_raw((index + 1) as u64);
        state.record_operation(&SteamworksServerOperation::ClientApproved {
            approval: SteamworksServerClientApproval { user, owner: user },
        });
    }

    assert_eq!(state.client_approvals().len(), limit);
    assert_eq!(
        state.client_approval(steamworks::SteamId::from_raw(1)),
        None
    );
    let last_user = steamworks::SteamId::from_raw((limit + 4) as u64);
    assert_eq!(
        state.client_approval(last_user),
        Some(&SteamworksServerClientApproval {
            user: last_user,
            owner: last_user,
        })
    );
}

#[test]
fn server_callback_registry_tracks_handles() {
    let registry = SteamworksServerCallbackRegistry::default();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}
