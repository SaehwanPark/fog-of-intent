use super::*;

pub const M2_LANE_RULESET: RulesetId = RulesetId::new(4);
pub const M2_OBSERVATION_SCHEMA: &str = "m2-lane-observation-v3";
pub const M2_ALLIED_OBSERVATION_SCHEMA: &str = "m2-allied-proposal-observation-v3";
pub const M2_REPLAY_ID: &str = "m2-one-lane-window-v3";
pub const M2_COORDINATION_REPLAY_ID: &str = "m2-one-lane-coordination-v3";
pub const M2_BRANCH_REPLAY_ID: &str = "m2-one-lane-window-branch-v3";
pub const SCRIPTED_ALLIED_PROFILE: &str = "scripted-allied-proposal-v3";
pub const PLAYER_LANER: ActorId = ActorId::new(1);
pub const OPPONENT_LANER: ActorId = ActorId::new(2);
pub const ALLIED_AUTONOMOUS_ACTOR: ActorId = ActorId::new(3);
pub const OPPOSING_JUNGLE_THREAT_ACTOR: ActorId = ActorId::new(4);

/// Stable role identities for the bounded M2 lane scenario.
///
/// The roster is scenario metadata, not mutable world state. It identifies
/// who a report refers to without granting an actor access to hidden values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneActorRole {
  HumanLaner,
  OpposingLaner,
  AlliedAutonomous,
  OpposingJungleThreat,
}

impl LaneActorRole {
  pub const fn roster() -> [Self; 4] {
    [
      Self::HumanLaner,
      Self::OpposingLaner,
      Self::AlliedAutonomous,
      Self::OpposingJungleThreat,
    ]
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneActorRoster {
  human_laner: ActorId,
  opposing_laner: ActorId,
  allied_autonomous: ActorId,
  opposing_jungle_threat: ActorId,
}

impl LaneActorRoster {
  pub const fn initial() -> Self {
    Self {
      human_laner: PLAYER_LANER,
      opposing_laner: OPPONENT_LANER,
      allied_autonomous: ALLIED_AUTONOMOUS_ACTOR,
      opposing_jungle_threat: OPPOSING_JUNGLE_THREAT_ACTOR,
    }
  }

  pub const fn actor(self, role: LaneActorRole) -> ActorId {
    match role {
      LaneActorRole::HumanLaner => self.human_laner,
      LaneActorRole::OpposingLaner => self.opposing_laner,
      LaneActorRole::AlliedAutonomous => self.allied_autonomous,
      LaneActorRole::OpposingJungleThreat => self.opposing_jungle_threat,
    }
  }

  pub const fn entries(self) -> [(LaneActorRole, ActorId); 4] {
    [
      (LaneActorRole::HumanLaner, self.human_laner),
      (LaneActorRole::OpposingLaner, self.opposing_laner),
      (LaneActorRole::AlliedAutonomous, self.allied_autonomous),
      (
        LaneActorRole::OpposingJungleThreat,
        self.opposing_jungle_threat,
      ),
    ]
  }
}

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

  pub(crate) fn saturating_add(self, amount: Self) -> Self {
    Self(self.0.saturating_add(amount.0).min(MAX_LANE_HEALTH))
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
    let total = self.0.checked_add(amount.0)?;
    if total <= MAX_LANE_GOLD {
      Some(Self(total))
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
    let total = self.0.checked_add(amount.0)?;
    if total <= MAX_LANE_EXPERIENCE {
      Some(Self(total))
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
    match u8::try_from(beats) {
      Ok(beats) => Self(self.0.saturating_sub(beats)),
      Err(_) => Self::zero(),
    }
  }

  pub(crate) fn add(self, amount: Self) -> Option<Self> {
    let total = self.0.checked_add(amount.0)?;
    if total <= MAX_LANE_COOLDOWN {
      Some(Self(total))
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
