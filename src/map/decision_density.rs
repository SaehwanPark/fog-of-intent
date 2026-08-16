//! Decision-density preservation through automatic routine execution for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! Delegated execution must absorb routine windows (wave clearing, resource
//! collection, transit continuation, ward refresh, regeneration) so routine
//! actions do not force actor decision windows, while the decisions that
//! remain stay dense enough to be meaningful. This module classifies each
//! caller-declared candidate execution window as `AutomaticallyExecuted` or
//! `DecisionRequired`, then evaluates the resulting decision density against
//! explicit targets.
//!
//! Classification is pure and deterministic over explicit inputs: strategic
//! window kinds always require a decision, and routine kinds escalate only
//! when a concrete trigger holds — value stakes at or above the decision
//! threshold, a visible threat, or an active neutral objective. Density
//! targets are explicit: the decision share of all windows must stay inside
//! `[1,000..=5,000]` bp and no gap between consecutive decision windows may
//! exceed 6 turns. All arithmetic is exact integer math; no floating point,
//! randomness, I/O, authoritative state access, or hidden state is involved.
//!
//! Malformed inputs fail closed: empty trajectories, out-of-range stakes, and
//! non-monotonic turns are rejected before any classification.

use core::fmt;

pub const M9_DECISION_DENSITY_SCHEMA_V1: &str = "m9-decision-density-v1";

/// Inclusive bound for declared window value stakes (`[0..=STAKES_BOUND_BP]` bp).
pub const STAKES_BOUND_BP: u32 = 10_000;
/// Stakes at or above this level escalate a routine window into a decision.
///
/// Mirrors the pivotal-decision `Routine` tier ceiling so "material enough to
/// decide" is consistent across both M9 evaluations.
pub const DECISION_STAKES_THRESHOLD_BP: u32 = 500;
/// Minimum decision share of all windows for density to stay meaningful (bp).
pub const DECISION_SHARE_MIN_BP: u16 = 1_000;
/// Maximum decision share of all windows before windows become excessive (bp).
pub const DECISION_SHARE_MAX_BP: u16 = 5_000;
/// Maximum turns between consecutive decision windows before density thins out.
pub const MAX_DECISION_GAP_TURNS: u16 = 6;

/// Category of a candidate execution window.
///
/// Routine kinds are delegatable to automatic execution; strategic kinds
/// always carry a decision the actor must make.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CandidateWindowKind {
  /// Routine: clearing a minion wave without contested stakes.
  WaveClear,
  /// Routine: passive gold/experience collection.
  ResourceCollection,
  /// Routine: continuing an already-committed rotation.
  TransitContinuation,
  /// Routine: refreshing an expired vision ward.
  WardRefresh,
  /// Routine: regeneration out of combat.
  Regeneration,
  /// Strategic: engaging or contesting a neutral objective.
  ObjectiveContest,
  /// Strategic: initiating or re-aiming a rotation.
  RotationChoice,
  /// Strategic: committing to a structure siege.
  SiegeCommit,
  /// Strategic: responding to a visible threat.
  ThreatResponse,
  /// Strategic: a coordination or communication window.
  TeamCoordination,
}

impl CandidateWindowKind {
  /// Whether this kind is delegatable to automatic routine execution when no
  /// escalation trigger holds.
  pub const fn is_routine(self) -> bool {
    matches!(
      self,
      Self::WaveClear
        | Self::ResourceCollection
        | Self::TransitContinuation
        | Self::WardRefresh
        | Self::Regeneration
    )
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::WaveClear => "wave-clear",
      Self::ResourceCollection => "resource-collection",
      Self::TransitContinuation => "transit-continuation",
      Self::WardRefresh => "ward-refresh",
      Self::Regeneration => "regeneration",
      Self::ObjectiveContest => "objective-contest",
      Self::RotationChoice => "rotation-choice",
      Self::SiegeCommit => "siege-commit",
      Self::ThreatResponse => "threat-response",
      Self::TeamCoordination => "team-coordination",
    }
  }
}

impl fmt::Display for CandidateWindowKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Why a candidate window requires an actor decision.
///
/// For routine windows the recorded trigger is the first that holds under the
/// fixed priority order: stakes, then threat, then objective.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EscalationTrigger {
  /// The window's kind is strategic, so a decision is inherent.
  StrategicKind,
  /// Value stakes reached `DECISION_STAKES_THRESHOLD_BP`.
  StakesAtThreshold,
  /// A visible threat is present in the window.
  ThreatPresent,
  /// A neutral objective is active in the window.
  ObjectiveActive,
}

impl EscalationTrigger {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::StrategicKind => "strategic-kind",
      Self::StakesAtThreshold => "stakes-at-threshold",
      Self::ThreatPresent => "threat-present",
      Self::ObjectiveActive => "objective-active",
    }
  }
}

