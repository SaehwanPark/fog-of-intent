//! Population-level validation measurement for M9: strategy diversity, role
//! activity, communication usage, and unused-mechanic justification.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! The M9 exit evidence requires multiple team strategies in representative
//! replays and no required mechanic left unused without an explicit reason.
//! This module measures both over an explicit caller-declared validation
//! population: each `ReplayObservation` summarizes one representative replay
//! — the strategy archetype played, the roles that took decisions, the
//! communication activity, and the M9 mechanics the replay exercised. No
//! authoritative match state is read here.
//!
//! All shares are exact truncated integers in basis points of the population
//! size; no floating point, randomness, I/O, or hidden state is involved.
//! Unused mechanics are only acceptable when the caller declares an explicit
//! exemption reason; unexplained unused mechanics fail the population.
//!
//! Malformed inputs fail closed: empty populations, duplicate replay ids, and
//! replays without a single active role are rejected before any measurement.

use core::fmt;

use super::composition::{CompositionArchetype, MatchRole};

pub const M9_POPULATION_VALIDATION_SCHEMA_V1: &str = "m9-population-validation-v1";

/// Minimum distinct strategy archetypes for a passing population.
///
/// Mirrors the M9 exit evidence: "Multiple team strategies appear in
/// representative replays."
pub const MIN_DISTINCT_STRATEGIES: u32 = 2;
/// Minimum share of replays in which each role took decisions (bp).
pub const ROLE_ACTIVITY_FLOOR_BP: u16 = 1_000;
/// Minimum share of replays using team communication (bp).
pub const COMMUNICATION_USAGE_FLOOR_BP: u16 = 2_500;

/// The closed catalog of M9 mechanics a validation population can exercise.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MechanicKind {
  /// Map rotations through the travel model.
  Rotation,
  /// Neutral objective contests and cross-map trades.
  ObjectiveContest,
  /// Ward placement and vision control.
  VisionControl,
  /// Structure sieges and the vulnerability hierarchy.
  StructureSiege,
  /// Comeback evaluation and variance-seeking play.
  ComebackPlay,
  /// Role-specific tactical actions.
  RoleTactics,
  /// Team communication channels and coordination.
  TeamCommunication,
  /// Pivotal-decision review in debriefs.
  PivotalReview,
}

impl MechanicKind {
  /// Every mechanic a complete validation population is expected to touch.
  pub const ALL: [Self; 8] = [
    Self::Rotation,
    Self::ObjectiveContest,
    Self::VisionControl,
    Self::StructureSiege,
    Self::ComebackPlay,
    Self::RoleTactics,
    Self::TeamCommunication,
    Self::PivotalReview,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Rotation => "rotation",
      Self::ObjectiveContest => "objective-contest",
      Self::VisionControl => "vision-control",
      Self::StructureSiege => "structure-siege",
      Self::ComebackPlay => "comeback-play",
      Self::RoleTactics => "role-tactics",
      Self::TeamCommunication => "team-communication",
      Self::PivotalReview => "pivotal-review",
    }
  }
}

impl fmt::Display for MechanicKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// One caller-declared representative replay summary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayObservation {
  /// Stable caller-assigned identity of the replay; must be unique across the
  /// population.
  pub replay_id: &'static str,
  /// Strategy archetype the replayed team played.
  pub strategy: CompositionArchetype,
  /// Roles that took decisions in this replay; at least one is required.
  pub active_roles: &'static [MatchRole],
  /// Count of communication messages exchanged in the replay; zero means the
  /// replay used no team communication.
  pub communication_events: u16,
  /// M9 mechanics this replay exercised.
  pub mechanics_used: &'static [MechanicKind],
}

impl ReplayObservation {
  /// Whether the replay used team communication at all.
  pub const fn uses_communication(&self) -> bool {
    self.communication_events > 0
  }

  /// Whether the given role took decisions in this replay.
  pub fn role_active(&self, role: MatchRole) -> bool {
    self.active_roles.contains(&role)
  }
}

/// A caller-declared explicit reason why a mechanic is unused in a
/// population. Only declared exemptions make an unused mechanic acceptable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MechanicExemption {
  pub mechanic: MechanicKind,
  pub reason: &'static str,
}

/// Typed fail-closed validation error for population measurement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopulationValidationError {
  /// No replays were declared.
  EmptyPopulation,
  /// Two replays share one identity.
  DuplicateReplayId { index: usize },
  /// A replay declares no active role at all.
  ReplayWithoutActiveRoles { index: usize },
}

