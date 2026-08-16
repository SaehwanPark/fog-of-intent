//! Match-level pivotal-decision detection for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! This module identifies which declared match decisions most changed the
//! match's value — the turning points a match-level debrief should surface.
//! Detection is a pure evaluation over an explicit caller-declared value
//! trajectory: each sample carries the Allied-perspective net match value
//! immediately before and after one decision, in integer basis points.
//!
//! Detection classifies swing magnitude (`PivotalTier`), swing direction,
//! strict lead changes, and whether the swing aligned with or against the
//! acting side. All arithmetic is exact integer math except the documented
//! saturating aggregation of `total_absolute_swing_bp`; no floating point,
//! randomness, I/O, authoritative state access, or hidden state is involved.
//!
//! Malformed inputs fail closed: empty trajectories, out-of-range values,
//! and non-monotonic turns are rejected before any classification because
//! tier thresholds depend on exact magnitudes.

use core::fmt;

use super::topology::TeamSide;

pub const M9_PIVOTAL_DECISION_SCHEMA_V1: &str = "m9-pivotal-decision-v1";

/// Inclusive bound for every declared match value (`[-VALUE_BOUND_BP..=VALUE_BOUND_BP]` bp).
pub const VALUE_BOUND_BP: i32 = 10_000;

/// Swing magnitude at or below which a decision is `Routine` (bp).
pub const ROUTINE_MAX_SWING_BP: u32 = 500;
/// Swing magnitude at or below which a decision is `Notable` (bp).
pub const NOTABLE_MAX_SWING_BP: u32 = 1_500;
/// Swing magnitude at or below which a decision is `Pivotal` (bp).
pub const PIVOTAL_MAX_SWING_BP: u32 = 3_500;

/// Discrete pivotality tier derived from absolute swing magnitude.
///
/// Thresholds mirror the comeback deficit-tier granularity so both
/// evaluations reason over comparable basis-point scales.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PivotalTier {
  /// Ordinary play with no material value shift (`|swing| <= 500` bp).
  Routine,
  /// Meaningful swing, but not decisive (`501..=1,500` bp).
  Notable,
  /// A major turning point (`1,501..=3,500` bp).
  Pivotal,
  /// A game-deciding swing (`> 3,500` bp).
  MatchDefining,
}

impl PivotalTier {
  /// Classify an absolute swing magnitude (non-negative) into a tier.
  pub const fn from_swing_magnitude(abs_swing_bp: u32) -> Self {
    if abs_swing_bp <= ROUTINE_MAX_SWING_BP {
      Self::Routine
    } else if abs_swing_bp <= NOTABLE_MAX_SWING_BP {
      Self::Notable
    } else if abs_swing_bp <= PIVOTAL_MAX_SWING_BP {
      Self::Pivotal
    } else {
      Self::MatchDefining
    }
  }

  /// Whether this tier counts as pivotal for report aggregation.
  pub const fn is_pivotal(self) -> bool {
    matches!(self, Self::Pivotal | Self::MatchDefining)
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Routine => "routine",
      Self::Notable => "notable",
      Self::Pivotal => "pivotal",
      Self::MatchDefining => "match-defining",
    }
  }
}

impl fmt::Display for PivotalTier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Which side a decision's value swing favored.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SwingDirection {
  /// The swing increased the Allied-perspective match value.
  AlliedFavorable,
  /// The swing decreased the Allied-perspective match value.
  OpposingFavorable,
  /// The swing changed nothing.
  Neutral,
}

impl SwingDirection {
  /// Classify an Allied-perspective swing delta.
  pub const fn from_swing(swing_bp: i32) -> Self {
    if swing_bp > 0 {
      Self::AlliedFavorable
    } else if swing_bp < 0 {
      Self::OpposingFavorable
    } else {
      Self::Neutral
    }
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::AlliedFavorable => "allied-favorable",
      Self::OpposingFavorable => "opposing-favorable",
      Self::Neutral => "neutral",
    }
  }
}

impl fmt::Display for SwingDirection {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Whether a decision's swing aligned with or against the acting side.
///
/// This separates "pivotal because the acting side made it count" from
/// "pivotal because the acting side threw" without collapsing attribution
/// into the raw outcome sign.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DecisionAlignment {
  /// The acting side's decision moved the match value toward itself.
  SwingWithActor,
  /// The acting side's decision moved the match value toward its opponent.
  SwingAgainstActor,
  /// The decision produced no value movement.
  NeutralSwing,
}

impl DecisionAlignment {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SwingWithActor => "swing-with-actor",
      Self::SwingAgainstActor => "swing-against-actor",
      Self::NeutralSwing => "neutral-swing",
    }
  }
}

