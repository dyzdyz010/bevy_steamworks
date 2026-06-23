use bevy_app::{App, Plugin};
use bevy_ecs::message::Messages;

use crate::SteamworksEvent;

use super::*;

#[test]
fn user_plugin_registers_resources_and_messages() {
    let mut app = App::new();

    app.add_plugins(SteamworksUserPlugin::new());

    assert!(app.world().contains_resource::<SteamworksUserState>());
    assert!(app.world().contains_resource::<Messages<SteamworksEvent>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUserCommand>>());
    assert!(app
        .world()
        .contains_resource::<Messages<SteamworksUserResult>>());
}

#[test]
fn plugin_name_matches_user_type_path_for_bevy_tracking() {
    let plugin = SteamworksUserPlugin::new();

    assert_eq!(plugin.name(), std::any::type_name::<SteamworksUserPlugin>());
    assert_eq!(plugin.name(), "bevy_steamworks::user::SteamworksUserPlugin");
}

#[test]
fn commands_fail_when_client_is_unavailable() {
    let mut app = App::new();

    app.add_plugins(SteamworksUserPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksUserCommand>>()
        .write(SteamworksUserCommand::get_current_user_info());

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUserResult>>();
    let drained = results.drain().collect::<Vec<_>>();

    assert_eq!(
        drained,
        vec![SteamworksUserResult::Err {
            command: SteamworksUserCommand::GetCurrentUserInfo,
            error: SteamworksUserError::ClientUnavailable,
        }]
    );

    let state = app.world().resource::<SteamworksUserState>();
    assert_eq!(
        state.last_error(),
        Some(&SteamworksUserError::ClientUnavailable)
    );
}

#[test]
fn constructors_preserve_inputs() {
    let steam_id = steamworks::SteamId::from_raw(1);
    let identity = steamworks::networking_types::NetworkingIdentity::new_ip(
        std::net::SocketAddr::from(([127, 0, 0, 1], 27015)),
    );
    let app_id = steamworks::AppId(480);

    assert_eq!(
        SteamworksUserCommand::get_current_user_info(),
        SteamworksUserCommand::GetCurrentUserInfo
    );
    assert_eq!(
        SteamworksUserCommand::get_steam_id(),
        SteamworksUserCommand::GetSteamId
    );
    assert_eq!(
        SteamworksUserCommand::get_level(),
        SteamworksUserCommand::GetLevel
    );
    assert_eq!(
        SteamworksUserCommand::is_logged_on(),
        SteamworksUserCommand::IsLoggedOn
    );
    assert_eq!(
        SteamworksUserCommand::get_authentication_session_ticket(steam_id),
        SteamworksUserCommand::GetAuthenticationSessionTicket { steam_id }
    );
    assert_eq!(
        SteamworksUserCommand::get_authentication_session_ticket_for_identity(
            crate::SteamworksNetworkingPeer::from(identity.clone())
        ),
        SteamworksUserCommand::GetAuthenticationSessionTicketForIdentity { identity }
    );
    assert_eq!(
        SteamworksUserCommand::get_authentication_session_ticket_for_web_api("service"),
        SteamworksUserCommand::GetAuthenticationSessionTicketForWebApi {
            identity: "service".to_owned(),
        }
    );
    let _cancel_constructor: fn(steamworks::AuthTicket) -> SteamworksUserCommand =
        SteamworksUserCommand::cancel_authentication_ticket;
    assert_eq!(
        SteamworksUserCommand::begin_authentication_session(steam_id, [1, 2, 3]),
        SteamworksUserCommand::BeginAuthenticationSession {
            user: steam_id,
            ticket: vec![1, 2, 3],
        }
    );
    assert_eq!(
        SteamworksUserCommand::end_authentication_session(steam_id),
        SteamworksUserCommand::EndAuthenticationSession { user: steam_id }
    );
    assert_eq!(
        SteamworksUserCommand::user_has_license_for_app(steam_id, app_id),
        SteamworksUserCommand::UserHasLicenseForApp {
            user: steam_id,
            app_id,
        }
    );
}

#[test]
fn command_debug_redacts_authentication_ticket_bytes() {
    let command = SteamworksUserCommand::begin_authentication_session(
        steamworks::SteamId::from_raw(1),
        vec![1, 2, 3, 4],
    );
    let debug = format!("{command:?}");

    assert!(debug.contains("ticket_len: 4"));
    assert!(!debug.contains("[1, 2, 3, 4]"));
}

#[test]
fn auth_session_errors_are_cloneable_and_comparable() {
    assert_eq!(
        SteamworksAuthSessionError::from(steamworks::AuthSessionError::InvalidTicket),
        SteamworksAuthSessionError::InvalidTicket
    );
    assert_eq!(
        SteamworksAuthSessionError::from(steamworks::AuthSessionError::DuplicateRequest),
        SteamworksAuthSessionError::DuplicateRequest
    );
    assert_eq!(
        SteamworksAuthSessionError::from(steamworks::AuthSessionError::InvalidVersion),
        SteamworksAuthSessionError::InvalidVersion
    );
    assert_eq!(
        SteamworksAuthSessionError::from(steamworks::AuthSessionError::GameMismatch),
        SteamworksAuthSessionError::GameMismatch
    );
    assert_eq!(
        SteamworksAuthSessionError::from(steamworks::AuthSessionError::ExpiredTicket),
        SteamworksAuthSessionError::ExpiredTicket
    );
}

#[test]
fn auth_session_validate_errors_are_cloneable_and_comparable() {
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::UserNotConnectedToSteam,
        ),
        SteamworksAuthSessionValidateError::UserNotConnectedToSteam
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::NoLicenseOrExpired,
        ),
        SteamworksAuthSessionValidateError::NoLicenseOrExpired
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(steamworks::AuthSessionValidateError::VACBanned,),
        SteamworksAuthSessionValidateError::VacBanned
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::LoggedInElseWhere,
        ),
        SteamworksAuthSessionValidateError::LoggedInElseWhere
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::VACCheckTimedOut,
        ),
        SteamworksAuthSessionValidateError::VacCheckTimedOut
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::AuthTicketCancelled,
        ),
        SteamworksAuthSessionValidateError::AuthTicketCancelled
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::AuthTicketInvalidAlreadyUsed,
        ),
        SteamworksAuthSessionValidateError::AuthTicketInvalidAlreadyUsed
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::AuthTicketInvalid,
        ),
        SteamworksAuthSessionValidateError::AuthTicketInvalid
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::PublisherIssuedBan,
        ),
        SteamworksAuthSessionValidateError::PublisherIssuedBan
    );
    assert_eq!(
        SteamworksAuthSessionValidateError::from(
            steamworks::AuthSessionValidateError::AuthTicketNetworkIdentityFailure,
        ),
        SteamworksAuthSessionValidateError::AuthTicketNetworkIdentityFailure
    );
}