impl fmt::Display for PopulationValidationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyPopulation => {
        f.write_str("empty population: at least one replay observation is required")
      }
      Self::DuplicateReplayId { index } => write!(
        f,
        "duplicate replay id: observation {index} reuses an earlier replay identity"
      ),
      Self::ReplayWithoutActiveRoles { index } => write!(
        f,
        "replay without active roles: observation {index} declares no role decisions"
      ),
    }
  }
}

/// Deterministic population validation report.
///
/// Produced entirely from explicit observations and exemptions; contains no
/// hidden state, hashes, or execution traces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopulationValidationReport {
  pub schema: &'static str,
  /// Number of validated replay observations.
  pub population_size: u32,
  /// Distinct strategy archetypes observed.
  pub distinct_strategy_count: u32,
  /// Observed share per archetype over `CompositionArchetype::ALL` (bp).
  pub strategy_shares_bp: [(CompositionArchetype, u16); 4],
  /// Whether at least `MIN_DISTINCT_STRATEGIES` archetypes appeared.
  pub strategy_diversity_passes: bool,
  /// Activity share per role over `MatchRole::ALL` (bp).
  pub role_activity_bp: [(MatchRole, u16); 5],
  /// Roles below the activity floor, in `MatchRole::ALL` order.
  pub inactive_roles: Vec<MatchRole>,
  /// Whether every role met the activity floor.
  pub role_activity_passes: bool,
  /// Share of replays using team communication (bp).
  pub communication_usage_bp: u16,
  /// Whether communication usage met its floor.
  pub communication_usage_passes: bool,
  /// Mechanics from `MechanicKind::ALL` no replay exercised.
  pub unused_mechanics: Vec<MechanicKind>,
  /// Unused mechanics without a declared exemption reason.
  pub unexplained_unused_mechanics: Vec<MechanicKind>,
  /// Whether every unused mechanic carries an explicit reason.
  pub all_required_mechanics_justified: bool,
}

impl PopulationValidationReport {
  /// Render a structured Markdown summary of this report.
  ///
  /// Does not include hashes, resolved inputs, or private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# M9 Population Validation Report\n\n");
    out.push_str(&format!(
      "- **Population Size**: {}\n",
      self.population_size
    ));
    out.push_str(&format!(
      "- **Distinct Strategies**: {} (minimum {})\n",
      self.distinct_strategy_count, MIN_DISTINCT_STRATEGIES
    ));
    let shares: Vec<String> = self
      .strategy_shares_bp
      .iter()
      .map(|(archetype, share)| format!("{} {} bp", archetype.as_str(), share))
      .collect();
    out.push_str(&format!("- **Strategy Shares**: {}\n", shares.join(", ")));
    let activity: Vec<String> = self
      .role_activity_bp
      .iter()
      .map(|(role, share)| format!("{} {} bp", role.as_str(), share))
      .collect();
    out.push_str(&format!("- **Role Activity**: {}\n", activity.join(", ")));
    if self.inactive_roles.is_empty() {
      out.push_str("- **Inactive Roles**: none\n");
    } else {
      let roles: Vec<&'static str> = self.inactive_roles.iter().map(|r| r.as_str()).collect();
      out.push_str(&format!(
        "- **Inactive Roles**: {} (floor {} bp)\n",
        roles.join(", "),
        ROLE_ACTIVITY_FLOOR_BP
      ));
    }
    out.push_str(&format!(
      "- **Communication Usage**: {} bp (floor {} bp)\n",
      self.communication_usage_bp, COMMUNICATION_USAGE_FLOOR_BP
    ));
    let unused: Vec<&'static str> = self.unused_mechanics.iter().map(|m| m.as_str()).collect();
    if unused.is_empty() {
      out.push_str("- **Unused Mechanics**: none\n");
    } else {
      out.push_str(&format!("- **Unused Mechanics**: {}\n", unused.join(", ")));
    }
    let unexplained: Vec<&'static str> = self
      .unexplained_unused_mechanics
      .iter()
      .map(|m| m.as_str())
      .collect();
    if unexplained.is_empty() {
      out.push_str("- **Unexplained Unused Mechanics**: none\n");
    } else {
      out.push_str(&format!(
        "- **Unexplained Unused Mechanics**: {}\n",
        unexplained.join(", ")
      ));
    }
    out.push_str(&format!(
      "- **Strategy Diversity Passes**: {}\n",
      yes_no(self.strategy_diversity_passes)
    ));
    out.push_str(&format!(
      "- **Role Activity Passes**: {}\n",
      yes_no(self.role_activity_passes)
    ));
    out.push_str(&format!(
      "- **Communication Usage Passes**: {}\n",
      yes_no(self.communication_usage_passes)
    ));
    out.push_str(&format!(
      "- **All Required Mechanics Justified**: {}\n",
      yes_no(self.all_required_mechanics_justified)
    ));
    out
  }
}