impl fmt::Display for EscalationTrigger {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Whether a candidate window was absorbed by automatic routine execution or
/// must surface an actor decision window.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindowDisposition {
  /// Delegated routine execution resolved the window; no decision forced.
  AutomaticallyExecuted,
  /// The window surfaced a decision the actor must make.
  DecisionRequired,
}

impl WindowDisposition {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::AutomaticallyExecuted => "automatically-executed",
      Self::DecisionRequired => "decision-required",
    }
  }
}

impl fmt::Display for WindowDisposition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// One caller-declared candidate execution window.
///
/// All fields are explicit caller-supplied values; no authoritative match
/// state is read here. `value_stakes_bp` is the caller's estimate of how much
/// match value the window can move, in basis points.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoutineWindowCandidate {
  /// Stable caller-assigned identity of the window. Uniqueness across
  /// candidates is the caller's responsibility; evaluation does not validate it.
  pub window_id: &'static str,
  /// Match turn at which the window occurs; strictly increasing across candidates.
  pub turn: u16,
  /// Category of work the window represents.
  pub kind: CandidateWindowKind,
  /// Declared value stakes of the window (`[0..=10,000]` bp).
  pub value_stakes_bp: u32,
  /// Whether a visible threat is present in this window.
  pub threat_present: bool,
  /// Whether a neutral objective is active in this window.
  pub objective_active: bool,
}

/// Typed fail-closed validation error for decision-density evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecisionDensityError {
  /// No candidate windows were declared.
  EmptyTrajectory,
  /// A candidate's stakes lie outside `[0..=10,000]` bp.
  StakesOutOfRange { index: usize },
  /// A candidate's turn does not strictly increase relative to its predecessor.
  NonMonotonicTurn { index: usize },
}

impl fmt::Display for DecisionDensityError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyTrajectory => {
        f.write_str("empty trajectory: at least one candidate window is required")
      }
      Self::StakesOutOfRange { index } => write!(
        f,
        "stakes out of range: candidate {index} declares value stakes outside [0..={STAKES_BOUND_BP}] bp"
      ),
      Self::NonMonotonicTurn { index } => write!(
        f,
        "non-monotonic turn: candidate {index} does not occur after its predecessor"
      ),
    }
  }
}

/// Derived classification for one candidate window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowFinding {
  pub window_id: &'static str,
  pub turn: u16,
  pub kind: CandidateWindowKind,
  pub disposition: WindowDisposition,
  /// Why a decision was required; `None` for absorbed routine windows.
  pub escalation: Option<EscalationTrigger>,
}

/// Deterministic decision-density evaluation report.
///
/// Produced entirely from explicit candidates; contains no hidden state,
/// hashes, resolved inputs, or execution traces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionDensityReport {
  pub schema: &'static str,
  /// Number of validated candidate windows.
  pub window_count: u32,
  /// Windows absorbed by automatic routine execution.
  pub automatic_count: u32,
  /// Windows that must surface an actor decision.
  pub decision_count: u32,
  /// Exact complement of `decision_share_bp`: share of windows absorbed by
  /// automatic routine execution, in basis points.
  pub routine_absorption_bp: u16,
  /// Decision share of all windows, in basis points.
  pub decision_share_bp: u16,
  /// Turns of decision-required windows, in declared (turn) order.
  pub decision_turns: Vec<u16>,
  /// Largest gap in turns between consecutive decision windows; `None` when
  /// fewer than two decisions occurred.
  pub max_decision_gap_turns: Option<u16>,
  /// Whether `decision_share_bp` lies inside the meaningful-density band.
  pub share_within_band: bool,
  /// Whether every decision gap stayed within `MAX_DECISION_GAP_TURNS`.
  pub gap_within_bound: bool,
  /// Whether density targets held: share band and gap bound together.
  pub meets_density_targets: bool,
  /// Findings in declared (turn) order.
  pub findings: Vec<WindowFinding>,
}

impl DecisionDensityReport {
  /// Render a structured Markdown summary of this report.
  ///
  /// Does not include hashes, resolved inputs, or private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Decision Density Report\n\n");
    out.push_str(&format!("- **Candidate Windows**: {}\n", self.window_count));
    out.push_str(&format!(
      "- **Automatically Executed**: {} ({} bp)\n",
      self.automatic_count, self.routine_absorption_bp
    ));
    out.push_str(&format!(
      "- **Decision Windows**: {} ({} bp)\n",
      self.decision_count, self.decision_share_bp
    ));
    match self.max_decision_gap_turns {
      Some(gap) => out.push_str(&format!(
        "- **Max Decision Gap**: {} turns (bound {})\n",
        gap, MAX_DECISION_GAP_TURNS
      )),
      None => out.push_str("- **Max Decision Gap**: none (fewer than two decisions)\n"),
    }
    out.push_str(&format!(
      "- **Density Targets Met**: {}\n",
      if self.meets_density_targets {
        "yes"
      } else {
        "no"
      }
    ));
    out.push_str("\n## Window Findings\n\n");
    for (position, finding) in self.findings.iter().enumerate() {
      match finding.escalation {
        Some(trigger) => out.push_str(&format!(
          "{}. turn {} — `{}` (`{}`): `{}`, trigger `{}`\n",
          position + 1,
          finding.turn,
          finding.window_id,
          finding.kind,
          finding.disposition,
          trigger,
        )),
        None => out.push_str(&format!(
          "{}. turn {} — `{}` (`{}`): `{}`\n",
          position + 1,
          finding.turn,
          finding.window_id,
          finding.kind,
          finding.disposition,
        )),
      }
    }
    out
  }
}