#[test]
fn auth_validation_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let user = steamworks::SteamId::from_raw(1);
    let owner = steamworks::SteamId::from_raw(2);

    app.add_plugins(SteamworksUserPlugin::new());
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
        .write(SteamworksEvent::ValidateAuthTicketResponse(
            steamworks::ValidateAuthTicketResponse {
                steam_id: user,
                owner_steam_id: owner,
                response: Err(steamworks::AuthSessionValidateError::VACBanned),
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUserResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    assert_eq!(
        drained,
        vec![
            SteamworksUserResult::Ok(
                SteamworksUserOperation::AuthenticationTicketValidationReceived {
                    validation: SteamworksAuthTicketValidation {
                        steam_id: user,
                        owner_steam_id: owner,
                        response: Ok(()),
                    },
                },
            ),
            SteamworksUserResult::Ok(
                SteamworksUserOperation::AuthenticationTicketValidationReceived {
                    validation: SteamworksAuthTicketValidation {
                        steam_id: user,
                        owner_steam_id: owner,
                        response: Err(SteamworksAuthSessionValidateError::VacBanned),
                    },
                },
            ),
        ]
    );

    let state = app.world().resource::<SteamworksUserState>();
    assert_eq!(
        state.last_auth_ticket_validation(),
        Some(&SteamworksAuthTicketValidation {
            steam_id: user,
            owner_steam_id: owner,
            response: Err(SteamworksAuthSessionValidateError::VacBanned),
        })
    );
    assert!(state.authenticated_users().is_empty());
    assert_eq!(state.last_error(), None);
}

#[test]
fn connection_and_microtxn_callbacks_are_bridged_without_client() {
    let mut app = App::new();
    let app_id = steamworks::AppId(480);

    app.add_plugins(SteamworksUserPlugin::new());
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::SteamServersConnected(
            steamworks::SteamServersConnected,
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::SteamServersDisconnected(
            steamworks::SteamServersDisconnected {
                reason: steamworks::SteamError::NoConnection,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::SteamServerConnectFailure(
            steamworks::SteamServerConnectFailure {
                reason: steamworks::SteamError::Timeout,
                still_retrying: true,
            },
        ));
    app.world_mut()
        .resource_mut::<Messages<SteamworksEvent>>()
        .write(SteamworksEvent::MicroTxnAuthorizationResponse(
            steamworks::MicroTxnAuthorizationResponse {
                app_id,
                order_id: 99,
                authorized: true,
            },
        ));

    app.update();

    let mut results = app
        .world_mut()
        .resource_mut::<Messages<SteamworksUserResult>>();
    let drained = results.drain().collect::<Vec<_>>();
    let disconnected = SteamworksSteamServerConnectionEvent::Disconnected {
        reason: steamworks::SteamError::NoConnection,
    };
    let failed = SteamworksSteamServerConnectionEvent::ConnectFailure {
        reason: steamworks::SteamError::Timeout,
        still_retrying: true,
    };
    let micro_txn = SteamworksMicroTxnAuthorizationResponse {
        app_id,
        order_id: 99,
        authorized: true,
    };

    assert_eq!(
        drained,
        vec![
            SteamworksUserResult::Ok(
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: SteamworksSteamServerConnectionEvent::Connected,
                },
            ),
            SteamworksUserResult::Ok(
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: disconnected.clone(),
                },
            ),
            SteamworksUserResult::Ok(
                SteamworksUserOperation::SteamServerConnectionEventReceived {
                    event: failed.clone(),
                },
            ),
            SteamworksUserResult::Ok(
                SteamworksUserOperation::MicroTxnAuthorizationResponseReceived {
                    response: micro_txn.clone(),
                },
            ),
        ]
    );

    let state = app.world().resource::<SteamworksUserState>();
    assert_eq!(state.steam_server_connected(), Some(false));
    assert_eq!(state.last_steam_server_connection_event(), Some(&failed));
    assert_eq!(state.steam_server_connection_events().len(), 3);
    assert_eq!(
        state.steam_server_connection_events(),
        &[
            SteamworksSteamServerConnectionEvent::Connected,
            disconnected,
            failed.clone(),
        ]
    );
    assert_eq!(
        state.last_micro_txn_authorization_response(),
        Some(&micro_txn)
    );
    assert_eq!(
        state.micro_txn_authorization_response(app_id, 99),
        Some(&micro_txn)
    );
    assert_eq!(state.micro_txn_authorized(app_id, 99), Some(true));
    assert_eq!(state.micro_txn_authorized(app_id, 100), None);
    assert_eq!(state.micro_txn_authorization_responses(), &[micro_txn]);
    assert_eq!(state.last_error(), None);
}

#[test]
fn state_updates_cached_logged_on_from_connection_operations() {
    let mut state = SteamworksUserState::default();
    let steam_id = steamworks::SteamId::from_raw(1);

    state.record_operation(&SteamworksUserOperation::CurrentUserInfoRead {
        info: SteamworksUserInfo {
            steam_id,
            level: 7,
            logged_on: false,
        },
    });
    state.record_operation(
        &SteamworksUserOperation::SteamServerConnectionEventReceived {
            event: SteamworksSteamServerConnectionEvent::Connected,
        },
    );

    assert_eq!(state.steam_server_connected(), Some(true));
    assert_eq!(
        state.current_user(),
        Some(&SteamworksUserInfo {
            steam_id,
            level: 7,
            logged_on: true,
        })
    );

    state.record_operation(&SteamworksUserOperation::LoggedOnRead { logged_on: false });

    assert_eq!(state.steam_server_connected(), Some(false));
    assert_eq!(
        state.current_user(),
        Some(&SteamworksUserInfo {
            steam_id,
            level: 7,
            logged_on: false,
        })
    );
}

#[test]
fn state_records_user_operations_without_unbounded_history() {
    let mut state = SteamworksUserState::default();
    let first_user = steamworks::SteamId::from_raw(1);
    let second_user = steamworks::SteamId::from_raw(2);
    let app_id = steamworks::AppId(480);

    state.record_operation(&SteamworksUserOperation::CurrentUserInfoRead {
        info: SteamworksUserInfo {
            steam_id: first_user,
            level: 7,
            logged_on: false,
        },
    });
    state.record_operation(&SteamworksUserOperation::SteamIdRead {
        steam_id: second_user,
    });
    state.record_operation(&SteamworksUserOperation::LevelRead { level: 9 });
    state.record_operation(&SteamworksUserOperation::AuthenticationSessionStarted {
        user: first_user,
    });
    state.record_operation(&SteamworksUserOperation::AuthenticationSessionStarted {
        user: first_user,
    });
    state.record_operation(&SteamworksUserOperation::UserLicenseForAppRead {
        user: first_user,
        app_id,
        license: steamworks::UserHasLicense::HasLicense,
    });
    state.record_operation(&SteamworksUserOperation::AuthenticationSessionEnded {
        user: first_user,
    });

    assert_eq!(
        state.current_user(),
        Some(&SteamworksUserInfo {
            steam_id: second_user,
            level: 9,
            logged_on: false,
        })
    );
    assert_eq!(state.last_steam_id(), Some(second_user));
    assert_eq!(state.last_level(), Some(9));
    assert_eq!(state.steam_id(), Some(second_user));
    assert_eq!(state.level(), Some(9));
    assert_eq!(state.logged_on(), Some(false));
    assert!(state.active_auth_tickets().is_empty());
    assert_eq!(state.active_auth_ticket_count(), 0);
    assert!(state.last_auth_session_ticket().is_none());
    assert!(state.last_auth_session_ticket_for_identity().is_none());
    assert_eq!(state.auth_session_ticket_issue_count(), 0);
    assert!(state.last_web_api_ticket_request().is_none());
    assert_eq!(state.web_api_ticket_request_count(), 0);
    assert!(state.last_cancelled_auth_ticket().is_none());
    assert_eq!(state.auth_ticket_cancel_count(), 0);
    assert!(state.authenticated_users().is_empty());
    assert!(!state.is_user_authenticated(first_user));
    assert_eq!(
        state.last_started_authentication_session(),
        Some(first_user)
    );
    assert_eq!(state.authentication_session_start_count(), 2);
    assert_eq!(state.last_ended_authentication_session(), Some(first_user));
    assert_eq!(state.authentication_session_end_count(), 1);
    assert_eq!(
        state.last_user_license_for_app(),
        Some(&SteamworksUserLicenseForApp {
            user: first_user,
            app_id,
            license: steamworks::UserHasLicense::HasLicense,
        })
    );
    assert_eq!(
        state.user_license_for_app(first_user, app_id),
        state.last_user_license_for_app()
    );
    assert_eq!(
        state.user_license(first_user, app_id),
        Some(&steamworks::UserHasLicense::HasLicense)
    );
    assert_eq!(
        state.user_has_license_for_app(first_user, app_id),
        Some(true)
    );
    assert_eq!(state.user_has_license_for_app(second_user, app_id), None);
    assert_eq!(state.user_licenses_for_apps().len(), 1);
    assert_eq!(state.user_license_check_count(), 1);
}

#[test]
fn user_state_lookup_caches_are_bounded() {
    let mut state = SteamworksUserState::default();
    let user = steamworks::SteamId::from_raw(1);
    let limit = 1_024;

    for index in 0..(limit + 4) {
        state.record_operation(&SteamworksUserOperation::UserLicenseForAppRead {
            user,
            app_id: steamworks::AppId((index + 1) as u32),
            license: steamworks::UserHasLicense::HasLicense,
        });
    }

    assert_eq!(state.user_licenses_for_apps().len(), limit);
    assert!(state
        .user_license_for_app(user, steamworks::AppId(1))
        .is_none());
    assert_eq!(
        state.user_license_for_app(user, steamworks::AppId((limit + 4) as u32)),
        Some(&SteamworksUserLicenseForApp {
            user,
            app_id: steamworks::AppId((limit + 4) as u32),
            license: steamworks::UserHasLicense::HasLicense,
        })
    );
}

#[test]
fn validation_callbacks_do_not_create_sessions_but_failures_remove_known_sessions() {
    let mut state = SteamworksUserState::default();
    let user = steamworks::SteamId::from_raw(1);
    let owner = steamworks::SteamId::from_raw(2);

    state.record_operation(
        &SteamworksUserOperation::AuthenticationTicketValidationReceived {
            validation: SteamworksAuthTicketValidation {
                steam_id: user,
                owner_steam_id: owner,
                response: Ok(()),
            },
        },
    );

    assert!(state.authenticated_users().is_empty());
    assert_eq!(
        state.last_auth_ticket_validation(),
        Some(&SteamworksAuthTicketValidation {
            steam_id: user,
            owner_steam_id: owner,
            response: Ok(()),
        })
    );
    assert_eq!(
        state.auth_ticket_validation(user),
        state.last_auth_ticket_validation()
    );
    assert_eq!(state.auth_ticket_validation_succeeded(user), Some(true));
    assert_eq!(state.auth_ticket_validations().len(), 1);

    state.record_operation(&SteamworksUserOperation::AuthenticationSessionStarted { user });
    assert_eq!(state.authenticated_users(), &[user]);
    assert!(state.is_user_authenticated(user));

    state.record_operation(
        &SteamworksUserOperation::AuthenticationTicketValidationReceived {
            validation: SteamworksAuthTicketValidation {
                steam_id: user,
                owner_steam_id: owner,
                response: Err(SteamworksAuthSessionValidateError::AuthTicketCancelled),
            },
        },
    );

    assert!(state.authenticated_users().is_empty());
    assert_eq!(
        state.last_auth_ticket_validation(),
        Some(&SteamworksAuthTicketValidation {
            steam_id: user,
            owner_steam_id: owner,
            response: Err(SteamworksAuthSessionValidateError::AuthTicketCancelled),
        })
    );
    assert_eq!(
        state.auth_ticket_validation(user),
        state.last_auth_ticket_validation()
    );
    assert_eq!(state.auth_ticket_validation_succeeded(user), Some(false));
    assert_eq!(state.auth_ticket_validations().len(), 1);
}