impl fmt::Display for DecisionAlignment {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// One caller-declared decision measurement on the match value trajectory.
///
/// All fields are explicit caller-supplied values; no authoritative match
/// state is read here. Values are Allied-perspective net match value in
/// basis points, each within `[-10,000..=10,000]`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PivotalDecisionSample {
  /// Stable caller-assigned identity of the decision. Uniqueness across
  /// samples is the caller's responsibility; detection does not validate it.
  pub decision_id: &'static str,
  /// Match turn at which the decision occurred; strictly increasing across samples.
  pub turn: u16,
  /// Side whose decision produced this measurement.
  pub acting_side: TeamSide,
  /// Allied-perspective net match value immediately before the decision.
  pub value_before_bp: i32,
  /// Allied-perspective net match value immediately after the decision.
  pub value_after_bp: i32,
}

/// Typed fail-closed validation error for pivotal-decision detection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PivotalDecisionError {
  /// No samples were declared.
  EmptyTrajectory,
  /// A sample's value lies outside `[-10,000..=10,000]` bp.
  ValueOutOfRange { index: usize },
  /// A sample's turn does not strictly increase relative to its predecessor.
  NonMonotonicTurn { index: usize },
}

impl fmt::Display for PivotalDecisionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyTrajectory => {
        f.write_str("empty trajectory: at least one decision sample is required")
      }
      Self::ValueOutOfRange { index } => write!(
        f,
        "value out of range: sample {index} declares a match value outside [{VALUE_BOUND_BP}..={VALUE_BOUND_BP}] bp"
      ),
      Self::NonMonotonicTurn { index } => write!(
        f,
        "non-monotonic turn: sample {index} does not occur after its predecessor"
      ),
    }
  }
}

/// Derived classification for one declared decision sample.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PivotalDecisionFinding {
  pub decision_id: &'static str,
  pub turn: u16,
  pub acting_side: TeamSide,
  /// Allied-perspective value delta produced by the decision.
  pub swing_bp: i32,
  pub direction: SwingDirection,
  pub tier: PivotalTier,
  /// Whether the decision strictly flipped which side held the value lead.
  pub lead_changed: bool,
  pub alignment: DecisionAlignment,
}

/// Deterministic pivotal-decision detection report.
///
/// Produced entirely from explicit samples; contains no hidden state,
/// hashes, resolved inputs, or execution traces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PivotalDecisionReport {
  pub schema: &'static str,
  /// Number of validated decision samples.
  pub sample_count: usize,
  /// Findings in declared (turn) order.
  pub findings: Vec<PivotalDecisionFinding>,
  /// The finding with the largest absolute swing; earliest turn breaks ties.
  pub most_pivotal: PivotalDecisionFinding,
  /// Count of `Pivotal` plus `MatchDefining` findings.
  pub pivotal_count: u32,
  /// Turns whose decision strictly flipped the value lead, in turn order.
  pub lead_change_turns: Vec<u16>,
  /// Allied-perspective match value after the final declared decision.
  pub final_value_bp: i32,
  /// Saturating sum of absolute swings across all findings.
  ///
  /// Saturation is unreachable through validation: strictly increasing
  /// `u16` turns cap the trajectory below 65,537 samples and each validated
  /// swing is at most 20,000 bp, so the true sum stays below `u32::MAX`.
  pub total_absolute_swing_bp: u32,
}

impl PivotalDecisionReport {
  /// Findings of `Pivotal` or `MatchDefining` tier, ranked by descending
  /// absolute swing with earliest-turn tie-break. Empty when no declared
  /// decision was pivotal.
  pub fn pivotal_findings(&self) -> Vec<&PivotalDecisionFinding> {
    let mut ranked: Vec<&PivotalDecisionFinding> = self
      .findings
      .iter()
      .filter(|finding| finding.tier.is_pivotal())
      .collect();
    ranked.sort_by(|a, b| {
      b.swing_bp
        .unsigned_abs()
        .cmp(&a.swing_bp.unsigned_abs())
        .then(a.turn.cmp(&b.turn))
    });
    ranked
  }

  /// Render a structured Markdown debrief summary of this report.
  ///
  /// Does not include hashes, resolved inputs, or private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Pivotal Decision Report\n\n");
    out.push_str(&format!("- **Sample Count**: {}\n", self.sample_count));
    out.push_str(&format!(
      "- **Pivotal Decisions**: {} (pivotal + match-defining)\n",
      self.pivotal_count
    ));
    out.push_str(&format!(
      "- **Most Pivotal Decision**: `{}` at turn {} (`{}`)\n",
      self.most_pivotal.decision_id, self.most_pivotal.turn, self.most_pivotal.tier
    ));
    if self.lead_change_turns.is_empty() {
      out.push_str("- **Lead Change Turns**: none\n");
    } else {
      let turns: Vec<String> = self
        .lead_change_turns
        .iter()
        .map(|turn| turn.to_string())
        .collect();
      out.push_str(&format!("- **Lead Change Turns**: {}\n", turns.join(", ")));
    }
    out.push_str(&format!(
      "- **Final Match Value**: {} bp\n",
      self.final_value_bp
    ));
    out.push_str(&format!(
      "- **Total Absolute Swing**: {} bp\n",
      self.total_absolute_swing_bp
    ));
    out.push_str("\n## Decision Findings\n\n");
    for (position, finding) in self.findings.iter().enumerate() {
      out.push_str(&format!(
        "{}. turn {} — `{}` by {:?}: {} bp swing (`{}`), {}, alignment `{}`, lead change: {}\n",
        position + 1,
        finding.turn,
        finding.decision_id,
        finding.acting_side,
        finding.swing_bp,
        finding.tier,
        finding.direction,
        finding.alignment,
        if finding.lead_changed { "yes" } else { "no" },
      ));
    }
    out
  }
}

