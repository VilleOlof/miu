/// The games backend server URL
pub const DOMAIN: &'static str = "www.miubackend.net";
/// The App Id used as the `X-Parse-Application-Id` header in the parse request
pub const APP_ID: &'static str = "ReVector";

/// Parse related constants in the classic version of the game
pub mod classic {
    /// Parse classname for the normal leaderboard
    pub const LEADERBOARD: &'static str = "SPLeaderboard";
    /// Parse classname for weekly leaderboards
    pub const WEEKLY: &'static str = "ChallengeLB";
    /// Parse classname for weekly metadata
    pub const WEEKLY_STATS: &'static str = "ChallengeStats";
}

/// Parse related constants in the ultra version of the game
pub mod ultra {
    /// Parse classname for the normal leaderboard
    pub const LEADERBOARD: &'static str = "SPLeaderboard_Ultra";
    /// Parse classname for weekly leaderboards
    pub const WEEKLY: &'static str = "ChallengeLB_Mayhem";
    /// Parse classname for weekly metadata
    pub const WEEKLY_STATS: &'static str = "ChallengeStats_Mayhem";
}

/// A Parse response, holds error data or the actual results
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Results<T> {
    /// The actual results, always stored in a `Vec`, generic
    ///
    /// `None` if the response contains an error
    pub results: Option<Vec<T>>,
    /// Holds the error code
    ///
    ///`None` if the response was successful
    pub code: Option<u32>,
    /// Holds the error message
    ///
    ///`None` if the response was successful
    pub error: Option<String>,
}

/// Returns the parse url given a classname
pub fn format_url(class: &str) -> String {
    format!("https://{DOMAIN}/parse/classes/{class}")
}
