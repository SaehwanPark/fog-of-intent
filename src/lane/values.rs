use super::*;

pub const M2_LANE_RULESET: RulesetId = RulesetId::new(2);
pub const M2_OBSERVATION_SCHEMA: &str = "m2-lane-observation-v1";
pub const M2_ALLIED_OBSERVATION_SCHEMA: &str = "m2-allied-proposal-observation-v1";
pub const M2_REPLAY_ID: &str = "m2-one-lane-window-v1";
pub const M2_COORDINATION_REPLAY_ID: &str = "m2-one-lane-coordination-v1";
pub const SCRIPTED_ALLIED_PROFILE: &str = "scripted-allied-proposal-v1";
pub const PLAYER_LANER: ActorId = ActorId::new(1);
pub const OPPONENT_LANER: ActorId = ActorId::new(2);
pub const ALLIED_AUTONOMOUS_ACTOR: ActorId = ActorId::new(3);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LaneTargetFocus {
    #[default]
    Minions,
    OpposingLaner,
    Tower,
}

impl LaneTargetFocus {
    pub const fn default_focus() -> Self {
        Self::Minions
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LaneCommitment {
    #[default]
    Standard,
    Cautious,
    Aggressive,
}

impl LaneCommitment {
    pub const fn default_commitment() -> Self {
        Self::Standard
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LanePingSignal {
    #[default]
    None,
    Danger,
    OnMyWay,
    Assist,
    EnemyMissing,
}

impl LanePingSignal {
    pub const fn default_signal() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LaneAbortCondition {
    #[default]
    None,
    HealthThreshold,
    ThreatSpotted,
    ResourceDepleted,
}

impl LaneAbortCondition {
    pub const fn default_condition() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LaneFallbackBehavior {
    #[default]
    MaintainPlan,
    RetreatToTower,
    SafeFarm,
    ConserveResources,
}

impl LaneFallbackBehavior {
    pub const fn default_behavior() -> Self {
        Self::MaintainPlan
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneHealth(pub(crate) u8);

impl LaneHealth {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_HEALTH {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_HEALTH,
            })
        }
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn subtract(self, amount: LaneDamage) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneMana(pub(crate) u8);

impl LaneMana {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_MANA {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_MANA,
            })
        }
    }

    pub const fn full() -> Self {
        Self(MAX_LANE_MANA)
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = self.0.checked_add(amount.0)?;
        if total <= MAX_LANE_MANA {
            Some(Self(total))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneGold(pub(crate) u8);

impl LaneGold {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_GOLD {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_GOLD,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_GOLD as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneExperience(pub(crate) u8);

impl LaneExperience {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_EXPERIENCE {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_EXPERIENCE,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_EXPERIENCE as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneCooldown(pub(crate) u8);

impl LaneCooldown {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_COOLDOWN {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_COOLDOWN,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub fn tick(self, beats: u32) -> Self {
        Self(self.0.saturating_sub(beats as u8))
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_COOLDOWN as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneBounty(pub(crate) u8);

impl LaneBounty {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_BOUNTY {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_BOUNTY,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_BOUNTY as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneLevel(pub(crate) u8);

impl LaneLevel {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_LEVEL {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_LEVEL,
            })
        }
    }

    pub const fn initial() -> Self {
        Self(1)
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_LEVEL as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneMinionKills(pub(crate) u8);

impl LaneMinionKills {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_MINION_KILLS {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_MINION_KILLS,
            })
        }
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_MINION_KILLS as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneShield(pub(crate) u8);

impl LaneShield {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_SHIELD {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_SHIELD,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_SHIELD as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneWard(pub(crate) u8);

impl LaneWard {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_WARD {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_WARD,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_WARD as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LanePotion(pub(crate) u8);

impl LanePotion {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_POTION {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_POTION,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_POTION as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneElixir(pub(crate) u8);

impl LaneElixir {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_ELIXIR {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_ELIXIR,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_ELIXIR as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneTrinket(pub(crate) u8);

impl LaneTrinket {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_TRINKET {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_TRINKET,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_TRINKET as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneRelic(pub(crate) u8);

impl LaneRelic {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_RELIC {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_RELIC,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_RELIC as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneCharm(pub(crate) u8);

impl LaneCharm {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_CHARM {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_CHARM,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub(crate) fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_CHARM as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneDamage(pub(crate) u8);

impl LaneDamage {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_HEALTH {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_HEALTH,
            })
        }
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LaneBoundsError {
    pub value: u8,
    pub maximum: u8,
}

impl fmt::Display for LaneBoundsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} exceeds maximum {}", self.value, self.maximum)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WavePressure(pub(crate) u8);

impl WavePressure {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_WAVE_PRESSURE {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_WAVE_PRESSURE,
            })
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }
}
