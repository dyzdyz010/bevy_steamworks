/// Snapshot of common Steam app information for the current process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteamworksCurrentAppInfo {
    /// Current Steam app ID.
    pub app_id: steamworks::AppId,
    /// Current app build ID.
    pub build_id: i32,
    /// Original owner Steam ID for this app.
    pub owner: steamworks::SteamId,
    /// Whether the current user is subscribed to this app.
    pub subscribed: bool,
    /// Whether the current user is subscribed via a free weekend.
    pub subscribed_from_free_weekend: bool,
    /// Whether the current user has a VAC ban.
    pub vac_banned: bool,
    /// Whether the current license is for a cyber cafe.
    pub cybercafe: bool,
    /// Whether the current license is a low-violence depot.
    pub low_violence: bool,
    /// Languages supported by the app.
    pub available_game_languages: Vec<String>,
    /// Current game language.
    pub current_game_language: String,
    /// Current beta branch name, if any.
    pub current_beta_name: Option<String>,
}
