pub(crate) mod challenge;
pub(crate) mod name_lang;
pub(crate) mod physics_mod;
pub(crate) mod scorebucket;

pub use challenge::{Challenge, ChallengeLevel};
pub use name_lang::NameLang;
pub use physics_mod::PhysicsMod;
pub use scorebucket::ScoreBucket;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{MIUError, parse::Results};

/// An entire weekly challenge
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weekly {
    /// The internal parse objectid
    #[serde(rename = "objectId")]
    pub object_id: String,

    /// The level id, this ones a bit off but its often `CHALLENGE_DATA` for the current week
    #[serde(rename = "LevelID")]
    pub level_id: String,

    /// Created at, in UTC
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// Updated at, in UTC
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    /// The scorebucket, contains levels, names, start/end dates and physics mods
    #[serde(rename = "ScoreBuckets")]
    pub score_buckets: ScoreBucket,
}

/// The week state
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeekState {
    /// For the current weekly challenge
    Current,
    /// For the previous weekly challenge
    Previous,
}

/// This is because ScoreBuckets is a json string, inside a json response
/// Only for internal deserialization
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MidWeekly {
    #[serde(rename = "objectId")]
    object_id: String,
    #[serde(rename = "LevelID")]
    level_id: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    updated_at: DateTime<Utc>,
    #[serde(rename = "ScoreBuckets")]
    score_buckets: String,
}

impl Weekly {
    /// Since some internal fields are json within already parsed json, this handles two json's.  
    pub fn from_json(json: &str) -> Result<Weekly, MIUError> {
        let mid = match serde_json::from_str::<Results<MidWeekly>>(json) {
            Ok(mid) => match mid.results {
                Some(results) => {
                    if results.is_empty() {
                        return Err(MIUError::EmptyResults);
                    }

                    results.first().unwrap().clone()
                }
                None => return Err(MIUError::EmptyResults),
            },
            Err(err) => return Err(MIUError::FailedToParseWeekly(err)),
        };

        let score_buckets = match serde_json::from_str::<ScoreBucket>(&mid.score_buckets) {
            Ok(s_bucket) => s_bucket,
            Err(err) => return Err(MIUError::FailedToParseScorebucket(err)),
        };

        Ok(Weekly {
            object_id: mid.object_id,
            level_id: mid.level_id,
            created_at: mid.created_at,
            updated_at: mid.updated_at,
            score_buckets,
        })
    }
}
