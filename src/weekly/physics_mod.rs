use serde::{Deserialize, Serialize};

/// All physics mods to ever exist.
///
/// Every mod has a value with it.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PhysicsMod {
    /// Changes the gravity
    #[serde(rename = "gravity")]
    Gravity(f32),

    /// Changes the jump height
    #[serde(rename = "jumpmult")]
    JumpMult(f32),

    /// Changes the jump height
    #[serde(rename = "jumpforce")]
    JumpForce(f32),

    /// Changes the bounce height
    #[serde(rename = "bouncemult")]
    BounceMult(f32),

    /// Changes the scale of the marble
    #[serde(rename = "scalemult")]
    ScaleMult(f32),

    /// Changes the mass of the marble
    #[serde(rename = "massmult")]
    MassMult(f32),

    /// Changes the friction of the marble
    #[serde(rename = "frictionmult")]
    FrictionMult(f32),

    /// Changes the blast jump height
    #[serde(rename = "blastjumpmult")]
    BlastJumpMult(f32),

    /// Changes the blast push "scale"
    #[serde(rename = "blastpushmult")]
    BlastPushMult(f32),

    /// Changes the blast range
    #[serde(rename = "blastrangemult")]
    BlastRangeMult(f32),

    /// Changes the cooldown for the blast
    #[serde(rename = "blastcooldownmult")]
    BlastCooldownMult(f32),

    /// Changes the X roll speed
    #[serde(rename = "rollX")]
    RollX(f32),

    /// Changes the Y roll speed
    #[serde(rename = "rollY")]
    RollY(f32),

    /// Changes the X air speed
    #[serde(rename = "airX")]
    AirX(f32),

    /// Changes the Y air speed
    #[serde(rename = "airY")]
    AirY(f32),

    /// If the marble can blast or not
    #[serde(rename = "canblast")]
    CanBlast(bool),

    /// How many airjumps is allowed
    #[serde(rename = "airjumps")]
    AirJumps(i32),

    /// If no powerups should spawn or not
    #[serde(rename = "nopowerups")]
    NoPowerups(bool),

    /// If the levels start and end should be reversed
    #[serde(rename = "reverse")]
    Reverse(bool),

    /// If each checkpoint should also spawn a gem
    #[serde(rename = "checkpointgems")]
    CheckpointGems(bool),

    /// Disables all gems
    #[serde(rename = "nogems")]
    NoGems(bool),

    /// Removes all time travels
    #[serde(rename = "notimetravel")]
    NoTimeTravel(bool),

    /// Replaces the trophy with a gem
    #[serde(rename = "trophygem")]
    TrophyGem(bool),

    /// Replaces the trophy with an end goal
    #[serde(rename = "trophyend")]
    TrophyEnd(bool),

    /// Makes the level a boomerang
    ///
    /// Spawn at the start, go to the end and back to the start
    #[serde(rename = "boomerang")]
    Boomerang(bool),

    /// Start with a specific powerup
    #[serde(rename = "startpowerup")]
    StartPowerup(String),

    /// Replaces all powerups with a specific one
    #[serde(rename = "replacepowerup")]
    ReplacePowerup(String),

    /// Changes the speed for all platforms
    #[serde(rename = "platformspeed")]
    PlatformSpeed(f32),

    /// Changes the X Blast "scale"
    #[serde(rename = "blastX")]
    BlastX(f32),

    /// Changes the Y Blast "scale"
    #[serde(rename = "blastY")]
    BlastY(f32),

    /// Changes the X Impact
    #[serde(rename = "impX")]
    ImpactX(f32),

    /// Changes the Y Impact
    #[serde(rename = "impY")]
    ImpactY(f32),

    /// To use sounds or not
    #[serde(rename = "usesounds")]
    UseSounds(bool),

    /// If the marble should have mega force
    #[serde(rename = "megaforce")]
    MegaForce(f32),

    /// Enable or disables full shadows
    #[serde(rename = "fullshadow")]
    FullShadow(bool),

    /// If the Multiplayer spawn offset is enabled
    #[serde(rename = "mpspawnoffset")]
    MPSpawnOffset(bool),
}

