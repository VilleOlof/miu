use serde::{Deserialize, Serialize};

use crate::Challenge;

/// Holds the current and previous challenge
///
/// And some internal sheet ids and stuff
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreBucket {
    /// The current challenge
    pub current: Challenge,
    /// The previous challenge
    pub previous: Challenge,

    /// The sheet id of the challenge
    #[serde(rename = "sheetID")]
    pub sheet_id: i32,

    /// the cur id?
    #[serde(rename = "curID")]
    pub cur_id: i32,

    /// i honestly dont know what this is help, look at the raw response or something idk
    pub level: String,
}
