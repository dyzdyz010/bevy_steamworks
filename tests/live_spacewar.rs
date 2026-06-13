use bevy_app::App;
use bevy_steamworks::{SteamworksClient, SteamworksPlugin};

#[test]
fn live_spacewar_smoke_test() {
    if std::env::var("BEVY_STEAMWORKS_LIVE").as_deref() != Ok("1") {
        eprintln!("skipping live Steamworks smoke test; set BEVY_STEAMWORKS_LIVE=1 to run");
        return;
    }

    let mut app = App::new();
    app.add_plugins(SteamworksPlugin::init_app(480).expect("Steamworks should initialize"));
    app.update();

    let steam = app
        .world()
        .get_resource::<SteamworksClient>()
        .expect("SteamworksClient resource should be inserted");
    let persona = steam.friends().name();

    assert!(
        !persona.trim().is_empty(),
        "Steam persona should not be empty"
    );
    println!("Steam persona: {persona}");
}