/// Classify one candidate window.
const fn classify_window(candidate: &RoutineWindowCandidate) -> WindowFinding {
  if !candidate.kind.is_routine() {
    return WindowFinding {
      window_id: candidate.window_id,
      turn: candidate.turn,
      kind: candidate.kind,
      disposition: WindowDisposition::DecisionRequired,
      escalation: Some(EscalationTrigger::StrategicKind),
    };
  }

  // Fixed priority: stakes, then threat, then objective.
  let escalation = if candidate.value_stakes_bp >= DECISION_STAKES_THRESHOLD_BP {
    Some(EscalationTrigger::StakesAtThreshold)
  } else if candidate.threat_present {
    Some(EscalationTrigger::ThreatPresent)
  } else if candidate.objective_active {
    Some(EscalationTrigger::ObjectiveActive)
  } else {
    None
  };

  let disposition = match escalation {
    Some(_) => WindowDisposition::DecisionRequired,
    None => WindowDisposition::AutomaticallyExecuted,
  };

  WindowFinding {
    window_id: candidate.window_id,
    turn: candidate.turn,
    kind: candidate.kind,
    disposition,
    escalation,
  }
}

/// Evaluate decision density over an explicit caller-declared window stream.
///
/// Pure function — no side effects, no hidden state, no randomness.
/// Validation is fail-closed and precedes classification: an empty
/// trajectory, stakes outside `[0..=10,000]` bp, or a turn that does not
/// strictly increase rejects the whole input.
pub fn evaluate_decision_density(
  candidates: &[RoutineWindowCandidate],
) -> Result<DecisionDensityReport, DecisionDensityError> {
  if candidates.is_empty() {
    return Err(DecisionDensityError::EmptyTrajectory);
  }

  let mut previous_turn: Option<u16> = None;
  for (index, candidate) in candidates.iter().enumerate() {
    if candidate.value_stakes_bp > STAKES_BOUND_BP {
      return Err(DecisionDensityError::StakesOutOfRange { index });
    }
    if let Some(previous) = previous_turn
      && candidate.turn <= previous
    {
      return Err(DecisionDensityError::NonMonotonicTurn { index });
    }
    previous_turn = Some(candidate.turn);
  }

  let findings: Vec<WindowFinding> = candidates.iter().map(classify_window).collect();
  let decision_count = findings
    .iter()
    .fold(0u32, |count, finding| match finding.disposition {
      WindowDisposition::DecisionRequired => count + 1,
      WindowDisposition::AutomaticallyExecuted => count,
    });
  let window_count = u32::try_from(findings.len()).expect("candidate window count fits in a u32");
  let automatic_count = window_count - decision_count;
  let decision_turns: Vec<u16> = findings
    .iter()
    .filter(|finding| finding.disposition == WindowDisposition::DecisionRequired)
    .map(|finding| finding.turn)
    .collect();

  // Shares are truncated integers of the exact rational share, so a reported
  // share understates the true share by less than 1 bp; the complement keeps
  // the two reported shares summing to exactly 10,000 bp. The u64 product
  // cannot overflow: window counts are bounded far below `u64::MAX / 10,000`.
  let decision_share_bp =
    u16::try_from(u64::from(decision_count) * 10_000 / u64::from(window_count))
      .expect("decision share is at most 10,000 bp");
  let routine_absorption_bp = 10_000 - decision_share_bp;

  let max_decision_gap_turns = if decision_turns.len() < 2 {
    None
  } else {
    decision_turns
      .windows(2)
      .map(|pair| pair[1] - pair[0])
      .max()
  };

  let share_within_band =
    (DECISION_SHARE_MIN_BP..=DECISION_SHARE_MAX_BP).contains(&decision_share_bp);
  let gap_within_bound = match max_decision_gap_turns {
    Some(gap) => gap <= MAX_DECISION_GAP_TURNS,
    None => true,
  };

  Ok(DecisionDensityReport {
    schema: M9_DECISION_DENSITY_SCHEMA_V1,
    window_count,
    automatic_count,
    decision_count,
    routine_absorption_bp,
    decision_share_bp,
    decision_turns,
    max_decision_gap_turns,
    share_within_band,
    gap_within_bound,
    meets_density_targets: share_within_band && gap_within_bound,
    findings,
  })
}