/// Whether an Allied-perspective value transition strictly flips the lead.
///
/// Passing to or from exact parity is not a lead change.
const fn lead_flipped(before_bp: i32, after_bp: i32) -> bool {
  (before_bp > 0 && after_bp < 0) || (before_bp < 0 && after_bp > 0)
}

/// Detect pivotal decisions on an explicit caller-declared value trajectory.
///
/// Pure function — no side effects, no hidden state, no randomness.
/// Validation is fail-closed and precedes classification: an empty
/// trajectory, a value outside `[-10,000..=10,000]` bp, or a turn that does
/// not strictly increase rejects the whole input.
pub fn detect_pivotal_decisions(
  samples: &[PivotalDecisionSample],
) -> Result<PivotalDecisionReport, PivotalDecisionError> {
  if samples.is_empty() {
    return Err(PivotalDecisionError::EmptyTrajectory);
  }

  let mut previous_turn: Option<u16> = None;
  for (index, sample) in samples.iter().enumerate() {
    // `unsigned_abs` is total on i32 (`.abs()` would panic on i32::MIN in
    // checked builds and wrap in release), keeping this check fail-closed
    // for every representable input.
    if sample.value_before_bp.unsigned_abs() > VALUE_BOUND_BP.unsigned_abs()
      || sample.value_after_bp.unsigned_abs() > VALUE_BOUND_BP.unsigned_abs()
    {
      return Err(PivotalDecisionError::ValueOutOfRange { index });
    }
    if let Some(previous) = previous_turn
      && sample.turn <= previous
    {
      return Err(PivotalDecisionError::NonMonotonicTurn { index });
    }
    previous_turn = Some(sample.turn);
  }

  let mut findings = Vec::with_capacity(samples.len());
  let mut lead_change_turns = Vec::new();
  let mut pivotal_count: u32 = 0;
  let mut total_absolute_swing_bp: u32 = 0;
  let mut most_pivotal: Option<PivotalDecisionFinding> = None;

  for sample in samples {
    let swing_bp = sample.value_after_bp.saturating_sub(sample.value_before_bp);
    let tier = PivotalTier::from_swing_magnitude(swing_bp.unsigned_abs());
    let lead_changed = lead_flipped(sample.value_before_bp, sample.value_after_bp);
    let alignment = if swing_bp == 0 {
      DecisionAlignment::NeutralSwing
    } else {
      let actor_gained = match sample.acting_side {
        TeamSide::Allied => swing_bp > 0,
        TeamSide::Opposing => swing_bp < 0,
      };
      if actor_gained {
        DecisionAlignment::SwingWithActor
      } else {
        DecisionAlignment::SwingAgainstActor
      }
    };

    let finding = PivotalDecisionFinding {
      decision_id: sample.decision_id,
      turn: sample.turn,
      acting_side: sample.acting_side,
      swing_bp,
      direction: SwingDirection::from_swing(swing_bp),
      tier,
      lead_changed,
      alignment,
    };

    if finding.tier.is_pivotal() {
      pivotal_count = pivotal_count.saturating_add(1);
    }
    if finding.lead_changed {
      lead_change_turns.push(finding.turn);
    }
    total_absolute_swing_bp = total_absolute_swing_bp.saturating_add(swing_bp.unsigned_abs());
    // Findings arrive in turn order, so a strict magnitude comparison alone
    // keeps the earliest turn on equal-swing ties.
    if most_pivotal
      .as_ref()
      .is_none_or(|current| finding.swing_bp.unsigned_abs() > current.swing_bp.unsigned_abs())
    {
      most_pivotal = Some(finding.clone());
    }

    findings.push(finding);
  }

  let most_pivotal = most_pivotal.expect("validated non-empty trajectory yields a finding");
  let final_value_bp = samples[samples.len() - 1].value_after_bp;

  Ok(PivotalDecisionReport {
    schema: M9_PIVOTAL_DECISION_SCHEMA_V1,
    sample_count: samples.len(),
    findings,
    most_pivotal,
    pivotal_count,
    lead_change_turns,
    final_value_bp,
    total_absolute_swing_bp,
  })
}