impl ToString for PhysicsMod {
    /// Maps the [`PhysicsMod`] to a `Effect: 0%` or `Description` depending on the data within.  
    fn to_string(&self) -> String {
        fn float_to_perct(f: &f32) -> String {
            format!("{}%", f * 100.0)
        }

        match self {
            PhysicsMod::Gravity(v) => format!("Gravity: {}", float_to_perct(v)),
            PhysicsMod::JumpMult(v) => format!("Jump Height: {}", float_to_perct(v)),
            PhysicsMod::JumpForce(v) => format!("Jump Force: {}", float_to_perct(v)),
            PhysicsMod::BounceMult(v) => format!("Bounce Force: {}", float_to_perct(v)),
            PhysicsMod::ScaleMult(v) => format!("Marble Size: {}", float_to_perct(v)),
            PhysicsMod::MassMult(v) => format!("Mass: {}", float_to_perct(v)),
            PhysicsMod::FrictionMult(v) => format!("Friction Force: {}", float_to_perct(v)),
            PhysicsMod::BlastJumpMult(v) => format!("Blast Height: {}", float_to_perct(v)),
            PhysicsMod::BlastPushMult(v) => format!("Blast Push: {}", float_to_perct(v)),
            PhysicsMod::BlastRangeMult(v) => format!("Blast Range: {}", float_to_perct(v)),
            PhysicsMod::BlastCooldownMult(v) => format!("Blast Cooldown: {}", float_to_perct(v)),
            PhysicsMod::RollX(v) => format!("Roll Force X: {}", float_to_perct(v)),
            PhysicsMod::RollY(v) => format!("Roll Force Y: {}", float_to_perct(v)),
            PhysicsMod::AirX(v) => format!("Air Force X: {}", float_to_perct(v)),
            PhysicsMod::AirY(v) => format!("Air Force Y: {}", float_to_perct(v)),
            PhysicsMod::CanBlast(_) => String::from("Blast Available"),
            PhysicsMod::AirJumps(v) => format!("Air Jumps: {}", v),
            PhysicsMod::NoPowerups(_) => String::from("No Powerups"),
            PhysicsMod::Reverse(_) => String::from("Level Reversed"),
            PhysicsMod::CheckpointGems(_) => String::from("Checkpoints Add Gems"),
            PhysicsMod::NoGems(_) => String::from("No Gems"),
            PhysicsMod::NoTimeTravel(_) => String::from("No Time Travels"),
            PhysicsMod::TrophyGem(_) => String::from("Trophy Adds Gem"),
            PhysicsMod::TrophyEnd(_) => String::from("Trophy is Goal"),
            PhysicsMod::Boomerang(_) => String::from("Boomerang"),
            PhysicsMod::StartPowerup(v) => format!("Start With: {}", v),
            PhysicsMod::ReplacePowerup(v) => format!("Replace Powerups: {}", v),
            PhysicsMod::PlatformSpeed(v) => format!("Platform Speed: {}", float_to_perct(v)),
            PhysicsMod::BlastX(v) => format!("Blast X: {}", float_to_perct(v)),
            PhysicsMod::BlastY(v) => format!("Blast Y: {}", float_to_perct(v)),
            PhysicsMod::ImpactX(v) => format!("Impact X: {}", float_to_perct(v)),
            PhysicsMod::ImpactY(v) => format!("Impact Y: {}", float_to_perct(v)),
            PhysicsMod::UseSounds(_) => format!("Use Sounds"),
            PhysicsMod::MegaForce(v) => format!("Mega Force: {}", float_to_perct(v)),
            PhysicsMod::FullShadow(_) => format!("Full Shadow"),
            PhysicsMod::MPSpawnOffset(_) => format!("MP Spawn Offset"),
        }
    }
}
