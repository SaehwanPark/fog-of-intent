//! Team composition archetypes, match roles, power spike curves, and matchup evaluation for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use super::topology::TeamSide;

/// Canonical player positions/roles in a standard five-player team.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchRole {
  TopLaner,
  Jungler,
  MidLaner,
  BotCarry,
  Support,
}

impl MatchRole {
  pub const ALL: [Self; 5] = [
    Self::TopLaner,
    Self::Jungler,
    Self::MidLaner,
    Self::BotCarry,
    Self::Support,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::TopLaner => "top-laner",
      Self::Jungler => "jungler",
      Self::MidLaner => "mid-laner",
      Self::BotCarry => "bot-carry",
      Self::Support => "support",
    }
  }
}

impl fmt::Display for MatchRole {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// High-level strategic archetype of a team composition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompositionArchetype {
  /// Early-game skirmish power, ganks, and pick potential.
  EarlyPick,
  /// 5v5 teamfight dominance scaling heavily into late game.
  TeamfightScaling,
  /// 1-3-1 or 1-4 cross-map side lane demolition.
  SplitPush,
  /// Long-range poke, zone control, and tower siege.
  PokeSiege,
}

impl CompositionArchetype {
  pub const ALL: [Self; 4] = [
    Self::EarlyPick,
    Self::TeamfightScaling,
    Self::SplitPush,
    Self::PokeSiege,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::EarlyPick => "early-pick",
      Self::TeamfightScaling => "teamfight-scaling",
      Self::SplitPush => "split-push",
      Self::PokeSiege => "poke-siege",
    }
  }
}

impl fmt::Display for CompositionArchetype {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Match phase categorization for power spike calculation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchPhase {
  /// Early game laning and skirmishes (turns 1..=10).
  EarlyGame,
  /// Mid game rotations and objective contests (turns 11..=20).
  MidGame,
  /// Late game base sieges and game-deciding teamfights (turns 21+).
  LateGame,
}

impl MatchPhase {
  pub const fn from_turn(turn: u32) -> Self {
    if turn <= 10 {
      Self::EarlyGame
    } else if turn <= 20 {
      Self::MidGame
    } else {
      Self::LateGame
    }
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::EarlyGame => "early-game",
      Self::MidGame => "mid-game",
      Self::LateGame => "late-game",
    }
  }
}

impl fmt::Display for MatchPhase {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Power scaling curve and tactical ratings for a team composition in basis points ($[0..=10,000]$ bp).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PowerScalingCurve {
  /// Power rating in turns 1..=10 ($[0..=10,000]$ bp).
  pub early_game_bp: u16,
  /// Power rating in turns 11..=20 ($[0..=10,000]$ bp).
  pub mid_game_bp: u16,
  /// Power rating in turns 21+ ($[0..=10,000]$ bp).
  pub late_game_bp: u16,
  /// Tower and structure siege capability rating ($[0..=10,000]$ bp).
  pub siege_rating_bp: u16,
  /// 5v5 teamfight cohesion rating ($[0..=10,000]$ bp).
  pub teamfight_rating_bp: u16,
  /// Wave clear and defense rating ($[0..=10,000]$ bp).
  pub wave_clear_bp: u16,
}

impl PowerScalingCurve {
  pub const fn power_at_phase(&self, phase: MatchPhase) -> u16 {
    match phase {
      MatchPhase::EarlyGame => self.early_game_bp,
      MatchPhase::MidGame => self.mid_game_bp,
      MatchPhase::LateGame => self.late_game_bp,
    }
  }

  pub const fn power_at_turn(&self, turn: u32) -> u16 {
    self.power_at_phase(MatchPhase::from_turn(turn))
  }
}

/// Team composition definition with archetype, power scaling, and role roster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamComposition {
  pub composition_id: &'static str,
  pub archetype: CompositionArchetype,
  pub scaling: PowerScalingCurve,
  pub description: &'static str,
}

impl TeamComposition {
  pub const fn new(
    composition_id: &'static str,
    archetype: CompositionArchetype,
    scaling: PowerScalingCurve,
    description: &'static str,
  ) -> Self {
    Self {
      composition_id,
      archetype,
      scaling,
      description,
    }
  }

  pub const fn power_at_turn(&self, turn: u32) -> u16 {
    self.scaling.power_at_turn(turn)
  }
}

/// Recommended macro posture derived from matchup evaluation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RecommendedPosture {
  /// Force early aggressive trades and skirmishes before opponent outscales.
  ForceEarlyFights,
  /// Play defensively, stall objectives, and scale into late-game strength.
  StallAndScale,
  /// Avoid 5v5 teamfights, create cross-map pressure via side-lane split pushing.
  SplitPushCrossMap,
  /// Establish river vision and poke enemies from safety before committing.
  PokeAndControlVision,
  /// Balanced neutral posture; trade evenly and contest on parity.
  EvenContest,
}

impl RecommendedPosture {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ForceEarlyFights => "force-early-fights",
      Self::StallAndScale => "stall-and-scale",
      Self::SplitPushCrossMap => "split-push-cross-map",
      Self::PokeAndControlVision => "poke-and-control-vision",
      Self::EvenContest => "even-contest",
    }
  }
}

