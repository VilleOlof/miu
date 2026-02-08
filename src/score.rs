use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Score struct
///
/// Common across normal leaderboards and weekly challenges leaderboards
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Score {
    /// The time of the score
    pub time: f32,

    /// The user id, is differently formatted depending on platform
    #[serde(rename = "userID")]
    pub user_id: String,

    /// The username of whoevers score
    pub username: String,

    /// The raw mapid, includes `SP_###`
    #[serde(rename = "mapID")]
    pub map_id: String,

    /// The id of the skin used
    #[serde(rename = "skinUsed")]
    pub skin_used: String,

    /// The replay version
    #[serde(rename = "replayVersion")]
    pub replay_version: u32,

    /// The platform the score was performed on
    pub platform: String,
    /// The replay struct, contains further replay information
    pub replay: Option<Replay>,

    /// Created at, in Utc
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// Updated at, in Utc
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    /// Parse internal object id
    #[serde(rename = "objectId")]
    pub object_id: Option<String>,
}

// A Replay struct
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    /// The replay type, often a file
    #[serde(rename = "__type")]
    pub r#type: String,
    /// The name of the replay
    pub name: String,
    /// The url for the replay
    ///
    /// Kinda odd one since its a local host, use the name and replay module instead of this
    pub url: String,
}

impl Score {
    /// Returns a formatted time
    ///
    /// In the format of: `MM:SS:MS` only if the time is above a minute,
    ///
    /// otherwise it just returns the time as string
    pub fn format_time(&self) -> String {
        if self.time < 60.0 {
            return self.time.to_string();
        }

        let dur = Duration::from_secs_f64(self.time as f64);
        let minutes = (dur.as_secs() / 60) % 60;
        let seconds = dur.as_secs() % 60;
        let millis = {
            let num = self.time.floor();
            let dec = self.time - num;

            format!("{:.6}", dec)[2..].to_string()
        };

        format!("{:0>2}:{:0>2}.{}", minutes, seconds, millis)
    }
}

#[cfg(test)]
mod test {
    use crate::test_util::gen_score;

    #[test]
    fn test_format_time() {
        let mut score_1 = gen_score(0.0..1.0);
        score_1.time = 1.0;
        let mut score_2 = gen_score(0.0..1.0);
        score_2.time = 5.242422;
        let mut score_3 = gen_score(0.0..1.0);
        score_3.time = 125.242966;
        let mut score_4 = gen_score(0.0..1.0);
        score_4.time = 2421.592041;

        assert_eq!("1", score_1.format_time());
        assert_eq!("5.242422", score_2.format_time());
        assert_eq!("02:05.242966", score_3.format_time());
        assert_eq!("40:21.592041", score_4.format_time());
    }
}
