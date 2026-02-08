use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{EnumMap, serde_as};

use crate::{NameLang, PhysicsMod};

/// A challenge, contains levels, name translation, start and end dates
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Challenge {
    /// The chapter set
    #[serde(rename = "chapterSet")]
    pub chapter_set: String,

    /// The challenge id
    #[serde(rename = "challengeID")]
    pub challenge_id: String,

    /// The levels, contains physics mods and level titles
    pub levels: Vec<ChallengeLevel>,
    /// Name translation, where key is the language and value is the corresponding name
    ///
    /// Use `Challenge::get_name(&self, lang: NameLang)` instead to get the translated name
    pub name: HashMap<String, String>,

    /// The start date of the challenge, in Utc
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,

    /// The end date of the challenge, in Utc
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,
}

/// A level in a challenge
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeLevel {
    /// The fancy level title
    pub name: String,
    /// The map id, *probably contains the "SP_###"*
    pub id: String,
    /// All physics mod for the level
    #[serde_as(as = "EnumMap")]
    pub physicsmod: Vec<PhysicsMod>,
}

impl Challenge {
    /// Returns a translated name of the challenge
    pub fn get_name(&self, lang: NameLang) -> String {
        if let Some(name) = self.name.get(&lang.to_string()) {
            return name.to_owned();
        }

        String::from("Unknown")
    }
}