impl fmt::Display for RecommendedPosture {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Deterministic matchup evaluation between allied and opposing team compositions at a specific turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionMatchupEvaluation {
  pub turn: u32,
  pub phase: MatchPhase,
  pub allied_archetype: CompositionArchetype,
  pub opposing_archetype: CompositionArchetype,
  pub allied_power_bp: u16,
  pub opposing_power_bp: u16,
  /// Net power differential from Allied perspective ($[-10,000..=10,000]$ bp).
  pub net_power_delta_bp: i32,
  pub favored_team: Option<TeamSide>,
  pub recommended_allied_posture: RecommendedPosture,
}

impl CompositionMatchupEvaluation {
  /// Evaluate the matchup between allied and opposing compositions deterministically.
  pub fn evaluate(turn: u32, allied: &TeamComposition, opposing: &TeamComposition) -> Self {
    let phase = MatchPhase::from_turn(turn);
    let allied_power_bp = allied.power_at_turn(turn);
    let opposing_power_bp = opposing.power_at_turn(turn);

    let net_power_delta_bp =
      i32::from(allied_power_bp).saturating_sub(i32::from(opposing_power_bp));

    let favored_team = if net_power_delta_bp > 200 {
      Some(TeamSide::Allied)
    } else if net_power_delta_bp < -200 {
      Some(TeamSide::Opposing)
    } else {
      None
    };

    let recommended_allied_posture = match allied.archetype {
      CompositionArchetype::EarlyPick => {
        if phase == MatchPhase::EarlyGame {
          RecommendedPosture::ForceEarlyFights
        } else {
          RecommendedPosture::PokeAndControlVision
        }
      }
      CompositionArchetype::TeamfightScaling => {
        if phase == MatchPhase::EarlyGame {
          RecommendedPosture::StallAndScale
        } else {
          RecommendedPosture::ForceEarlyFights
        }
      }
      CompositionArchetype::SplitPush => RecommendedPosture::SplitPushCrossMap,
      CompositionArchetype::PokeSiege => RecommendedPosture::PokeAndControlVision,
    };

    Self {
      turn,
      phase,
      allied_archetype: allied.archetype,
      opposing_archetype: opposing.archetype,
      allied_power_bp,
      opposing_power_bp,
      net_power_delta_bp,
      favored_team,
      recommended_allied_posture,
    }
  }
}

/// Catalog of canonical baseline team compositions for M9.
pub struct CompositionCatalog;

impl CompositionCatalog {
  pub const EARLY_PICK: TeamComposition = TeamComposition::new(
    "composition-early-pick-v1",
    CompositionArchetype::EarlyPick,
    PowerScalingCurve {
      early_game_bp: 8000,
      mid_game_bp: 6000,
      late_game_bp: 4000,
      siege_rating_bp: 5000,
      teamfight_rating_bp: 5500,
      wave_clear_bp: 6000,
    },
    "High early skirmish power and pick potential that falls off in 5v5 late game.",
  );

  pub const TEAMFIGHT_SCALING: TeamComposition = TeamComposition::new(
    "composition-scaling-teamfight-v1",
    CompositionArchetype::TeamfightScaling,
    PowerScalingCurve {
      early_game_bp: 4000,
      mid_game_bp: 6500,
      late_game_bp: 9000,
      siege_rating_bp: 6000,
      teamfight_rating_bp: 9500,
      wave_clear_bp: 8000,
    },
    "Vulnerable early game that scales into overwhelming 5v5 teamfight dominance.",
  );

  pub const SPLIT_PUSH: TeamComposition = TeamComposition::new(
    "composition-split-push-v1",
    CompositionArchetype::SplitPush,
    PowerScalingCurve {
      early_game_bp: 5500,
      mid_game_bp: 7500,
      late_game_bp: 7000,
      siege_rating_bp: 9000,
      teamfight_rating_bp: 4500,
      wave_clear_bp: 7500,
    },
    "Exceptional 1-3-1 structure demolition that avoids head-on 5v5 clashes.",
  );

  pub const POKE_SIEGE: TeamComposition = TeamComposition::new(
    "composition-poke-siege-v1",
    CompositionArchetype::PokeSiege,
    PowerScalingCurve {
      early_game_bp: 6000,
      mid_game_bp: 7500,
      late_game_bp: 6500,
      siege_rating_bp: 8500,
      teamfight_rating_bp: 6000,
      wave_clear_bp: 9000,
    },
    "Long-range zone control and turret chipping from maximum safety.",
  );

  pub const ALL_COMPOSITIONS: [TeamComposition; 4] = [
    Self::EARLY_PICK,
    Self::TEAMFIGHT_SCALING,
    Self::SPLIT_PUSH,
    Self::POKE_SIEGE,
  ];

  pub fn get_by_archetype(archetype: CompositionArchetype) -> &'static TeamComposition {
    match archetype {
      CompositionArchetype::EarlyPick => &Self::EARLY_PICK,
      CompositionArchetype::TeamfightScaling => &Self::TEAMFIGHT_SCALING,
      CompositionArchetype::SplitPush => &Self::SPLIT_PUSH,
      CompositionArchetype::PokeSiege => &Self::POKE_SIEGE,
    }
  }

  pub fn get_by_id(id: &str) -> Option<&'static TeamComposition> {
    Self::ALL_COMPOSITIONS
      .iter()
      .find(|c| c.composition_id == id)
  }
}