const fn yes_no(flag: bool) -> &'static str {
  if flag { "yes" } else { "no" }
}

/// Truncated basis-point share of `part` over `whole`.
fn share_bp(part: u32, whole: u32) -> u16 {
  u16::try_from(u64::from(part) * 10_000 / u64::from(whole)).expect("share is at most 10,000 bp")
}

/// Measure strategy diversity, role activity, communication usage, and unused
/// mechanics over an explicit caller-declared validation population.
///
/// Pure function — no side effects, no hidden state, no randomness.
/// Validation is fail-closed and precedes measurement: an empty population, a
/// duplicate replay id, or a replay without active roles rejects the whole
/// input.
pub fn measure_validation_population(
  observations: &[ReplayObservation],
  exemptions: &[MechanicExemption],
) -> Result<PopulationValidationReport, PopulationValidationError> {
  if observations.is_empty() {
    return Err(PopulationValidationError::EmptyPopulation);
  }
  for (index, observation) in observations.iter().enumerate() {
    if observations[..index]
      .iter()
      .any(|earlier| earlier.replay_id == observation.replay_id)
    {
      return Err(PopulationValidationError::DuplicateReplayId { index });
    }
    if observation.active_roles.is_empty() {
      return Err(PopulationValidationError::ReplayWithoutActiveRoles { index });
    }
  }

  let population_size = u32::try_from(observations.len()).expect("population fits in a u32");

  let strategy_shares_bp = CompositionArchetype::ALL.map(|archetype| {
    let count = u32::try_from(
      observations
        .iter()
        .filter(|observation| observation.strategy == archetype)
        .count(),
    )
    .expect("archetype count fits in a u32");
    (archetype, share_bp(count, population_size))
  });
  let distinct_strategy_count = u32::try_from(
    strategy_shares_bp
      .iter()
      .filter(|(_, share)| *share > 0)
      .count(),
  )
  .expect("distinct archetype count fits in a u32");

  let role_activity_bp = MatchRole::ALL.map(|role| {
    let count = u32::try_from(
      observations
        .iter()
        .filter(|observation| observation.role_active(role))
        .count(),
    )
    .expect("role count fits in a u32");
    (role, share_bp(count, population_size))
  });
  let inactive_roles: Vec<MatchRole> = role_activity_bp
    .iter()
    .filter(|(_, share)| *share < ROLE_ACTIVITY_FLOOR_BP)
    .map(|(role, _)| *role)
    .collect();

  let communicating = u32::try_from(
    observations
      .iter()
      .filter(|observation| observation.uses_communication())
      .count(),
  )
  .expect("communication count fits in a u32");
  let communication_usage_bp = share_bp(communicating, population_size);

  let unused_mechanics: Vec<MechanicKind> = MechanicKind::ALL
    .iter()
    .copied()
    .filter(|mechanic| {
      !observations
        .iter()
        .any(|observation| observation.mechanics_used.contains(mechanic))
    })
    .collect();
  let unexplained_unused_mechanics: Vec<MechanicKind> = unused_mechanics
    .iter()
    .copied()
    .filter(|mechanic| {
      !exemptions
        .iter()
        .any(|exemption| exemption.mechanic == *mechanic)
    })
    .collect();

  Ok(PopulationValidationReport {
    schema: M9_POPULATION_VALIDATION_SCHEMA_V1,
    distinct_strategy_count,
    strategy_diversity_passes: distinct_strategy_count >= MIN_DISTINCT_STRATEGIES,
    role_activity_passes: inactive_roles.is_empty(),
    communication_usage_passes: communication_usage_bp >= COMMUNICATION_USAGE_FLOOR_BP,
    all_required_mechanics_justified: unexplained_unused_mechanics.is_empty(),
    population_size,
    strategy_shares_bp,
    role_activity_bp,
    inactive_roles,
    communication_usage_bp,
    unused_mechanics,
    unexplained_unused_mechanics,
  })
}
